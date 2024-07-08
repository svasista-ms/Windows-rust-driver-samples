#![no_std]

#[cfg(not(test))]
extern crate wdk_panic;

use wdk_sys::{
    DRIVER_OBJECT,
    NTSTATUS,
    PCUNICODE_STRING,
 };

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    0
}