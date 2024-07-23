use core::ptr::null_mut;

use wdk::{nt_success, paged_code, println};
use wdk_sys::{
    macros::call_unsafe_wdf_function_binding, ntddk::KeGetCurrentIrql, APC_LEVEL, NTSTATUS, PVOID, ULONG, WDFDEVICE, WDFMEMORY, WDFOBJECT, 
    WDF_NO_OBJECT_ATTRIBUTES, 
    _POOL_TYPE::NonPagedPoolNx, _WDF_EXECUTION_LEVEL, _WDF_OBJECT_ATTRIBUTES, _WDF_SYNCHRONIZATION_SCOPE
};

pub unsafe fn echo_memory_create(_device: WDFDEVICE) -> NTSTATUS {

    paged_code!();
    
    let buffer_size: usize = 512; // Length of the allocated memory buffer
    let mut memory_handle: WDFMEMORY = core::ptr::null_mut(); // Placeholder for memory handle
    let pool_tag: ULONG = 0; // Pool tag for memory allocation
    let mut buffer_ptr: PVOID = null_mut(); // Placeholder for memory buffer pointer

    let mut attributes: _WDF_OBJECT_ATTRIBUTES = _WDF_OBJECT_ATTRIBUTES {
        Size: core::mem::size_of::<_WDF_OBJECT_ATTRIBUTES>() as ULONG,
        ExecutionLevel: _WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent,
        SynchronizationScope: _WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent,
        ParentObject: _device as WDFOBJECT,
        .._WDF_OBJECT_ATTRIBUTES::default()
    };


    let nt_status: NTSTATUS = unsafe {
        call_unsafe_wdf_function_binding!(
            WdfMemoryCreate,
            // &mut attributes,
            WDF_NO_OBJECT_ATTRIBUTES,
            NonPagedPoolNx,
            pool_tag,
            buffer_size,
            &mut memory_handle,
            &mut buffer_ptr,
        )
    };

    if !buffer_ptr.is_null() {
        println!("Memory buffer allocated at: {:?}", buffer_ptr);
        
        buffer_ptr.write_bytes(0xAB, buffer_size);

        let invalid_buffer_ptr = unsafe { buffer_ptr.offset(buffer_size as isize + 1) };

        unsafe {
            println!("Invalid buffer pointer: {:?}", invalid_buffer_ptr);
            invalid_buffer_ptr.write_bytes(0xCD, buffer_size);
        }

    }

    if !nt_success(nt_status) {
        // Handle error
        println!("Memory allocation failed with status: {}", nt_status);
        return nt_status;
    }

    println!("Exit: echo_memory_create");

    nt_status
}
