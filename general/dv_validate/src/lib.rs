#![no_std]
#![cfg_attr(feature = "nightly", feature(hint_must_use))]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_safety_doc)]

mod driver;
mod device;
mod pnp_power_callbacks;
use wdk_alloc::WDKAllocator;
#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

#[cfg(not(test))]
extern crate wdk_panic;


