use wdk::{nt_success, println, paged_code};

use wdk_sys::{
    WDFCMRESLIST,
    ntddk::KeGetCurrentIrql,
    GUID,
    macros, 
    DRIVER_OBJECT, 
    NTSTATUS, 
    PCUNICODE_STRING, 
    PDRIVER_OBJECT, 
    PWDFDEVICE_INIT, 
    ULONG,
    WDFDRIVER, 
    WDF_DRIVER_CONFIG, 
    WDF_NO_HANDLE, 
    WDF_NO_OBJECT_ATTRIBUTES, 
    APC_LEVEL,
    WDF_PNPPOWER_EVENT_CALLBACKS,
    WDFDEVICE,
    WDFDEVICE_INIT,
    WDF_OBJECT_ATTRIBUTES,
    WDF_POWER_DEVICE_STATE,
    _WDF_EXECUTION_LEVEL,
    _WDF_SYNCHRONIZATION_SCOPE,
};

const GUID_DEVINTERFACE: GUID = GUID {
    Data1: 0xCDC3_5B6A,
    Data2: 0x0BE5,
    Data3: 0x4934,
    Data4: [0xBA, 0x5C, 0x55, 0x37, 0x38, 0x0A, 0x7C, 0x1A],
};


#[link_section = "INIT"]
#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, registry_path: PCUNICODE_STRING) -> NTSTATUS {

    println!("Enter: DriverEntry Routine");

    let mut driver_config = WDF_DRIVER_CONFIG {
        Size: core::mem::size_of::<WDF_DRIVER_CONFIG>() as ULONG,
        EvtDriverDeviceAdd: Some(evt_device_add),
        EvtDriverUnload: Some(evt_driver_unload),
        // DriverInitFlags: 0x0,
        // DriverPoolTag: 0x546C_6C65, // "Tlle"
        ..WDF_DRIVER_CONFIG::default()
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

    // print_driver_version();

    println!("Exit: DriverEntry Routine");

    nt_status


}

// Device/Driver lifecycle callbacks
// EvtDeviceAdd - Called by the framework in response to AddDevice call from the PnP manager.
extern "C" fn evt_device_add(_driver: WDFDRIVER, device_init: PWDFDEVICE_INIT) -> NTSTATUS {
    paged_code!();

    println!("Enter: EvtDeviceAdd");

    let mut device_init = unsafe {
        device_init.as_mut().expect("device_init is null. WDF should never provide a null pointer for device_init")
    };

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

    if nt_success(nt_status) {
        nt_status = unsafe {
            macros::call_unsafe_wdf_function_binding!(
                WdfDeviceCreateDeviceInterface,
                device,
                &GUID_DEVINTERFACE,
                core::ptr::null_mut(),
            )
        };
        if nt_success(nt_status) {
            // Initialize the I/O Package and any Queues
            // nt_status = unsafe { echo_queue_initialize(device) };
            println!("Initialize the I/O Package and any Queues");
        }
    } else {
        // todo!("WdfDeviceCreate failed");
    }

    println!("Exit EvtDeviceAdd");

    nt_status
}

extern "C" fn evt_driver_unload(_driver: WDFDRIVER) {
    println!("Enter: EvtDriverUnload");

    println!("Exit: EvtDriverUnload");
}



// PNP/Power callbacks
// EvtDeviceD0Entry - Called when the device enters the D0 state, which is the fully on state.
extern "C" fn evt_device_d0_entry(_device: WDFDEVICE, _previous_state: WDF_POWER_DEVICE_STATE) -> NTSTATUS {
    println!("Enter: EvtDeviceD0Entry");

    println!("Exit: EvtDeviceD0Entry");

    0
}
// EvtDeviceD0Exit - Called when the device exits the D0 state.
extern "C" fn evt_device_d0_exit(_device: WDFDEVICE, _target_state: WDF_POWER_DEVICE_STATE) -> NTSTATUS {
    println!("Enter: EvtDeviceD0Exit");

    println!("Exit: EvtDeviceD0Exit");

    0
}

extern "C" fn evt_device_d0_entry_post_interrupts_enabled(_device: WDFDEVICE, _previous_state: WDF_POWER_DEVICE_STATE) -> NTSTATUS {
    println!("Enter: EvtDeviceD0EntryPostInterruptsEnabled");

    println!("Exit: EvtDeviceD0EntryPostInterruptsEnabled");

    0
}

extern "C" fn evt_device_d0_exit_pre_interrupts_disabled(_device: WDFDEVICE, _target_state: WDF_POWER_DEVICE_STATE) -> NTSTATUS{
    println!("Enter: EvtDeviceD0ExitPreInterruptsDisabled");

    println!("Exit: EvtDeviceD0ExitPreInterruptsDisabled");

    0
}

extern "C" fn evt_device_prepare_hardware(_device: WDFDEVICE, _resources: WDFCMRESLIST, _requirements: WDFCMRESLIST) -> NTSTATUS {
    println!("Enter: EvtDevicePrepareHardware");

    println!("Exit: EvtDevicePrepareHardware");

    0
}

extern "C" fn evt_device_release_hardware(_device: WDFDEVICE, _resources: WDFCMRESLIST) -> NTSTATUS {
    println!("Enter: EvtDeviceReleaseHardware");

    println!("Exit: EvtDeviceReleaseHardware");

    0
}

extern "C" fn evt_device_self_managed_io_cleanup(_device: WDFDEVICE) {
    println!("Enter: EvtDeviceSelfManagedIoCleanup");

    println!("Exit: EvtDeviceSelfManagedIoCleanup");
}

extern "C" fn evt_device_self_managed_io_flush(_device: WDFDEVICE) {
    println!("Enter: EvtDeviceSelfManagedIoFlush");

    println!("Exit: EvtDeviceSelfManagedIoFlush");
}

extern "C" fn evt_device_self_managed_io_suspend(_device: WDFDEVICE) -> NTSTATUS{
    println!("Enter: EvtDeviceSelfManagedIoSuspend");

    println!("Exit: EvtDeviceSelfManagedIoSuspend");

    0
}

extern "C" fn evt_device_self_managed_io_start(_device: WDFDEVICE) -> NTSTATUS {
    println!("Enter: EvtDeviceSelfManagedIoStart");

    println!("Exit: EvtDeviceSelfManagedIoStart");

    0
}

extern "C" fn evt_device_surprise_removal(_device: WDFDEVICE) {
    println!("Enter: EvtDeviceSurpriseRemoval");

    println!("Exit: EvtDeviceSurpriseRemoval");
}

extern "C" fn evt_device_query_remove(_device: WDFDEVICE) -> NTSTATUS {
    println!("Enter: EvtDeviceQueryRemove");

    println!("Exit: EvtDeviceQueryRemove");

    0
}

extern "C" fn evt_device_query_stop(_device: WDFDEVICE) -> NTSTATUS {
    println!("Enter: EvtDeviceQueryStop");

    println!("Exit: EvtDeviceQueryStop");

    0
}

extern "C" fn evt_device_usage_notification(_device: WDFDEVICE, _notification_type: i32, _allow: u8) {
    println!("Enter: EvtDeviceUsageNotification");

    println!("Exit: EvtDeviceUsageNotification");
}

extern "C" fn evt_device_relations_query(_device: WDFDEVICE, _relations: i32) {
    println!("Enter: EvtDeviceRelationsQuery");

    println!("Exit: EvtDeviceRelationsQuery");
}

extern "C" fn evt_device_usage_notification_ex(_device: WDFDEVICE, _notification_type: i32, _allow: u8) -> NTSTATUS {
    println!("Enter: EvtDeviceUsageNotificationEx");

    println!("Exit: EvtDeviceUsageNotificationEx");

    0
}