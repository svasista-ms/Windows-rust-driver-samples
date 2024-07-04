#![no_std]
#![cfg_attr(feature = "nightly", feature(hint_must_use))]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_safety_doc)]

use wdk_sys::GUID;

mod driver;
mod device;
mod pnp_power_callbacks;
mod utils;

pub const GUID_DEVINTERFACE: GUID = GUID {
    Data1: 0xA1B2_C3D4u32,
    Data2: 0xE5F6u16,
    Data3: 0x7890u16,
    Data4: [
        0x01u8, 0x23u8, 0x45u8, 0x67u8, 0x89u8, 0xABu8, 0xCDu8, 0xEFu8,
    ],
};

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: wdk_alloc::WDKAllocator = wdk_alloc::WDKAllocator;

#[cfg(not(test))]
extern crate wdk_panic;


