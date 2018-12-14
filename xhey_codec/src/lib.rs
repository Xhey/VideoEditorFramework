#![allow(non_sname_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
extern crate fnv;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate libc;
extern crate jni;
extern crate android_logger;
extern crate arrayvec;
extern crate rust_ffmpeg;

pub mod ffi;
pub mod encode;