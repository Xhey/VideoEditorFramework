use rust_ffmpeg::*;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jbyteArray, jint};
use android_logger::Filter;
use log::Level;
use std::ptr;
use jni::JavaVM;
use std::thread;
use std::sync::Arc;
use crate::encode::common::record_processor::RecordProcessor;
use crate::encode::encoder::audio_encoder::PCMWriter;
use std::sync::mpsc::channel;

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_demo_XHeyCodec_initLogger(env: JNIEnv, _: JClass) {
    android_logger::init_once(Filter::default().with_min_level(Level::Debug), Some("xhey_codec"));
}

/// 初始化录制处理器 返回结构指针
#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_demo_XHeyCodec_initRecordProcessor(env: JNIEnv,
                                                                          _: JClass,
                                                                          filePath: JString,
                                                                          sampleRate: jint,
                                                                          audioBufferSize: jint) -> jlong {
    let (sender, receiver) = channel();
    let file_path: String = env.get_string(filePath).expect("Couldn't get java string!").into();
    let processor = Box::new(RecordProcessor::new(sender.clone()));
    let wtr = PCMWriter::new(file_path);
    wtr.start(receiver);
    Box::into_raw(processor) as jlong
}

/// 将数据发送到异步管道
#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_demo_XHeyCodec_pushAudioBufferToQueue(env: JNIEnv, _: JClass,
                                                                             processorPointer: jlong,
                                                                             samples: jbyteArray,
                                                                             dataLen: jint) {
    let processor = (processorPointer as *mut RecordProcessor).as_mut().unwrap();
    let samples_data = env.convert_byte_array(samples).unwrap();
    let len = processor.push_audio_buffer_to_queue(samples_data, dataLen as usize);
}
