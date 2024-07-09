use wdk::println;
use wdk_sys::{
    macros, DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT, ULONG, WDFDEVICE_INIT, WDFDRIVER, WDFOBJECT, WDF_DRIVER_CONFIG, WDF_NO_HANDLE, 
    WDF_NO_OBJECT_ATTRIBUTES, 
    WDF_OBJECT_ATTRIBUTES,
    // _WDF_OBJECT_ATTRIBUTES,
 };

#[link_section = "INIT"]
#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {

    println!("Enter: DriverEntry");

    println!("Hello, world!");

    // println!("Driver: {:?}", driver);

    // println!("Registry Path: {:?}", registry_path);

    
    let driver_handle: *mut WDFDRIVER = WDF_NO_HANDLE.cast::<WDFDRIVER>();
    
    let mut _driver_object_attributes = WDF_OBJECT_ATTRIBUTES {
        Size: core::mem::size_of::<WDF_OBJECT_ATTRIBUTES>() as ULONG,
        // EvtCleanupCallback: Some(evt_driver_object_cleanup),
        // EvtDestroyCallback: Some(evt_driver_object_destroy),
        ..WDF_OBJECT_ATTRIBUTES::default()
    };

    // let attributes_ptr_mut: *mut _WDF_OBJECT_ATTRIBUTES = ; 
    
    let mut driver_config = WDF_DRIVER_CONFIG {
        Size: core::mem::size_of::<WDF_DRIVER_CONFIG>() as ULONG,
        EvtDriverDeviceAdd: Some(evt_device_add),
        EvtDriverUnload: Some(evt_driver_unload),
        ..WDF_DRIVER_CONFIG::default()
    };

    let nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDriverCreate,
            driver as PDRIVER_OBJECT,
            registry_path,
            WDF_NO_OBJECT_ATTRIBUTES,
            // &mut driver_object_attributes as *mut _WDF_OBJECT_ATTRIBUTES,
            &mut driver_config,
            driver_handle
        )
    };

    if !nt_status == 0 {
        println!("Error: WdfDriverCreate failed {nt_status:#010X}");
        return nt_status;
    }

    println!("Exit: DriverEntry");

    nt_status
}


// Driver Object Attributes
extern "C" fn evt_driver_object_cleanup(driver_object: WDFOBJECT){
    println!("Enter: evt_driver_object_cleanup");

    println!("Driver Object: {:?}", driver_object);

    println!("Exit: evt_driver_object_cleanup");
}

extern "C" fn evt_driver_object_destroy(driver_object: WDFOBJECT){
    println!("Enter: evt_driver_object_destroy");

    println!("Driver Object: {:?}", driver_object);

    println!("Exit: evt_driver_object_destroy");
}

// Driver Config
#[link_section = "PAGE"]
extern "C" fn evt_device_add(
    driver: WDFDRIVER, 
    device_init: *mut WDFDEVICE_INIT
) -> NTSTATUS {

    println!("Enter: evt_device_add");

    println!("Driver: {:?}", driver);



    println!("Device Init: {:?}", device_init);

    println!("Exit: evt_device_add");

    0 as NTSTATUS
}

extern "C" fn evt_driver_unload(driver: WDFDRIVER) {
    println!("Enter: evt_driver_unload");

    println!("Driver: {:?}", driver);

    println!("Exit: evt_driver_unload");
}