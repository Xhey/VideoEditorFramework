use rust_ffmpeg::*;

pub struct VideoEnc {
    av_codec: Box<AVCodec>,
    av_codec_ctx: Box<AVCodecContext>,
    av_frame: Box<AVFrame>,
    av_packet: Box<AVPacket>
}
