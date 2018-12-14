use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::ptr;
use super::packets::*;

#[derive(Debug)]
pub struct RecordProcessor {
    // 发送器 这里我们使用Rust的异步管道做缓冲
    sender: Sender<AudioPacket>,
    // 音频开始采集的时间
    is_recording_flag: bool,
    startTimeMills: i128,
}


impl RecordProcessor {
    pub fn new(sender: Sender<AudioPacket>) -> RecordProcessor {
        RecordProcessor {
            sender,
            is_recording_flag: false,
            startTimeMills: -1_i128,
        }
    }


    pub fn push_audio_buffer_to_queue(&mut self, samples: Vec<u8>, size: usize) -> usize {
        if size <= 0 {
            return size;
        } else {
            let mut packet = AudioPacket::new( samples, size);
            self.sender.send(packet);
            return size;
        }
    }
}