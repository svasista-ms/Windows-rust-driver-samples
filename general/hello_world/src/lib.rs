#![no_std]
#![cfg_attr(feature = "nightly", feature(hint_must_use))]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_safety_doc)]

mod driver;

#[cfg(not(test))]
extern crate wdk_panic;

use core::sync::atomic::AtomicI32;
use wdk_alloc::WDKAllocator;
#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;


use wdk::wdf;
use wdk_sys::{
    macros,
    GUID,
    NTSTATUS,
    PVOID,
    ULONG,
    WDFOBJECT,
    WDFREQUEST,
    WDF_OBJECT_CONTEXT_TYPE_INFO,
};


mod wdf_object_context;
use wdf_object_context::{wdf_declare_context_type, wdf_declare_context_type_with_name};

const GUID_DEVINTERFACE_HELLO_WORLD: GUID = GUID {
    Data1: 0x4d36e988,
    Data2: 0xe315,
    Data3: 0x11ce,
    Data4: [0xbf, 0xc1, 0x08, 0x01, 0x2b, 0xe1, 0x03, 0x18],
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