use wdk::{nt_success, paged_code, println};
use wdk_sys::{
    macros, ntddk::KeGetCurrentIrql, APC_LEVEL, DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT, PWDFDEVICE_INIT, ULONG, WDFDEVICE, WDFDEVICE_INIT, WDFDRIVER, WDFOBJECT, WDF_DRIVER_CONFIG, WDF_NO_HANDLE, WDF_NO_OBJECT_ATTRIBUTES,
    WDF_OBJECT_ATTRIBUTES, 
    WDF_PNPPOWER_EVENT_CALLBACKS, 
    _WDF_EXECUTION_LEVEL, _WDF_SYNCHRONIZATION_SCOPE
};

use crate::{wdf_object_context::wdf_get_context_type_info, wdf_object_get_device_context, DeviceContext};
use crate::{WDF_DEVICE_CONTEXT_TYPE_INFO, WDF_REQUEST_CONTEXT_TYPE_INFO};


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

    
    let mut driver_config = WDF_DRIVER_CONFIG {
        Size: core::mem::size_of::<WDF_DRIVER_CONFIG>() as ULONG,
        EvtDriverDeviceAdd: Some(evt_device_add),
        EvtDriverUnload: Some(evt_driver_unload),
        ..WDF_DRIVER_CONFIG::default()
    };
    
    println!("Driver Config: {:?}", driver_config);
    
    let driver_handle = WDF_NO_HANDLE.cast::<WDFDRIVER>();

    let nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDriverCreate,
            driver as PDRIVER_OBJECT,
            registry_path,
            // (core::ptr::addr_of_mut!(driver_object_attributes)) as *mut WDF_OBJECT_ATTRIBUTES,
            // &mut driver_object_attributes as *mut WDF_OBJECT_ATTRIBUTES,
            // &mut driver_object_attributes,
            WDF_NO_OBJECT_ATTRIBUTES,
            &mut driver_config,
            driver_handle
        )
    };

    println!("NTSTATUS: {:?}, Driver Handle After WdfDriverCreate: {:?}", nt_status, driver_handle);

    if !nt_status == 0 {
        println!("Error: WdfDriverCreate failed {nt_status:#010X}");
        return nt_status;
    }

    println!("Exit: DriverEntry");

    nt_status
}



// Driver Config
#[link_section = "PAGE"]
extern "C" fn evt_device_add(
    driver: WDFDRIVER, 
    device_init: PWDFDEVICE_INIT
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

    let mut pnp_power_callbacks: WDF_PNPPOWER_EVENT_CALLBACKS = WDF_PNPPOWER_EVENT_CALLBACKS {
        Size: core::mem::size_of::<WDF_PNPPOWER_EVENT_CALLBACKS>() as ULONG,
        ..WDF_PNPPOWER_EVENT_CALLBACKS::default()
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
    
    let mut request_attributes = WDF_OBJECT_ATTRIBUTES {
        Size: core::mem::size_of::<WDF_OBJECT_ATTRIBUTES>() as ULONG,
        ExecutionLevel: _WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent,
        SynchronizationScope: _WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent,
        ContextTypeInfo: wdf_get_context_type_info!(RequestContext),
        ..WDF_OBJECT_ATTRIBUTES::default()
    };

    let () = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceInitSetRequestAttributes,
            device_init,
            // WDF_NO_OBJECT_ATTRIBUTES
            &mut request_attributes
        )
    };
    
    println!("Request Attributes Set");
    
    let mut device_attributes = WDF_OBJECT_ATTRIBUTES {
        Size: core::mem::size_of::<WDF_OBJECT_ATTRIBUTES>() as ULONG,
        ExecutionLevel: _WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent,
        SynchronizationScope: _WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent,
        ContextTypeInfo: wdf_get_context_type_info!(DeviceContext),
        ..WDF_OBJECT_ATTRIBUTES::default()
    };

    println!("Device Attributes: {:?}", device_attributes);
    
    let mut device: WDFDEVICE = WDF_NO_HANDLE as WDFDEVICE;

    println!("Device Handle Before WdfDeviceCreate: {:?}", device);
    
    let mut nt_status: NTSTATUS = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceCreate,
            (core::ptr::addr_of_mut!(device_init)) as *mut *mut WDFDEVICE_INIT,
            // WDF_NO_OBJECT_ATTRIBUTES,
            &mut device_attributes,
            &mut device
        )
    };

    println!("Device Handle After WdfDeviceCreate: {:?}", device);
    
    println!("NTSTATUS: {:?} \n Device: {:?}", nt_status, device);
    
    if !nt_success(nt_status) {
        println!("Error: WdfDeviceCreate failed {nt_status:#010X}");
        return nt_status;
    }

    let device_context: *mut DeviceContext =
        unsafe { wdf_object_get_device_context(device as WDFOBJECT) };
    unsafe { (*device_context).private_device_data = 0 };

    println!("Device Context: {:?}", device_context);

    nt_status = unsafe {
        macros::call_unsafe_wdf_function_binding!(
            WdfDeviceCreateDeviceInterface,
            device,
            &crate::GUID_DEVINTERFACE_HELLO_WORLD,
            core::ptr::null_mut()
        )
    };
    
    if !nt_success(nt_status) {
        println!("Error: WdfDeviceCreateDeviceInterface failed {nt_status:#010X}");
        return nt_status;
    }

    println!("Exit: evt_device_add");
    
    nt_status
}

extern "C" fn evt_driver_unload(driver: WDFDRIVER) {
    println!("Enter: evt_driver_unload");

    println!("Driver: {:?}", driver);
    
    println!("Exit: evt_driver_unload");
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