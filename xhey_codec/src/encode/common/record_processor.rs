use std::sync::mpsc::Sender;
use super::packets::*;

pub struct RecordProcessor {
    // 发送器 这里我们使用Rust的异步管道做缓冲
    sender: Sender<AudioPacket>,
    sample_rate: i32,
    // 当前这里存储的音频采样
    audio_samples: Vec<i16>,
    audio_samples_time_mills: f32,
    data_accumulate_time_mills: i64,
    // 当前音频采样的大小
    audio_samples_cursor: usize,
    // 每一个要入队的buffer大小
    audio_buffer_size: usize,
    audio_buffer_time_mills: f32,
    // 音频开始采集的时间
    is_recording_flag: bool,
    startTimeMills: i128,
}


impl RecordProcessor {
    pub fn new(sender: Sender<AudioPacket>) -> RecordProcessor {
        RecordProcessor {
            sender,
            sample_rate: 0_i32,
            audio_samples: Vec::<i16>::default(),
            audio_samples_time_mills: 0_f32,
            data_accumulate_time_mills: 0_i64,
            audio_samples_cursor: 0_usize,
            audio_buffer_size: 0_usize,
            audio_buffer_time_mills: 0_f32,
            is_recording_flag: false,
            startTimeMills: -1_i128,
        }
    }

    fn cpy_to_audio_samples(&mut self, mut source_buffer: Vec<i16>, start: usize, size: usize) {
        let slice: Vec<_> = source_buffer.drain(start..size).collect();
        self.audio_samples.copy_from_slice(&slice);
    }
    /// 将采样数据构造成AudioPacket并发送到异步队列
    fn flush_audio_buffer_to_queue(&mut self) {}

    pub fn push_audio_buffer_to_queue(&mut self, samples: Vec<i16>, size: usize) -> usize {
        if size <= 0 {
            return size;
        } else {
            let mut samples_cursor = 0;
            let mut samples_cnt = size;
            while samples_cnt > 0 {
                if (self.audio_samples_cursor + samples_cnt) < self.audio_buffer_size {
                    self.cpy_to_audio_samples(samples.clone(), 0, samples_cnt);
                    self.audio_samples_cursor += samples_cnt;
                    samples_cursor += samples_cnt;
                    samples_cnt = 0;
                } else {
                    let sub_full_size = self.audio_buffer_size - self.audio_samples_cursor;
                    self.cpy_to_audio_samples(samples.clone(), samples_cursor, sub_full_size);
                    self.audio_samples_cursor += sub_full_size;
                    samples_cursor += sub_full_size;
                    samples_cnt -= sub_full_size;
                    self.flush_audio_buffer_to_queue();
                }
            }
            return size;
        }
    }
}