use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};

#[derive(Copy, Clone, Debug)]
pub struct AudioPacket {
    pub samples: Vec<i16>,
    pub size: usize
}

impl AudioPacket {
    pub fn new() -> AudioPacket {
        AudioPacket {
            samples: Vec::<i16>::default(),
            size: 0
        }
    }
}

impl Drop for AudioPacket {
    fn drop(&mut self) {
        self.samples.clear();
        self.size = 0;
    }
}

