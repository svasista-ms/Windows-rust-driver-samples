// Copyright (c) Microsoft Corporation.
// License: MIT OR Apache-2.0

use wdk::{nt_success, paged_code, println};
use wdk_sys::{
    macros,
    ntddk::{ExAllocatePool2, ExFreePool, KeEnterCriticalRegion, KeGetCurrentIrql},
    APC_LEVEL,
    DRIVER_OBJECT,
    NTSTATUS,
    PCUNICODE_STRING,
    PDRIVER_OBJECT,
    POOL_FLAG_NON_PAGED,
    SIZE_T,
    ULONG,
    WDFDEVICE,
    WDFDEVICE_INIT,
    WDFDRIVER,
    WDF_DRIVER_CONFIG,
    WDF_NO_HANDLE,
    WDF_NO_OBJECT_ATTRIBUTES,
    WDF_OBJECT_ATTRIBUTES,
    WDF_PNPPOWER_EVENT_CALLBACKS,
    _WDF_EXECUTION_LEVEL,
    _WDF_SYNCHRONIZATION_SCOPE,
};

use crate::{initialize_spinlock, GLOBAL_BUFFER, GUID_DEVINTERFACE, SPINLOCK};

/// `DriverEntry` initializes the driver and is the first routine called by the
/// system after the driver is loaded. `DriverEntry` specifies the other entry
/// points in the function driver, such as `EvtDevice` and `DriverUnload`.
///
/// # Arguments
///
/// * `driver` - represents the instance of the function driver that is loaded
///   into memory. `DriverEntry` must initialize members of `DriverObject`
///   before it returns to the caller. `DriverObject` is allocated by the system
///   before the driver is loaded, and it is released by the system after the
///   system unloads the function driver from memory.
/// * `registry_path` - represents the driver specific path in the Registry. The
///   function driver can use the path to store driver related data between
///   reboots. The path does not store hardware instance specific data.
///
/// # Return value:
///
/// * `STATUS_SUCCESS` - if successful,
/// * `STATUS_UNSUCCESSFUL` - otherwise.
#[link_section = "INIT"]
#[export_name = "DriverEntry"]
extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("Enter: driver_entry");

    let mut driver_config = {
        let wdf_driver_config_size: ULONG;

        // clippy::cast_possible_truncation cannot currently check compile-time constants: https://github.com/rust-lang/rust-clippy/issues/9613
        #[allow(clippy::cast_possible_truncation)]
        {
            const WDF_DRIVER_CONFIG_SIZE: usize = core::mem::size_of::<WDF_DRIVER_CONFIG>();

            // Manually assert there is not truncation since clippy doesn't work for
            // compile-time constants
            const { assert!(WDF_DRIVER_CONFIG_SIZE <= ULONG::MAX as usize) }

            wdf_driver_config_size = WDF_DRIVER_CONFIG_SIZE as ULONG;
        }

        WDF_DRIVER_CONFIG {
            Size: wdf_driver_config_size,
            EvtDriverDeviceAdd: Some(evt_driver_device_add),
            EvtDriverUnload: Some(evt_driver_unload),
            ..WDF_DRIVER_CONFIG::default()
        }
    };

    let driver_handle_output = WDF_NO_HANDLE.cast::<WDFDRIVER>();

    let nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDriverCreate,
            driver as PDRIVER_OBJECT,
            registry_path,
            WDF_NO_OBJECT_ATTRIBUTES,
            &mut driver_config,
            driver_handle_output,
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDriverCreate failed {nt_status:#010X}");
        return nt_status;
    }

    // Allocate non-paged memory pool of 1 byte (arbitrarily chosen) for the
    // Global buffer
    unsafe {
        const LENGTH: usize = 1;
        GLOBAL_BUFFER = ExAllocatePool2(POOL_FLAG_NON_PAGED, LENGTH as SIZE_T, 's' as u32);
    }

    // Initialize a spinlock that can be used to synchronize access to the buffer
    if let Err(status) = initialize_spinlock() {
        println!("Failed to initialize spinlock: {status:#010X}");
        return status;
    }

    println!("Exit: driver_entry");

    nt_status
}

/// `EvtDeviceAdd` is called by the framework in response to `AddDevice`
/// call from the `PnP` manager. We create and initialize a device object to
/// represent a new instance of the device.
///
/// # Arguments:
///
/// * `_driver` - Handle to a framework driver object created in `DriverEntry`
/// * `device_init` - Pointer to a framework-allocated `WDFDEVICE_INIT`
///   structure.
///
/// # Return value:
///
///   * `NTSTATUS`
#[link_section = "PAGE"]
extern "C" fn evt_driver_device_add(
    _driver: WDFDRIVER,
    mut device_init: *mut WDFDEVICE_INIT,
) -> NTSTATUS {
    paged_code!();

    println!("Enter: evt_driver_device_add");

    let mut pnp_power_callbacks = WDF_PNPPOWER_EVENT_CALLBACKS {
        Size: core::mem::size_of::<WDF_PNPPOWER_EVENT_CALLBACKS>() as ULONG,
        EvtDeviceD0Entry: Some(evt_device_d0_entry),
        EvtDeviceD0Exit: Some(evt_device_d0_exit),
        ..WDF_PNPPOWER_EVENT_CALLBACKS::default()
    };

    let [()] = [unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceInitSetPnpPowerEventCallbacks,
            device_init,
            &mut pnp_power_callbacks
        );
    }];

    #[allow(clippy::cast_possible_truncation)]
    let mut attributes = WDF_OBJECT_ATTRIBUTES {
        Size: core::mem::size_of::<WDF_OBJECT_ATTRIBUTES>() as ULONG,
        ExecutionLevel: _WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent,
        SynchronizationScope: _WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent,
        ..WDF_OBJECT_ATTRIBUTES::default()
    };

    let mut device = WDF_NO_HANDLE as WDFDEVICE;
    let mut nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceCreate,
            &mut device_init,
            &mut attributes,
            &mut device,
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDeviceCreate failed {nt_status:#010X}");
        return nt_status;
    }

    nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceCreateDeviceInterface,
            device,
            &GUID_DEVINTERFACE,
            core::ptr::null_mut(),
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDeviceCreateDeviceInterface failed {nt_status:#010X}");
        return nt_status;
    }

    println!("Exit: evt_driver_device_add");

    nt_status
}

/// This event callback function is called before the driver is unloaded
///
/// The `EvtDriverUnload` callback function must deallocate any
/// non-device-specific system resources that the driver's DriverEntry routine
/// allocated.
///
/// # Argument:
///
/// * `driver` - Handle to the framework driver object
///
/// # Return Value:
///
/// None
extern "C" fn evt_driver_unload(_driver: WDFDRIVER) {
    println!("Enter: evt_driver_unload");

    unsafe { ExFreePool(GLOBAL_BUFFER) };

    println!("Exit: evt_driver_unload");
}

/// `EvtDeviceD0Entry` event callback function performs operations
/// that are needed when the driver's device enters the D0 power state.
///
/// # Arguments:
///
/// * `Device` - A handle to a framework device object.
/// * `PreviousState` - A WDF_POWER_DEVICE_STATE-typed enumerator that
///   identifies the previous device power state.
///
/// # Return Value:
///
/// * STATUS_SUCCESS or another status value for which NT_SUCCESS(status) equals
///   TRUE. For failures, return a status value for which NT_SUCCESS(status)
///   equals FALSE
extern "C" fn evt_device_d0_entry(_device: WDFDEVICE, _prev_state: i32) -> i32 {
    println!("Enter: evt_device_d0_entry");
    unsafe {
        if let Some(ref spinlock) = SPINLOCK {
            spinlock.acquire();
            if !GLOBAL_BUFFER.is_null() {
                core::ptr::write_bytes(GLOBAL_BUFFER, 1, 1);
            } else {
                println!("Global buffer is null");
            }
            spinlock.release();
        } else {
            println!("Spinlock is not initialized");
            return -1;
        }
    }
    println!("Exit: evt_device_d0_entry");
    0
}

/// `EvtDeviceD0Exit` event callback function performs operations
/// that are needed when the driver's device leaves the D0 power state.
///
/// NOTE: The IRQL violation fault is injected in this callback
///
/// # Arguments:
///
/// * `Device` - A handle to a framework device object.
/// * `TargetState` - A WDF_POWER_DEVICE_STATE-typed enumerator that identifies
///   the device power state that the device is about to enter.
///
/// # Return Value:
///
/// * STATUS_SUCCESS or another status value for which NT_SUCCESS(status) equals
///   TRUE. For failures, return a status value for which NT_SUCCESS(status)
///   equals FALSE
extern "C" fn evt_device_d0_exit(_device: WDFDEVICE, _prev_state: i32) -> i32 {
    println!("Enter: evt_device_d0_exit");
    unsafe {
        if let Some(ref spinlock) = SPINLOCK {
            spinlock.acquire();
            if !GLOBAL_BUFFER.is_null() {
                core::ptr::write_bytes(GLOBAL_BUFFER, 0, 1);
                // Illegal call to KeEnterCriticalRegion will lead to a
                // violation of 'IrqlKeApcLte2' rule
                KeEnterCriticalRegion();
            } else {
                println!("Global buffer is null");
            }
            spinlock.release();
        } else {
            println!("Spinlock is not initialized");
            return -1;
        }
    }
    println!("Exit: evt_device_d0_exit");
    0
}