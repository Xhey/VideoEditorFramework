use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::mem;
use rust_ffmpeg::*;
pub use rust_ffmpeg::AVPacket;
use libc::memset;

pub trait SafelyPacketMgr {
    fn new() -> Box<AVPacket> {
        unsafe {
            let mut packet = av_packet_alloc();
            Box::new(*packet)
        }
    }
}

impl SafelyPacketMgr for AVPacket {}


#[derive(Debug)]
pub struct AudioPacket {
    samples: Vec<u8>,
    size: usize
}

impl AudioPacket {
    pub fn new(samples_data: Vec<u8>, size: usize) -> AudioPacket {
        let mut samples = Vec::<u8>::with_capacity(size);
        unsafe { samples.set_len(size) };
        samples.copy_from_slice(&samples_data);
        AudioPacket {
            samples,
            size
        }
    }

    pub fn samples(&self) -> &Vec<u8> {
        &self.samples
    }

    pub fn samples_size(&self) -> &usize {
        &self.size
    }
}

impl Drop for AudioPacket {
    fn drop(&mut self) {
        self.samples.clear();
        self.size = 0;
    }
}

