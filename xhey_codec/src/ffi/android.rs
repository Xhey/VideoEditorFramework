use rust_ffmpeg::*;

use jni::JNIEnv;
use jni::objects::JClass;
use android_logger::Filter;
use log::Level;
use std::ptr;
use jni::JavaVM;

use std::thread;
use std::sync::Arc;

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_demo_XHeyCodec_initLogger(env: JNIEnv, _: JClass) {
    android_logger::init_once(Filter::default().with_min_level(Level::Debug), Some("xhey_codec"));
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_xhey_demo_XHeyCodec_showFFmpegInfo(env: JNIEnv, _: JClass) {

}