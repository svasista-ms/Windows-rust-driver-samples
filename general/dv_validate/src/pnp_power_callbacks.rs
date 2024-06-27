use wdk_sys::{
    WDFDEVICE,
    NTSTATUS,
    WDF_POWER_DEVICE_STATE,
    WDFCMRESLIST
};

use wdk::println;

// PNP/Power callbacks
// EvtDeviceD0Entry - Called when the device enters the D0 state, which is the fully on state.
pub extern "C" fn evt_device_d0_entry(_device: WDFDEVICE, _previous_state: WDF_POWER_DEVICE_STATE) -> NTSTATUS {
    println!("Enter: EvtDeviceD0Entry");

    println!("Exit: EvtDeviceD0Entry");

    0
}
// EvtDeviceD0Exit - Called when the device exits the D0 state.
pub extern "C" fn evt_device_d0_exit(_device: WDFDEVICE, _target_state: WDF_POWER_DEVICE_STATE) -> NTSTATUS {
    println!("Enter: EvtDeviceD0Exit");

    println!("Exit: EvtDeviceD0Exit");

    0
}

pub extern "C" fn evt_device_d0_entry_post_interrupts_enabled(_device: WDFDEVICE, _previous_state: WDF_POWER_DEVICE_STATE) -> NTSTATUS {
    println!("Enter: EvtDeviceD0EntryPostInterruptsEnabled");

    println!("Exit: EvtDeviceD0EntryPostInterruptsEnabled");

    0
}

pub extern "C" fn evt_device_d0_exit_pre_interrupts_disabled(_device: WDFDEVICE, _target_state: WDF_POWER_DEVICE_STATE) -> NTSTATUS{
    println!("Enter: EvtDeviceD0ExitPreInterruptsDisabled");

    println!("Exit: EvtDeviceD0ExitPreInterruptsDisabled");

    0
}

pub extern "C" fn evt_device_prepare_hardware(_device: WDFDEVICE, _resources: WDFCMRESLIST, _requirements: WDFCMRESLIST) -> NTSTATUS {
    println!("Enter: EvtDevicePrepareHardware");

    println!("Exit: EvtDevicePrepareHardware");

    0
}

pub extern "C" fn evt_device_release_hardware(_device: WDFDEVICE, _resources: WDFCMRESLIST) -> NTSTATUS {
    println!("Enter: EvtDeviceReleaseHardware");

    println!("Exit: EvtDeviceReleaseHardware");

    0
}

pub extern "C" fn evt_device_self_managed_io_cleanup(_device: WDFDEVICE) {
    println!("Enter: EvtDeviceSelfManagedIoCleanup");

    println!("Exit: EvtDeviceSelfManagedIoCleanup");
}

pub extern "C" fn evt_device_self_managed_io_flush(_device: WDFDEVICE) {
    println!("Enter: EvtDeviceSelfManagedIoFlush");

    println!("Exit: EvtDeviceSelfManagedIoFlush");
}

pub extern "C" fn evt_device_self_managed_io_suspend(_device: WDFDEVICE) -> NTSTATUS{
    println!("Enter: EvtDeviceSelfManagedIoSuspend");

    println!("Exit: EvtDeviceSelfManagedIoSuspend");

    0
}

pub extern "C" fn evt_device_self_managed_io_start(_device: WDFDEVICE) -> NTSTATUS {
    println!("Enter: EvtDeviceSelfManagedIoStart");

    println!("Exit: EvtDeviceSelfManagedIoStart");

    0
}

pub extern "C" fn evt_device_surprise_removal(_device: WDFDEVICE) {
    println!("Enter: EvtDeviceSurpriseRemoval");

    println!("Exit: EvtDeviceSurpriseRemoval");
}

pub extern "C" fn evt_device_query_remove(_device: WDFDEVICE) -> NTSTATUS {
    println!("Enter: EvtDeviceQueryRemove");

    println!("Exit: EvtDeviceQueryRemove");

    0
}

pub extern "C" fn evt_device_query_stop(_device: WDFDEVICE) -> NTSTATUS {
    println!("Enter: EvtDeviceQueryStop");

    println!("Exit: EvtDeviceQueryStop");

    0
}

pub extern "C" fn evt_device_usage_notification(_device: WDFDEVICE, _notification_type: i32, _allow: u8) {
    println!("Enter: EvtDeviceUsageNotification");

    println!("Exit: EvtDeviceUsageNotification");
}

pub extern "C" fn evt_device_relations_query(_device: WDFDEVICE, _relations: i32) {
    println!("Enter: EvtDeviceRelationsQuery");

    println!("Exit: EvtDeviceRelationsQuery");
}

pub extern "C" fn evt_device_usage_notification_ex(_device: WDFDEVICE, _notification_type: i32, _allow: u8) -> NTSTATUS {
    println!("Enter: EvtDeviceUsageNotificationEx");

    println!("Exit: EvtDeviceUsageNotificationEx");

    0
}   