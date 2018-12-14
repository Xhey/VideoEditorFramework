use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};

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

