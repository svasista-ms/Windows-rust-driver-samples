use wdk_sys::{
    macros, NTSTATUS, PCUNICODE_STRING, ULONG, UNICODE_STRING, WDFDEVICE, WDFDEVICE_INIT, WDF_NO_HANDLE, WDF_OBJECT_ATTRIBUTES, WDF_PNPPOWER_EVENT_CALLBACKS, _WDF_EXECUTION_LEVEL, _WDF_SYNCHRONIZATION_SCOPE
};

use wdk::{nt_success, println};

use crate::{pnp_power_callbacks::*, GUID_DEVINTERFACE};

extern crate alloc;
use alloc::string::String;

// EvtDeviceAdd - Called by the framework in response to AddDevice call from the PnP manager.
#[link_section = "PAGE"]


pub fn device_create(mut device_init: &mut WDFDEVICE_INIT) -> NTSTATUS {
    // WdfDeviceInitAssignName method assigns a device name to a device's device object.

    let device_name_utf16: &[u16] = &[
    'D' as u16, 'e' as u16, 'v' as u16, 'i' as u16, 'c' as u16, 'e' as u16, '0' as u16, '1' as u16, 0
    ];

    // Create the UNICODE_STRING
    let pus: PCUNICODE_STRING = &UNICODE_STRING {
        Length: (device_name_utf16.len() * 2) as u16 - 2, // Length in bytes, excluding null terminator
        MaximumLength: (device_name_utf16.len() * 2) as u16, // Maximum length in bytes
        Buffer: device_name_utf16.as_ptr() as *mut u16, // Pointer to the UTF-16 array
    };

    let device_name_string = String::from_utf16_lossy(device_name_utf16);

    println!("Device Name: {:?}", device_name_string);
    
    let nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceInitAssignName,
            device_init,
            pus
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDeviceInitAssignName failed {nt_status:#010X}");
        return nt_status;
    }

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

    let mut attributes = WDF_OBJECT_ATTRIBUTES {
        Size: core::mem::size_of::<WDF_OBJECT_ATTRIBUTES>() as ULONG,
        ExecutionLevel: _WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent,
        SynchronizationScope: _WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent,
        ..WDF_OBJECT_ATTRIBUTES::default()
    };

    let [()] = [unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceInitSetRequestAttributes,
            device_init,
            &mut attributes
        );
    }];

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

    println!("NT_STATUS: After WdfDeviceCreate {:#010X}", nt_status);
    nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceCreateDeviceInterface,
            device,
            &GUID_DEVINTERFACE,
            core::ptr::null_mut(),
        )
    };

    println!("NT_STATUS: After WdfDeviceCreateDeviceInterface {:#010X}", nt_status);

    if !nt_success(nt_status) {
        println!("Error: WdfDeviceCreateDeviceInterface failed {nt_status:#010X}");
        return nt_status;
    }

    // todo!("Initialize the I/O Package and any Queues");

    println!("Exit EvtDeviceAdd");

    nt_status
}