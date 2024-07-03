use wdk::{nt_success, println};

use wdk_sys::{
    macros, DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, ULONG, UNICODE_STRING, WDFDRIVER, WDFSTRING, WDF_DRIVER_CONFIG, WDF_NO_HANDLE, WDF_NO_OBJECT_ATTRIBUTES 
};

use crate::device::*;

extern crate alloc;

use alloc::string::String;


pub struct Driver<'a> {
    driver: &'a mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
    driver_handle: *mut WDFDRIVER,
    driver_config: WDF_DRIVER_CONFIG,
    driver_version: String,
}

#[link_section = "INIT"]
#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, registry_path: PCUNICODE_STRING) -> NTSTATUS {

    println!("Enter: DriverEntry Routine");

    let mut driver_object = Driver {
        driver,
        registry_path,
        driver_handle: WDF_NO_HANDLE.cast::<WDFDRIVER>(),
        driver_config: WDF_DRIVER_CONFIG {
            Size: core::mem::size_of::<WDF_DRIVER_CONFIG>() as ULONG,
            EvtDriverDeviceAdd: Some(evt_device_add),
            EvtDriverUnload: Some(evt_driver_unload),
            // DriverInitFlags: 0x0,
            // DriverPoolTag: 0x546C_6C65, // "Tlle"
            ..WDF_DRIVER_CONFIG::default()
        },
        driver_version: String::new(),
    };

    let nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDriverCreate,
            driver_object.driver,
            registry_path,
            WDF_NO_OBJECT_ATTRIBUTES,
            &mut driver_object.driver_config,
            driver_object.driver_handle,
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDriverCreate failed {nt_status:#010X}");
        return nt_status;
    }

    // set_driver_version(&mut driver_object);

    // println!("Driver Version: {:?}", driver_object.driver_version);

    // println!("Driver Registry Path: {:?}", driver_object.registry_path);

    println!("Exit: DriverEntry Routine");

    nt_status
}

extern "C" fn evt_driver_unload(_driver: WDFDRIVER) {
    println!("Enter: EvtDriverUnload");

    println!("Exit: EvtDriverUnload");
}

fn set_driver_version(driver_object: &mut Driver) -> NTSTATUS {
    println!("Enter: set_driver_version");

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

    nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDriverRetrieveVersionString,
            *driver_object.driver_handle,
            string
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDriverRetrieveVersionString failed {nt_status:#010X}");
        return nt_status;
    }
    
    let _ = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfStringGetUnicodeString,
            string,
            &mut us
        )
    };

    driver_object.driver_version = String::from_utf16_lossy(unsafe {
        core::slice::from_raw_parts(
            us.Buffer,
            us.Length as usize / core::mem::size_of_val(&(*us.Buffer)),
        )
    });

    println!("Exit: set_driver_version");

    nt_status
}