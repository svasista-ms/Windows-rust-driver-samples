use wdk::{nt_success, println};

use wdk_sys::{
    macros, DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, ULONG, UNICODE_STRING, WDFDRIVER, WDFOBJECT, WDFSTRING, WDF_DRIVER_CONFIG, WDF_DRIVER_VERSION_AVAILABLE_PARAMS, WDF_NO_HANDLE, WDF_NO_OBJECT_ATTRIBUTES, _UNICODE_STRING, _WDF_DRIVER_VERSION_AVAILABLE_PARAMS 
};

use crate::device::*;

extern crate alloc;

use alloc::string::String;

#[link_section = "INIT"]
#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, registry_path: PCUNICODE_STRING) -> NTSTATUS {

    println!("Enter: DriverEntry Routine");

    let mut driver_config = WDF_DRIVER_CONFIG {
        Size: core::mem::size_of::<WDF_DRIVER_CONFIG>() as ULONG,
        EvtDriverDeviceAdd: Some(evt_device_add),
        EvtDriverUnload: Some(evt_driver_unload),
        ..WDF_DRIVER_CONFIG::default()
    };

    let driver_handle_output = WDF_NO_HANDLE.cast::<WDFDRIVER>();

    let mut nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDriverCreate,
            driver,
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

    // nt_status = print_driver_version();

    // if !nt_success(nt_status) {
    //     println!("Error: print_driver_version failed {nt_status:#010X}");
    //     return nt_status;
    // }

    // println!("Driver Registry Path: {:?}", crate::utils::print_punicode_string(registry_path as *mut _UNICODE_STRING));

    println!("Exit: DriverEntry Routine");

    nt_status
}


fn print_driver_version() -> NTSTATUS {
    println!("Enter: print_driver_version");
    
    let mut string: WDFSTRING = core::ptr::null_mut();
    let mut us: UNICODE_STRING = UNICODE_STRING::default();
    let mut nt_status: i32 = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfStringCreate,
            core::ptr::null_mut(),
            WDF_NO_OBJECT_ATTRIBUTES,
            &mut string
        )
    };
    
    if !nt_success(nt_status) {
        println!("Error: WdfStringCreate failed {nt_status:#010X}");
        return nt_status;
    }

    let driver = unsafe { (*wdk_sys::WdfDriverGlobals).Driver };

    nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDriverRetrieveVersionString,
            driver,
            string
        )
    };
    
    if !nt_success(nt_status) {
        println!("Error: WdfDriverRetrieveVersionString failed {nt_status:#010X}");
        return nt_status;
    }
    
    let () = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfStringGetUnicodeString,
            string,
            &mut us
        )
    };

    let driver_version = String::from_utf16_lossy(unsafe {
        core::slice::from_raw_parts(
            us.Buffer,
            us.Length as usize / core::mem::size_of_val(&(*us.Buffer)),
        )
    });

    println!("Driver Version: {}", driver_version);

    let () = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfObjectDelete,
            string as WDFOBJECT
        )
    };

    let mut ver: _WDF_DRIVER_VERSION_AVAILABLE_PARAMS = WDF_DRIVER_VERSION_AVAILABLE_PARAMS {
        Size: core::mem::size_of::<WDF_DRIVER_VERSION_AVAILABLE_PARAMS>() as ULONG,
        MajorVersion: 1,
        MinorVersion: 0,
    };

    let res = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDriverIsVersionAvailable,
            driver,
            &mut ver
        )
    };

    if res > 0 {
        println!("Yes, framework version is 1.0");
    } else {
        println!("No, framework version is not 1.0");
    }

    println!("Exit: print_driver_version");
    
    nt_status
}
    
    
extern "C" fn evt_driver_unload(_driver: WDFDRIVER) {
    println!("Enter: EvtDriverUnload");

    println!("Exit: EvtDriverUnload");
}