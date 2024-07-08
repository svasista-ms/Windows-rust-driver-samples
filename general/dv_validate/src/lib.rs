#[cfg(not(test))]
extern crate wdk_panic;

use wdk::wdf;
#[cfg(not(test))]
use wdk_alloc::WDKAllocator;

use wdk_sys::{
    macros,
    ntddk::KeGetCurrentIrql,
    GUID,
    NTSTATUS,
    PVOID,
    ULONG,
    WDFOBJECT,
    WDFREQUEST,
    WDF_OBJECT_CONTEXT_TYPE_INFO,
};
use core::sync::atomic::AtomicI32;

use wdf_object_context::{wdf_declare_context_type, wdf_declare_context_type_with_name};

mod driver;
mod device;
mod wdf_object_context;
mod pnp_power_callbacks;

pub const GUID_DEVINTERFACE: GUID = GUID {
    Data1: 0xA1B2_C3D4u32,
    Data2: 0xE5F6u16,
    Data3: 0x7890u16,
    Data4: [
        0x01u8, 0x23u8, 0x45u8, 0x67u8, 0x89u8, 0xABu8, 0xCDu8, 0xEFu8,
    ],
};

// Declare queue context.
//
// ====== CONTEXT SETUP ========//

// The device context performs the same job as
// a WDM device extension in the driver frameworks
pub struct DeviceContext {
    private_device_data: ULONG, // just a placeholder
}
wdf_declare_context_type!(DeviceContext);

pub struct QueueContext {
    buffer: PVOID,
    length: usize,
    timer: wdf::Timer,
    current_request: WDFREQUEST,
    current_status: NTSTATUS,
    spin_lock: wdf::SpinLock,
}
wdf_declare_context_type_with_name!(QueueContext, queue_get_context);

pub struct RequestContext {
    cancel_completion_ownership_count: AtomicI32,
}
wdf_declare_context_type_with_name!(RequestContext, request_get_context);

