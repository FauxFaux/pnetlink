//! Netlink is a linux kernel interface used for communication between
//! kernel and userspace.
//!
//! `socket` module can be used to establish Netlink socket
//! `packet` contains high level functions and traits
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate libc;
extern crate pnet;
extern crate pnet_macros_support;

pub mod socket;
pub mod packet;
pub mod util;
