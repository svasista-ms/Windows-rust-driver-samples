use wdk_sys::{
    macros,
    ULONG,
    NTSTATUS,
    GUID,
    WDFDRIVER,
    ntddk::KeGetCurrentIrql,
    // UNICODE_STRING,
    // WCHAR,
    PWDFDEVICE_INIT,
    APC_LEVEL,
    WDF_PNPPOWER_EVENT_CALLBACKS,
    WDFDEVICE,
    WDF_OBJECT_ATTRIBUTES,
    _WDF_EXECUTION_LEVEL,
    _WDF_SYNCHRONIZATION_SCOPE,
    WDF_NO_HANDLE,
    WDFDEVICE_INIT
};

use wdk::{nt_success, println, paged_code};

use crate::pnp_power_callbacks::*;

// const DEVICE_NAME_BUFFER: [WCHAR; 17] = [
//     '\\' as WCHAR, 'D' as WCHAR, 'e' as WCHAR, 'v' as WCHAR, 'i' as WCHAR, 'c' as WCHAR, 'e' as WCHAR,
//     '\\' as WCHAR, 'V' as WCHAR, 'a' as WCHAR, 'l' as WCHAR, 'i' as WCHAR, 'd' as WCHAR, 'a' as WCHAR, 't' as WCHAR, 'e' as WCHAR, 
//     0,
// ];

// // Define a static UNICODE_STRING for "\\Device\\Validate".
// // Length is the length of the string in bytes, not including the null terminator.
// // MaximumLength includes the null terminator.
// const DEVICE_NAME: UNICODE_STRING = UNICODE_STRING {
//     Length: (16 * 2) as u16, // Length in bytes of the string content, excluding null terminator
//     MaximumLength: (17 * 2) as u16, // Maximum length in bytes, including null terminator
//     Buffer: DEVICE_NAME_BUFFER.as_ptr() as *mut WCHAR,
// };

const GUID_DEVINTERFACE: GUID = GUID {
    Data1: 0x4D36_E97Du32,
    Data2: 0xE325u16,
    Data3: 0x11CEu16,
    Data4: [
        0xBFu8, 0xC1u8, 0x08u8, 0x00u8, 0x2Bu8, 0xE1u8, 0x03u8, 0x18u8,
    ],
};

// EvtDeviceAdd - Called by the framework in response to AddDevice call from the PnP manager.

pub extern "C" fn evt_device_add(_driver: WDFDRIVER, device_init: PWDFDEVICE_INIT) -> NTSTATUS {
    paged_code!();

    println!("Enter: EvtDeviceAdd");

    let mut device_init = unsafe {
        device_init.as_mut().expect("device_init is null. WDF should never provide a null pointer for device_init")
    };

    // WdfDeviceInitAssignName method assigns a device name to a device's device object.
    
    // let _nt_status = unsafe {
    //     macros::call_unsafe_wdf_function_binding!(
    //         WdfDeviceInitAssignName,
    //         device_init,
    //         &DEVICE_NAME
    //     )
    // };

    // Setup pnp/power callbacks

    let mut pnp_power_event_callbacks = WDF_PNPPOWER_EVENT_CALLBACKS {
        Size: core::mem::size_of::<WDF_PNPPOWER_EVENT_CALLBACKS>() as ULONG,
        EvtDeviceD0Entry: Some(evt_device_d0_entry),
        EvtDeviceD0Exit: Some(evt_device_d0_exit),
        EvtDeviceD0EntryPostInterruptsEnabled: Some(evt_device_d0_entry_post_interrupts_enabled),
        EvtDeviceD0ExitPreInterruptsDisabled: Some(evt_device_d0_exit_pre_interrupts_disabled),
        EvtDevicePrepareHardware: Some(evt_device_prepare_hardware),
        EvtDeviceReleaseHardware: Some(evt_device_release_hardware),
        EvtDeviceSelfManagedIoCleanup: Some(evt_device_self_managed_io_cleanup),
        EvtDeviceSelfManagedIoFlush: Some(evt_device_self_managed_io_flush),
        EvtDeviceSelfManagedIoInit: Some(evt_device_self_managed_io_start),
        EvtDeviceSelfManagedIoSuspend: Some(evt_device_self_managed_io_suspend),
        EvtDeviceSelfManagedIoRestart: Some(evt_device_self_managed_io_start),
        EvtDeviceSurpriseRemoval: Some(evt_device_surprise_removal),
        EvtDeviceQueryRemove: Some(evt_device_query_remove),
        EvtDeviceQueryStop: Some(evt_device_query_stop),
        EvtDeviceUsageNotification: Some(evt_device_usage_notification),
        EvtDeviceRelationsQuery: Some(evt_device_relations_query),
        EvtDeviceUsageNotificationEx: Some(evt_device_usage_notification_ex),
    };

    // Call WdfDeviceInitSetPnpPowerEventCallbacks and register the pnp/power callbacks

    let _ = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceInitSetPnpPowerEventCallbacks,
            device_init,
            &mut pnp_power_event_callbacks
        )
    };

    // todo!("WdfDeviceInitSetRequestAttributes");

    let mut wdfdevice_object_attributes = WDF_OBJECT_ATTRIBUTES {
        Size: core::mem::size_of::<WDF_OBJECT_ATTRIBUTES>() as ULONG,
        ExecutionLevel: _WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent,
        SynchronizationScope: _WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent,
        ..WDF_OBJECT_ATTRIBUTES::default()
    };

    let mut device = WDF_NO_HANDLE as WDFDEVICE;
    let mut nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceCreate,
            (core::ptr::addr_of_mut!(device_init)) as *mut *mut WDFDEVICE_INIT,
            &mut wdfdevice_object_attributes,
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

    // todo!("Initialize the I/O Package and any Queues");

    println!("Exit EvtDeviceAdd");

    nt_status
}