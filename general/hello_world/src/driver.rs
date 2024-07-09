use wdk::{nt_success, paged_code, println};
use wdk_sys::{
    macros,
    ntddk::KeGetCurrentIrql,
    APC_LEVEL,
    DRIVER_OBJECT, GUID, NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT, ULONG, WDFDEVICE, WDFDEVICE_INIT, WDFDRIVER, WDF_DRIVER_CONFIG, WDF_NO_HANDLE, WDF_NO_OBJECT_ATTRIBUTES, WDF_OBJECT_ATTRIBUTES, _WDF_PNPPOWER_EVENT_CALLBACKS, 
    // _WDF_PNPPOWER_EVENT_CALLBACKS
    // _WDF_OBJECT_ATTRIBUTES,
};


const GUID_DEVINTERFACE_HELLO_WORLD: GUID = GUID {
    Data1: 0x4d36e988,
    Data2: 0xe315,
    Data3: 0x11ce,
    Data4: [0xbf, 0xc1, 0x08, 0x01, 0x2b, 0xe1, 0x03, 0x18],
};

#[link_section = "INIT"]
#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {

    println!("Enter: DriverEntry");

    println!("Hello, world!");

    println!("Driver: {:?}", driver);

    println!("Registry Path: {:?}", registry_path);
    
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
    
    let driver_handle: *mut WDFDRIVER = WDF_NO_HANDLE.cast::<WDFDRIVER>();

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
// extern "C" fn evt_driver_object_cleanup(driver_object: WDFOBJECT){
//     println!("Enter: evt_driver_object_cleanup");

//     println!("Driver Object: {:?}", driver_object);

//     println!("Exit: evt_driver_object_cleanup");
// }

// extern "C" fn evt_driver_object_destroy(driver_object: WDFOBJECT){
//     println!("Enter: evt_driver_object_destroy");

//     println!("Driver Object: {:?}", driver_object);

//     println!("Exit: evt_driver_object_destroy");
// }

// Driver Config
#[link_section = "PAGE"]
extern "C" fn evt_device_add(
    driver: WDFDRIVER, 
    device_init: *mut WDFDEVICE_INIT
) -> NTSTATUS {
    paged_code!();

    println!("Enter: evt_device_add");

    println!("Driver: {:?}", driver);

    println!("Device Init: {:?}", device_init);

    let mut device_init = unsafe {
        device_init
        .as_mut()
        .expect("Device Init cannot be null")
    };

    println!("Device Init: {:?}", device_init);

    let mut pnp_power_callbacks: _WDF_PNPPOWER_EVENT_CALLBACKS = _WDF_PNPPOWER_EVENT_CALLBACKS {
        Size: core::mem::size_of::<_WDF_PNPPOWER_EVENT_CALLBACKS>() as ULONG,
        .._WDF_PNPPOWER_EVENT_CALLBACKS::default()
    };

    println!("Pnp Power Callbacks: {:?}", pnp_power_callbacks);

    let () = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceInitSetPnpPowerEventCallbacks,
            device_init,
            &mut pnp_power_callbacks
        )
    };

    println!("Pnp Power Callbacks Set");

    // let _ = unsafe {
    //     macros::call_unsafe_wdf_function_binding!(
    //         WdfDeviceInitSetRequestAttributes,
    //         device_init,
    //         WDF_NO_OBJECT_ATTRIBUTES,
    //     )
    // };

    // println!("Request Attributes Set");

    let mut device: WDFDEVICE = WDF_NO_HANDLE as WDFDEVICE;

    println!("Device Handle Before WdfDeviceCreate: {:?}", device);

    let mut nt_status: NTSTATUS = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceCreate,
            (core::ptr::addr_of_mut!(device_init)) as *mut *mut WDFDEVICE_INIT,
            WDF_NO_OBJECT_ATTRIBUTES,
            &mut device
        )
    };

    println!("Device Handle After WdfDeviceCreate: {:?}", device);

    println!("NTSTATUS: {:?} \n Device: {:?}", nt_status, device);

    if !nt_success(nt_status) {
        println!("Error: WdfDeviceCreate failed {nt_status:#010X}");
        return nt_status;
    }

    nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceCreateDeviceInterface,
            device,
            &GUID_DEVINTERFACE_HELLO_WORLD,
            core::ptr::null_mut()
        )
    };

    if !nt_success(nt_status) {
        println!("Error: WdfDeviceCreateDeviceInterface failed {nt_status:#010X}");
        return nt_status;
    }

    println!("Exit: evt_device_add");

    0 as NTSTATUS
}

extern "C" fn evt_driver_unload(driver: WDFDRIVER) {
    println!("Enter: evt_driver_unload");

    println!("Driver: {:?}", driver);

    println!("Exit: evt_driver_unload");
}