use wdk::{nt_success, paged_code, println};
use wdk_sys::{
    macros::call_unsafe_wdf_function_binding, 
    ntddk::KeGetCurrentIrql, APC_LEVEL,
    NTSTATUS, ULONG, WDFDEVICE, WDFMEMORY, WDF_NO_OBJECT_ATTRIBUTES, _POOL_TYPE::NonPagedPoolNx
};

pub unsafe fn echo_memory_create(_device: WDFDEVICE) -> NTSTATUS {

    paged_code!();
    
    let buffer_size: usize = 512; // Length of the allocated memory buffer
    let mut memory_handle: WDFMEMORY = core::ptr::null_mut(); // Placeholder for memory handle
    let pool_tag: ULONG = 0; // Pool tag for memory allocation

    let nt_status: NTSTATUS = unsafe {
        call_unsafe_wdf_function_binding!(
            WdfMemoryCreate,
            WDF_NO_OBJECT_ATTRIBUTES,
            NonPagedPoolNx,
            pool_tag,
            buffer_size,
            &mut memory_handle,
            core::ptr::null_mut(),
        )
    };

    if !nt_success(nt_status) {
        // Handle error
        println!("Memory allocation failed with status: {}", nt_status);
        return nt_status;
    }

    println!("Memory allocated successfully.");
    // Use the allocated memory via `buffer` pointer

    nt_status
}
