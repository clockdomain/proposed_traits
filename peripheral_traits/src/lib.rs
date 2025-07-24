#![no_std]
#![deny(unsafe_code)]

pub mod block_device;
pub mod i2c_target;
pub mod i3c_master;
pub mod i3c_target;
pub mod otp;
pub mod otp_aspeed;
pub mod system_control;