use wdk::{nt_success, println};

use wdk_sys::{
    macros, 
    DRIVER_OBJECT, 
    NTSTATUS, 
    PCUNICODE_STRING,
    PDRIVER_OBJECT, 
    ULONG,
    WDFDRIVER, 
    WDF_DRIVER_CONFIG, 
    WDF_NO_OBJECT_ATTRIBUTES,
    WDF_NO_HANDLE 
};

use crate::device::*;

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

    // todo(print_driver_version())

    // let driver_object: WDFDRIVER = unsafe {
    //     macros::call_unsafe_wdf_function_binding!(
    //         WdfGetDriver,
    //     )
    // };

    // println!("Driver Object: {:?}", driver);

    let registry_path: *mut u16 = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDriverGetRegistryPath,
            *driver_handle_output
        )
    };

    println!("Registry Path: {:?}", registry_path);

    println!("Exit: DriverEntry Routine");

    nt_status
}

extern "C" fn evt_driver_unload(_driver: WDFDRIVER) {
    println!("Enter: EvtDriverUnload");

    println!("Exit: EvtDriverUnload");
}