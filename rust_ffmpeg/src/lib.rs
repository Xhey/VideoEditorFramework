#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;

/// Structure describes basic parameters of the device.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVDeviceInfo {
    /// < device name, format depends on device
    pub device_name: *mut libc::c_char,
    /// < human friendly name
    pub device_description: *mut libc::c_char,
}

/// List of devices.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVDeviceInfoList {
    /// < list of autodetected devices
    pub devices: *mut *mut AVDeviceInfo,
    /// < number of autodetected devices
    pub nb_devices: libc::c_int,
    /// < index of default device or -1 if no default
    pub default_device: libc::c_int,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVBuffer {
    _unused: [u8; 0]
}

#[repr(u32)]      /// @defgroup lavc_packet AVPacket
///
/// Types and functions for working with AVPacket.
/// @{
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVPacketSideDataType {
    AV_PKT_DATA_PALETTE = 0,
    AV_PKT_DATA_NEW_EXTRADATA = 1,
    AV_PKT_DATA_PARAM_CHANGE = 2,
    AV_PKT_DATA_H263_MB_INFO = 3,
    AV_PKT_DATA_REPLAYGAIN = 4,
    AV_PKT_DATA_DISPLAYMATRIX = 5,
    AV_PKT_DATA_STEREO3D = 6,
    AV_PKT_DATA_AUDIO_SERVICE_TYPE = 7,
    AV_PKT_DATA_QUALITY_STATS = 8,
    AV_PKT_DATA_FALLBACK_TRACK = 9,
    AV_PKT_DATA_CPB_PROPERTIES = 10,
    AV_PKT_DATA_SKIP_SAMPLES = 11,
    AV_PKT_DATA_JP_DUALMONO = 12,
    AV_PKT_DATA_STRINGS_METADATA = 13,
    AV_PKT_DATA_SUBTITLE_POSITION = 14,
    AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL = 15,
    AV_PKT_DATA_WEBVTT_IDENTIFIER = 16,
    AV_PKT_DATA_WEBVTT_SETTINGS = 17,
    AV_PKT_DATA_METADATA_UPDATE = 18,
    AV_PKT_DATA_MPEGTS_STREAM_ID = 19,
    AV_PKT_DATA_MASTERING_DISPLAY_METADATA = 20,
    AV_PKT_DATA_SPHERICAL = 21,
    AV_PKT_DATA_CONTENT_LIGHT_LEVEL = 22,
    AV_PKT_DATA_A53_CC = 23,
    AV_PKT_DATA_ENCRYPTION_INIT_INFO = 24,
    AV_PKT_DATA_ENCRYPTION_INFO = 25,
    AV_PKT_DATA_AFD = 26,
    AV_PKT_DATA_NB = 27
}

/// A reference to a common buffer.
///
/// The size of this struct is not a part of the public ABI and it is not meant
/// to be allocated directly.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    /// The common buffer. It is considered writable if and only if
   /// this is the only reference to the buffer, in which case
   /// av_buffer_is_writable() returns 1.
    pub data: *mut u8,
    /// Size of common in bytes.
    pub size: libc::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVPacketSideData {
    pub data: *mut u8,
    pub size: libc::c_int,
    pub type_: AVPacketSideDataType
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVPacket {
    /// A reference to the reference-counted buffer where the packet common is
   /// stored.
   /// May be NULL, then the packet common is not reference-counted.
    pub buf: *mut AVBufferRef,
    /// Presentation timestamp in AVStream->time_base units; the time at which
   /// the decompressed packet will be presented to the user.
   /// Can be AV_NOPTS_VALUE if it is not stored in the file.
   /// pts MUST be larger or equal to dts as presentation cannot happen before
   /// decompression, unless one wants to view hex dumps. Some formats misuse
   /// the terms dts and pts/cts to mean something different. Such timestamps
   /// must be converted to true pts/dts before they are stored in AVPacket.
    pub pts: i64,
    /// Decompression timestamp in AVStream->time_base units; the time at which
   /// the packet is decompressed.
   /// Can be AV_NOPTS_VALUE if it is not stored in the file.
    pub dts: i64,
    pub data: *mut u8,
    pub size: libc::c_int,
    pub stream_index: libc::c_int,
    /// A combination of AV_PKT_FLAG values
    pub flags: libc::c_int,
    /// Additional packet common that can be provided by the container.
   /// Packet can contain several types of side information.
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: libc::c_int,
    /// Duration of this packet in AVStream->time_base units, 0 if unknown.
   /// Equals next_pts - this_pts in presentation order.
    pub duration: i64,
    /// < byte position in stream, -1 if unknown
    pub pos: i64,
    /// @deprecated Same as the duration field, but as int64_t. This was required
   /// for Matroska subtitles, whose duration values could overflow when the
   /// duration field was still an int.
    pub convergence_duration: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVProbeData {
    pub filename: *const libc::c_char,
    /// < Buffer must have AVPROBE_PADDING_SIZE of extra allocated bytes filled with zero.
    pub buf: *mut libc::c_uchar,
    /// < Size of buf except extra allocated bytes
    pub buf_size: libc::c_int,
    /// < mime_type, when known.
    pub mime_type: *const libc::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecTag {
    _unused: [u8; 0]
}

#[repr(i32)]      /// Audio sample formats
///
/// - The common described by the sample format is always in native-endian order.
/// Sample values can be expressed by native C types, hence the lack of a signed
/// 24-bit sample format even though it is a common raw audio common format.
///
/// - The floating-point formats are based on full volume being in the range
/// [-1.0, 1.0]. Any values outside this range are beyond full volume level.
///
/// - The common layout as used in av_samples_fill_arrays() and elsewhere in FFmpeg
/// (such as AVFrame in libavcodec) is as follows:
///
/// @par
/// For planar sample formats, each audio channel is in a separate common plane,
/// and linesize is the buffer size, in bytes, for a single plane. All common
/// planes must be the same size. For packed sample formats, only the first common
/// plane is used, and samples for each channel are interleaved. In this case,
/// linesize is the buffer size, in bytes, for the 1 plane.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVSampleFormat {
    AV_SAMPLE_FMT_NONE = -1, AV_SAMPLE_FMT_U8 = 0, AV_SAMPLE_FMT_S16 = 1,
    AV_SAMPLE_FMT_S32 = 2, AV_SAMPLE_FMT_FLT = 3, AV_SAMPLE_FMT_DBL = 4,
    AV_SAMPLE_FMT_U8P = 5, AV_SAMPLE_FMT_S16P = 6, AV_SAMPLE_FMT_S32P = 7,
    AV_SAMPLE_FMT_FLTP = 8, AV_SAMPLE_FMT_DBLP = 9, AV_SAMPLE_FMT_S64 = 10,
    AV_SAMPLE_FMT_S64P = 11, AV_SAMPLE_FMT_NB = 12
}

#[repr(i32)]      /// Pixel format.
///
/// @note
/// AV_PIX_FMT_RGB32 is handled in an endian-specific manner. An RGBA
/// color is put together as:
/// (A << 24) | (R << 16) | (G << 8) | B
/// This is stored as BGRA on little-endian CPU architectures and ARGB on
/// big-endian CPUs.
///
/// @note
/// If the resolution is not a multiple of the chroma subsampling factor
/// then the chroma plane resolution must be rounded up.
///
/// @par
/// When the pixel format is palettized RGB32 (AV_PIX_FMT_PAL8), the palettized
/// image common is stored in AVFrame.common[0]. The palette is transported in
/// AVFrame.common[1], is 1024 bytes long (256 4-byte entries) and is
/// formatted the same as in AV_PIX_FMT_RGB32 described above (i.e., it is
/// also endian-specific). Note also that the individual RGB32 palette
/// components stored in AVFrame.common[1] should be in the range 0..255.
/// This is important as many custom PAL8 video codecs that were designed
/// to run on the IBM VGA graphics adapter use 6-bit palette components.
///
/// @par
/// For all the 8 bits per pixel formats, an RGB32 palette is in common[1] like
/// for pal8. This palette is filled in automatically by the function
/// allocating the picture.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVPixelFormat {
    AV_PIX_FMT_NONE = -1, AV_PIX_FMT_YUV420P = 0, AV_PIX_FMT_YUYV422 = 1,
    AV_PIX_FMT_RGB24 = 2, AV_PIX_FMT_BGR24 = 3, AV_PIX_FMT_YUV422P = 4,
    AV_PIX_FMT_YUV444P = 5, AV_PIX_FMT_YUV410P = 6, AV_PIX_FMT_YUV411P = 7,
    AV_PIX_FMT_GRAY8 = 8, AV_PIX_FMT_MONOWHITE = 9, AV_PIX_FMT_MONOBLACK = 10,
    AV_PIX_FMT_PAL8 = 11, AV_PIX_FMT_YUVJ420P = 12, AV_PIX_FMT_YUVJ422P = 13,
    AV_PIX_FMT_YUVJ444P = 14, AV_PIX_FMT_UYVY422 = 15, AV_PIX_FMT_UYYVYY411 = 16,
    AV_PIX_FMT_BGR8 = 17, AV_PIX_FMT_BGR4 = 18, AV_PIX_FMT_BGR4_BYTE = 19,
    AV_PIX_FMT_RGB8 = 20, AV_PIX_FMT_RGB4 = 21, AV_PIX_FMT_RGB4_BYTE = 22,
    AV_PIX_FMT_NV12 = 23, AV_PIX_FMT_NV21 = 24, AV_PIX_FMT_ARGB = 25,
    AV_PIX_FMT_RGBA = 26, AV_PIX_FMT_ABGR = 27, AV_PIX_FMT_BGRA = 28,
    AV_PIX_FMT_GRAY16BE = 29, AV_PIX_FMT_GRAY16LE = 30, AV_PIX_FMT_YUV440P = 31,
    AV_PIX_FMT_YUVJ440P = 32, AV_PIX_FMT_YUVA420P = 33, AV_PIX_FMT_RGB48BE = 34,
    AV_PIX_FMT_RGB48LE = 35, AV_PIX_FMT_RGB565BE = 36, AV_PIX_FMT_RGB565LE = 37,
    AV_PIX_FMT_RGB555BE = 38, AV_PIX_FMT_RGB555LE = 39, AV_PIX_FMT_BGR565BE = 40,
    AV_PIX_FMT_BGR565LE = 41, AV_PIX_FMT_BGR555BE = 42, AV_PIX_FMT_BGR555LE = 43,
    AV_PIX_FMT_VAAPI_MOCO = 44, AV_PIX_FMT_VAAPI_IDCT = 45, AV_PIX_FMT_VAAPI_VLD = 46,
    AV_PIX_FMT_YUV420P16LE = 47, AV_PIX_FMT_YUV420P16BE = 48, AV_PIX_FMT_YUV422P16LE = 49,
    AV_PIX_FMT_YUV422P16BE = 50, AV_PIX_FMT_YUV444P16LE = 51, AV_PIX_FMT_YUV444P16BE = 52,
    AV_PIX_FMT_DXVA2_VLD = 53, AV_PIX_FMT_RGB444LE = 54, AV_PIX_FMT_RGB444BE = 55,
    AV_PIX_FMT_BGR444LE = 56, AV_PIX_FMT_BGR444BE = 57, AV_PIX_FMT_YA8 = 58,
    AV_PIX_FMT_BGR48BE = 59, AV_PIX_FMT_BGR48LE = 60, AV_PIX_FMT_YUV420P9BE = 61,
    AV_PIX_FMT_YUV420P9LE = 62, AV_PIX_FMT_YUV420P10BE = 63, AV_PIX_FMT_YUV420P10LE = 64,
    AV_PIX_FMT_YUV422P10BE = 65, AV_PIX_FMT_YUV422P10LE = 66, AV_PIX_FMT_YUV444P9BE = 67,
    AV_PIX_FMT_YUV444P9LE = 68, AV_PIX_FMT_YUV444P10BE = 69, AV_PIX_FMT_YUV444P10LE = 70,
    AV_PIX_FMT_YUV422P9BE = 71, AV_PIX_FMT_YUV422P9LE = 72, AV_PIX_FMT_GBRP = 73,
    AV_PIX_FMT_GBRP9BE = 74, AV_PIX_FMT_GBRP9LE = 75, AV_PIX_FMT_GBRP10BE = 76,
    AV_PIX_FMT_GBRP10LE = 77, AV_PIX_FMT_GBRP16BE = 78, AV_PIX_FMT_GBRP16LE = 79,
    AV_PIX_FMT_YUVA422P = 80, AV_PIX_FMT_YUVA444P = 81, AV_PIX_FMT_YUVA420P9BE = 82,
    AV_PIX_FMT_YUVA420P9LE = 83, AV_PIX_FMT_YUVA422P9BE = 84, AV_PIX_FMT_YUVA422P9LE = 85,
    AV_PIX_FMT_YUVA444P9BE = 86, AV_PIX_FMT_YUVA444P9LE = 87, AV_PIX_FMT_YUVA420P10BE = 88,
    AV_PIX_FMT_YUVA420P10LE = 89, AV_PIX_FMT_YUVA422P10BE = 90, AV_PIX_FMT_YUVA422P10LE = 91,
    AV_PIX_FMT_YUVA444P10BE = 92, AV_PIX_FMT_YUVA444P10LE = 93, AV_PIX_FMT_YUVA420P16BE = 94,
    AV_PIX_FMT_YUVA420P16LE = 95, AV_PIX_FMT_YUVA422P16BE = 96, AV_PIX_FMT_YUVA422P16LE = 97,
    AV_PIX_FMT_YUVA444P16BE = 98, AV_PIX_FMT_YUVA444P16LE = 99, AV_PIX_FMT_VDPAU = 100,
    AV_PIX_FMT_XYZ12LE = 101, AV_PIX_FMT_XYZ12BE = 102, AV_PIX_FMT_NV16 = 103,
    AV_PIX_FMT_NV20LE = 104, AV_PIX_FMT_NV20BE = 105, AV_PIX_FMT_RGBA64BE = 106,
    AV_PIX_FMT_RGBA64LE = 107, AV_PIX_FMT_BGRA64BE = 108, AV_PIX_FMT_BGRA64LE = 109,
    AV_PIX_FMT_YVYU422 = 110, AV_PIX_FMT_YA16BE = 111, AV_PIX_FMT_YA16LE = 112,
    AV_PIX_FMT_GBRAP = 113, AV_PIX_FMT_GBRAP16BE = 114, AV_PIX_FMT_GBRAP16LE = 115,
    AV_PIX_FMT_QSV = 116, AV_PIX_FMT_MMAL = 117, AV_PIX_FMT_D3D11VA_VLD = 118,
    AV_PIX_FMT_CUDA = 119, AV_PIX_FMT_0RGB = 120, AV_PIX_FMT_RGB0 = 121,
    AV_PIX_FMT_0BGR = 122, AV_PIX_FMT_BGR0 = 123, AV_PIX_FMT_YUV420P12BE = 124,
    AV_PIX_FMT_YUV420P12LE = 125, AV_PIX_FMT_YUV420P14BE = 126, AV_PIX_FMT_YUV420P14LE = 127,
    AV_PIX_FMT_YUV422P12BE = 128, AV_PIX_FMT_YUV422P12LE = 129, AV_PIX_FMT_YUV422P14BE = 130,
    AV_PIX_FMT_YUV422P14LE = 131, AV_PIX_FMT_YUV444P12BE = 132, AV_PIX_FMT_YUV444P12LE = 133,
    AV_PIX_FMT_YUV444P14BE = 134, AV_PIX_FMT_YUV444P14LE = 135, AV_PIX_FMT_GBRP12BE = 136,
    AV_PIX_FMT_GBRP12LE = 137, AV_PIX_FMT_GBRP14BE = 138, AV_PIX_FMT_GBRP14LE = 139,
    AV_PIX_FMT_YUVJ411P = 140, AV_PIX_FMT_BAYER_BGGR8 = 141, AV_PIX_FMT_BAYER_RGGB8 = 142,
    AV_PIX_FMT_BAYER_GBRG8 = 143, AV_PIX_FMT_BAYER_GRBG8 = 144, AV_PIX_FMT_BAYER_BGGR16LE = 145,
    AV_PIX_FMT_BAYER_BGGR16BE = 146, AV_PIX_FMT_BAYER_RGGB16LE = 147, AV_PIX_FMT_BAYER_RGGB16BE = 148,
    AV_PIX_FMT_BAYER_GBRG16LE = 149, AV_PIX_FMT_BAYER_GBRG16BE = 150, AV_PIX_FMT_BAYER_GRBG16LE = 151,
    AV_PIX_FMT_BAYER_GRBG16BE = 152, AV_PIX_FMT_XVMC = 153, AV_PIX_FMT_YUV440P10LE = 154,
    AV_PIX_FMT_YUV440P10BE = 155, AV_PIX_FMT_YUV440P12LE = 156, AV_PIX_FMT_YUV440P12BE = 157,
    AV_PIX_FMT_AYUV64LE = 158, AV_PIX_FMT_AYUV64BE = 159, AV_PIX_FMT_VIDEOTOOLBOX = 160,
    AV_PIX_FMT_P010LE = 161, AV_PIX_FMT_P010BE = 162, AV_PIX_FMT_GBRAP12BE = 163,
    AV_PIX_FMT_GBRAP12LE = 164, AV_PIX_FMT_GBRAP10BE = 165, AV_PIX_FMT_GBRAP10LE = 166,
    AV_PIX_FMT_MEDIACODEC = 167, AV_PIX_FMT_GRAY12BE = 168, AV_PIX_FMT_GRAY12LE = 169,
    AV_PIX_FMT_GRAY10BE = 170, AV_PIX_FMT_GRAY10LE = 171, AV_PIX_FMT_P016LE = 172,
    AV_PIX_FMT_P016BE = 173, AV_PIX_FMT_D3D11 = 174, AV_PIX_FMT_GRAY9BE = 175,
    AV_PIX_FMT_GRAY9LE = 176, AV_PIX_FMT_GBRPF32BE = 177, AV_PIX_FMT_GBRPF32LE = 178,
    AV_PIX_FMT_GBRAPF32BE = 179, AV_PIX_FMT_GBRAPF32LE = 180, AV_PIX_FMT_DRM_PRIME = 181,
    AV_PIX_FMT_OPENCL = 182, AV_PIX_FMT_GRAY14BE = 183, AV_PIX_FMT_GRAY14LE = 184,
    AV_PIX_FMT_GRAYF32BE = 185, AV_PIX_FMT_GRAYF32LE = 186, AV_PIX_FMT_NB = 187
}

/// Structure describes device capabilities.
///
/// It is used by devices in conjunction with av_device_capabilities AVOption table
/// to implement capabilities probing API based on AVOption API. Should not be used directly.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVDeviceCapabilitiesQuery {
    pub av_class: *const AVClass,
    pub device_context: *mut AVFormatContext,
    pub codec: AVCodecID,
    pub sample_format: AVSampleFormat,
    pub pixel_format: AVPixelFormat,
    pub sample_rate: libc::c_int,
    pub channels: libc::c_int,
    pub channel_layout: i64,
    pub window_width: libc::c_int,
    pub window_height: libc::c_int,
    pub frame_width: libc::c_int,
    pub frame_height: libc::c_int,
    pub fps: AVRational
}

#[repr(u32)]      /// Identify the syntax and semantics of the bitstream.
/// The principle is roughly:
/// Two decoders with the same ID can decode the same streams.
/// Two encoders with the same ID can encode compatible streams.
/// There may be slight deviations from the principle due to implementation
/// details.
///
/// If you add a codec ID to this list, add it so that
/// 1. no value of an existing codec ID changes (that would break ABI),
/// 2. it is as close as possible to similar codecs
///
/// After adding new codec IDs, do not forget to add an entry to the codec
/// descriptor list and bump libavcodec minor version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVCodecID {
    AV_CODEC_ID_NONE = 0, AV_CODEC_ID_MPEG1VIDEO = 1, AV_CODEC_ID_MPEG2VIDEO = 2, AV_CODEC_ID_H261 = 3,
    AV_CODEC_ID_H263 = 4, AV_CODEC_ID_RV10 = 5, AV_CODEC_ID_RV20 = 6, AV_CODEC_ID_MJPEG = 7,
    AV_CODEC_ID_MJPEGB = 8, AV_CODEC_ID_LJPEG = 9, AV_CODEC_ID_SP5X = 10, AV_CODEC_ID_JPEGLS = 11,
    AV_CODEC_ID_MPEG4 = 12, AV_CODEC_ID_RAWVIDEO = 13, AV_CODEC_ID_MSMPEG4V1 = 14, AV_CODEC_ID_MSMPEG4V2 = 15,
    AV_CODEC_ID_MSMPEG4V3 = 16, AV_CODEC_ID_WMV1 = 17, AV_CODEC_ID_WMV2 = 18, AV_CODEC_ID_H263P = 19,
    AV_CODEC_ID_H263I = 20, AV_CODEC_ID_FLV1 = 21, AV_CODEC_ID_SVQ1 = 22, AV_CODEC_ID_SVQ3 = 23,
    AV_CODEC_ID_DVVIDEO = 24, AV_CODEC_ID_HUFFYUV = 25, AV_CODEC_ID_CYUV = 26, AV_CODEC_ID_H264 = 27,
    AV_CODEC_ID_INDEO3 = 28, AV_CODEC_ID_VP3 = 29, AV_CODEC_ID_THEORA = 30, AV_CODEC_ID_ASV1 = 31,
    AV_CODEC_ID_ASV2 = 32, AV_CODEC_ID_FFV1 = 33, AV_CODEC_ID_4XM = 34, AV_CODEC_ID_VCR1 = 35,
    AV_CODEC_ID_CLJR = 36, AV_CODEC_ID_MDEC = 37, AV_CODEC_ID_ROQ = 38, AV_CODEC_ID_INTERPLAY_VIDEO = 39,
    AV_CODEC_ID_XAN_WC3 = 40, AV_CODEC_ID_XAN_WC4 = 41, AV_CODEC_ID_RPZA = 42, AV_CODEC_ID_CINEPAK = 43,
    AV_CODEC_ID_WS_VQA = 44, AV_CODEC_ID_MSRLE = 45, AV_CODEC_ID_MSVIDEO1 = 46, AV_CODEC_ID_IDCIN = 47,
    AV_CODEC_ID_8BPS = 48, AV_CODEC_ID_SMC = 49, AV_CODEC_ID_FLIC = 50, AV_CODEC_ID_TRUEMOTION1 = 51,
    AV_CODEC_ID_VMDVIDEO = 52, AV_CODEC_ID_MSZH = 53, AV_CODEC_ID_ZLIB = 54, AV_CODEC_ID_QTRLE = 55,
    AV_CODEC_ID_TSCC = 56, AV_CODEC_ID_ULTI = 57, AV_CODEC_ID_QDRAW = 58, AV_CODEC_ID_VIXL = 59,
    AV_CODEC_ID_QPEG = 60, AV_CODEC_ID_PNG = 61, AV_CODEC_ID_PPM = 62, AV_CODEC_ID_PBM = 63,
    AV_CODEC_ID_PGM = 64, AV_CODEC_ID_PGMYUV = 65, AV_CODEC_ID_PAM = 66, AV_CODEC_ID_FFVHUFF = 67,
    AV_CODEC_ID_RV30 = 68, AV_CODEC_ID_RV40 = 69, AV_CODEC_ID_VC1 = 70, AV_CODEC_ID_WMV3 = 71,
    AV_CODEC_ID_LOCO = 72, AV_CODEC_ID_WNV1 = 73, AV_CODEC_ID_AASC = 74, AV_CODEC_ID_INDEO2 = 75,
    AV_CODEC_ID_FRAPS = 76, AV_CODEC_ID_TRUEMOTION2 = 77, AV_CODEC_ID_BMP = 78, AV_CODEC_ID_CSCD = 79,
    AV_CODEC_ID_MMVIDEO = 80, AV_CODEC_ID_ZMBV = 81, AV_CODEC_ID_AVS = 82, AV_CODEC_ID_SMACKVIDEO = 83,
    AV_CODEC_ID_NUV = 84, AV_CODEC_ID_KMVC = 85, AV_CODEC_ID_FLASHSV = 86, AV_CODEC_ID_CAVS = 87,
    AV_CODEC_ID_JPEG2000 = 88, AV_CODEC_ID_VMNC = 89, AV_CODEC_ID_VP5 = 90, AV_CODEC_ID_VP6 = 91,
    AV_CODEC_ID_VP6F = 92, AV_CODEC_ID_TARGA = 93, AV_CODEC_ID_DSICINVIDEO = 94, AV_CODEC_ID_TIERTEXSEQVIDEO = 95,
    AV_CODEC_ID_TIFF = 96, AV_CODEC_ID_GIF = 97, AV_CODEC_ID_DXA = 98, AV_CODEC_ID_DNXHD = 99,
    AV_CODEC_ID_THP = 100, AV_CODEC_ID_SGI = 101, AV_CODEC_ID_C93 = 102, AV_CODEC_ID_BETHSOFTVID = 103,
    AV_CODEC_ID_PTX = 104, AV_CODEC_ID_TXD = 105, AV_CODEC_ID_VP6A = 106, AV_CODEC_ID_AMV = 107,
    AV_CODEC_ID_VB = 108, AV_CODEC_ID_PCX = 109, AV_CODEC_ID_SUNRAST = 110, AV_CODEC_ID_INDEO4 = 111,
    AV_CODEC_ID_INDEO5 = 112, AV_CODEC_ID_MIMIC = 113, AV_CODEC_ID_RL2 = 114, AV_CODEC_ID_ESCAPE124 = 115,
    AV_CODEC_ID_DIRAC = 116, AV_CODEC_ID_BFI = 117, AV_CODEC_ID_CMV = 118, AV_CODEC_ID_MOTIONPIXELS = 119,
    AV_CODEC_ID_TGV = 120, AV_CODEC_ID_TGQ = 121, AV_CODEC_ID_TQI = 122, AV_CODEC_ID_AURA = 123,
    AV_CODEC_ID_AURA2 = 124, AV_CODEC_ID_V210X = 125, AV_CODEC_ID_TMV = 126, AV_CODEC_ID_V210 = 127,
    AV_CODEC_ID_DPX = 128, AV_CODEC_ID_MAD = 129, AV_CODEC_ID_FRWU = 130, AV_CODEC_ID_FLASHSV2 = 131,
    AV_CODEC_ID_CDGRAPHICS = 132, AV_CODEC_ID_R210 = 133, AV_CODEC_ID_ANM = 134, AV_CODEC_ID_BINKVIDEO = 135,
    AV_CODEC_ID_IFF_ILBM = 136, AV_CODEC_ID_KGV1 = 137, AV_CODEC_ID_YOP = 138, AV_CODEC_ID_VP8 = 139,
    AV_CODEC_ID_PICTOR = 140, AV_CODEC_ID_ANSI = 141, AV_CODEC_ID_A64_MULTI = 142, AV_CODEC_ID_A64_MULTI5 = 143,
    AV_CODEC_ID_R10K = 144, AV_CODEC_ID_MXPEG = 145, AV_CODEC_ID_LAGARITH = 146, AV_CODEC_ID_PRORES = 147,
    AV_CODEC_ID_JV = 148, AV_CODEC_ID_DFA = 149, AV_CODEC_ID_WMV3IMAGE = 150, AV_CODEC_ID_VC1IMAGE = 151,
    AV_CODEC_ID_UTVIDEO = 152, AV_CODEC_ID_BMV_VIDEO = 153, AV_CODEC_ID_VBLE = 154, AV_CODEC_ID_DXTORY = 155,
    AV_CODEC_ID_V410 = 156, AV_CODEC_ID_XWD = 157, AV_CODEC_ID_CDXL = 158, AV_CODEC_ID_XBM = 159,
    AV_CODEC_ID_ZEROCODEC = 160, AV_CODEC_ID_MSS1 = 161, AV_CODEC_ID_MSA1 = 162, AV_CODEC_ID_TSCC2 = 163,
    AV_CODEC_ID_MTS2 = 164, AV_CODEC_ID_CLLC = 165, AV_CODEC_ID_MSS2 = 166, AV_CODEC_ID_VP9 = 167,
    AV_CODEC_ID_AIC = 168, AV_CODEC_ID_ESCAPE130 = 169, AV_CODEC_ID_G2M = 170, AV_CODEC_ID_WEBP = 171,
    AV_CODEC_ID_HNM4_VIDEO = 172, AV_CODEC_ID_HEVC = 173, AV_CODEC_ID_FIC = 174, AV_CODEC_ID_ALIAS_PIX = 175,
    AV_CODEC_ID_BRENDER_PIX = 176, AV_CODEC_ID_PAF_VIDEO = 177, AV_CODEC_ID_EXR = 178, AV_CODEC_ID_VP7 = 179,
    AV_CODEC_ID_SANM = 180, AV_CODEC_ID_SGIRLE = 181, AV_CODEC_ID_MVC1 = 182, AV_CODEC_ID_MVC2 = 183,
    AV_CODEC_ID_HQX = 184, AV_CODEC_ID_TDSC = 185, AV_CODEC_ID_HQ_HQA = 186, AV_CODEC_ID_HAP = 187,
    AV_CODEC_ID_DDS = 188, AV_CODEC_ID_DXV = 189, AV_CODEC_ID_SCREENPRESSO = 190, AV_CODEC_ID_RSCC = 191,
    AV_CODEC_ID_AVS2 = 192, AV_CODEC_ID_Y41P = 32768, AV_CODEC_ID_AVRP = 32769, AV_CODEC_ID_012V = 32770,
    AV_CODEC_ID_AVUI = 32771, AV_CODEC_ID_AYUV = 32772, AV_CODEC_ID_TARGA_Y216 = 32773, AV_CODEC_ID_V308 = 32774,
    AV_CODEC_ID_V408 = 32775, AV_CODEC_ID_YUV4 = 32776, AV_CODEC_ID_AVRN = 32777, AV_CODEC_ID_CPIA = 32778,
    AV_CODEC_ID_XFACE = 32779, AV_CODEC_ID_SNOW = 32780, AV_CODEC_ID_SMVJPEG = 32781, AV_CODEC_ID_APNG = 32782,
    AV_CODEC_ID_DAALA = 32783, AV_CODEC_ID_CFHD = 32784, AV_CODEC_ID_TRUEMOTION2RT = 32785, AV_CODEC_ID_M101 = 32786,
    AV_CODEC_ID_MAGICYUV = 32787, AV_CODEC_ID_SHEERVIDEO = 32788, AV_CODEC_ID_YLC = 32789, AV_CODEC_ID_PSD = 32790,
    AV_CODEC_ID_PIXLET = 32791, AV_CODEC_ID_SPEEDHQ = 32792, AV_CODEC_ID_FMVC = 32793, AV_CODEC_ID_SCPR = 32794,
    AV_CODEC_ID_CLEARVIDEO = 32795, AV_CODEC_ID_XPM = 32796, AV_CODEC_ID_AV1 = 32797, AV_CODEC_ID_BITPACKED = 32798,
    AV_CODEC_ID_MSCC = 32799, AV_CODEC_ID_SRGC = 32800, AV_CODEC_ID_SVG = 32801, AV_CODEC_ID_GDV = 32802,
    AV_CODEC_ID_FITS = 32803, AV_CODEC_ID_IMM4 = 32804, AV_CODEC_ID_PROSUMER = 32805, AV_CODEC_ID_MWSC = 32806,
    AV_CODEC_ID_WCMV = 32807, AV_CODEC_ID_RASC = 32808, AV_CODEC_ID_FIRST_AUDIO = 65536, AV_CODEC_ID_PCM_S16BE = 65537,
    AV_CODEC_ID_PCM_U16LE = 65538, AV_CODEC_ID_PCM_U16BE = 65539, AV_CODEC_ID_PCM_S8 = 65540, AV_CODEC_ID_PCM_U8 = 65541,
    AV_CODEC_ID_PCM_MULAW = 65542, AV_CODEC_ID_PCM_ALAW = 65543, AV_CODEC_ID_PCM_S32LE = 65544, AV_CODEC_ID_PCM_S32BE = 65545,
    AV_CODEC_ID_PCM_U32LE = 65546, AV_CODEC_ID_PCM_U32BE = 65547, AV_CODEC_ID_PCM_S24LE = 65548, AV_CODEC_ID_PCM_S24BE = 65549,
    AV_CODEC_ID_PCM_U24LE = 65550, AV_CODEC_ID_PCM_U24BE = 65551, AV_CODEC_ID_PCM_S24DAUD = 65552, AV_CODEC_ID_PCM_ZORK = 65553,
    AV_CODEC_ID_PCM_S16LE_PLANAR = 65554, AV_CODEC_ID_PCM_DVD = 65555, AV_CODEC_ID_PCM_F32BE = 65556, AV_CODEC_ID_PCM_F32LE = 65557,
    AV_CODEC_ID_PCM_F64BE = 65558, AV_CODEC_ID_PCM_F64LE = 65559, AV_CODEC_ID_PCM_BLURAY = 65560, AV_CODEC_ID_PCM_LXF = 65561,
    AV_CODEC_ID_S302M = 65562, AV_CODEC_ID_PCM_S8_PLANAR = 65563, AV_CODEC_ID_PCM_S24LE_PLANAR = 65564, AV_CODEC_ID_PCM_S32LE_PLANAR = 65565,
    AV_CODEC_ID_PCM_S16BE_PLANAR = 65566, AV_CODEC_ID_PCM_S64LE = 67584, AV_CODEC_ID_PCM_S64BE = 67585, AV_CODEC_ID_PCM_F16LE = 67586,
    AV_CODEC_ID_PCM_F24LE = 67587, AV_CODEC_ID_PCM_VIDC = 67588, AV_CODEC_ID_ADPCM_IMA_QT = 69632, AV_CODEC_ID_ADPCM_IMA_WAV = 69633,
    AV_CODEC_ID_ADPCM_IMA_DK3 = 69634, AV_CODEC_ID_ADPCM_IMA_DK4 = 69635, AV_CODEC_ID_ADPCM_IMA_WS = 69636, AV_CODEC_ID_ADPCM_IMA_SMJPEG = 69637,
    AV_CODEC_ID_ADPCM_MS = 69638, AV_CODEC_ID_ADPCM_4XM = 69639, AV_CODEC_ID_ADPCM_XA = 69640, AV_CODEC_ID_ADPCM_ADX = 69641,
    AV_CODEC_ID_ADPCM_EA = 69642, AV_CODEC_ID_ADPCM_G726 = 69643, AV_CODEC_ID_ADPCM_CT = 69644, AV_CODEC_ID_ADPCM_SWF = 69645,
    AV_CODEC_ID_ADPCM_YAMAHA = 69646, AV_CODEC_ID_ADPCM_SBPRO_4 = 69647, AV_CODEC_ID_ADPCM_SBPRO_3 = 69648, AV_CODEC_ID_ADPCM_SBPRO_2 = 69649,
    AV_CODEC_ID_ADPCM_THP = 69650, AV_CODEC_ID_ADPCM_IMA_AMV = 69651, AV_CODEC_ID_ADPCM_EA_R1 = 69652, AV_CODEC_ID_ADPCM_EA_R3 = 69653,
    AV_CODEC_ID_ADPCM_EA_R2 = 69654, AV_CODEC_ID_ADPCM_IMA_EA_SEAD = 69655, AV_CODEC_ID_ADPCM_IMA_EA_EACS = 69656, AV_CODEC_ID_ADPCM_EA_XAS = 69657,
    AV_CODEC_ID_ADPCM_EA_MAXIS_XA = 69658, AV_CODEC_ID_ADPCM_IMA_ISS = 69659, AV_CODEC_ID_ADPCM_G722 = 69660, AV_CODEC_ID_ADPCM_IMA_APC = 69661,
    AV_CODEC_ID_ADPCM_VIMA = 69662, AV_CODEC_ID_ADPCM_AFC = 71680, AV_CODEC_ID_ADPCM_IMA_OKI = 71681, AV_CODEC_ID_ADPCM_DTK = 71682,
    AV_CODEC_ID_ADPCM_IMA_RAD = 71683, AV_CODEC_ID_ADPCM_G726LE = 71684, AV_CODEC_ID_ADPCM_THP_LE = 71685, AV_CODEC_ID_ADPCM_PSX = 71686,
    AV_CODEC_ID_ADPCM_AICA = 71687, AV_CODEC_ID_ADPCM_IMA_DAT4 = 71688, AV_CODEC_ID_ADPCM_MTAF = 71689, AV_CODEC_ID_AMR_NB = 73728,
    AV_CODEC_ID_AMR_WB = 73729, AV_CODEC_ID_RA_144 = 77824, AV_CODEC_ID_RA_288 = 77825, AV_CODEC_ID_ROQ_DPCM = 81920,
    AV_CODEC_ID_INTERPLAY_DPCM = 81921, AV_CODEC_ID_XAN_DPCM = 81922, AV_CODEC_ID_SOL_DPCM = 81923, AV_CODEC_ID_SDX2_DPCM = 83968,
    AV_CODEC_ID_GREMLIN_DPCM = 83969, AV_CODEC_ID_MP2 = 86016, AV_CODEC_ID_MP3 = 86017, AV_CODEC_ID_AAC = 86018,
    AV_CODEC_ID_AC3 = 86019, AV_CODEC_ID_DTS = 86020, AV_CODEC_ID_VORBIS = 86021, AV_CODEC_ID_DVAUDIO = 86022, AV_CODEC_ID_WMAV1 = 86023,
    AV_CODEC_ID_WMAV2 = 86024, AV_CODEC_ID_MACE3 = 86025, AV_CODEC_ID_MACE6 = 86026, AV_CODEC_ID_VMDAUDIO = 86027,
    AV_CODEC_ID_FLAC = 86028, AV_CODEC_ID_MP3ADU = 86029, AV_CODEC_ID_MP3ON4 = 86030, AV_CODEC_ID_SHORTEN = 86031,
    AV_CODEC_ID_ALAC = 86032, AV_CODEC_ID_WESTWOOD_SND1 = 86033, AV_CODEC_ID_GSM = 86034, AV_CODEC_ID_QDM2 = 86035,
    AV_CODEC_ID_COOK = 86036, AV_CODEC_ID_TRUESPEECH = 86037, AV_CODEC_ID_TTA = 86038, AV_CODEC_ID_SMACKAUDIO = 86039,
    AV_CODEC_ID_QCELP = 86040, AV_CODEC_ID_WAVPACK = 86041, AV_CODEC_ID_DSICINAUDIO = 86042, AV_CODEC_ID_IMC = 86043,
    AV_CODEC_ID_MUSEPACK7 = 86044, AV_CODEC_ID_MLP = 86045, AV_CODEC_ID_GSM_MS = 86046, AV_CODEC_ID_ATRAC3 = 86047,
    AV_CODEC_ID_APE = 86048, AV_CODEC_ID_NELLYMOSER = 86049, AV_CODEC_ID_MUSEPACK8 = 86050, AV_CODEC_ID_SPEEX = 86051,
    AV_CODEC_ID_WMAVOICE = 86052, AV_CODEC_ID_WMAPRO = 86053, AV_CODEC_ID_WMALOSSLESS = 86054, AV_CODEC_ID_ATRAC3P = 86055,
    AV_CODEC_ID_EAC3 = 86056, AV_CODEC_ID_SIPR = 86057, AV_CODEC_ID_MP1 = 86058, AV_CODEC_ID_TWINVQ = 86059,
    AV_CODEC_ID_TRUEHD = 86060, AV_CODEC_ID_MP4ALS = 86061, AV_CODEC_ID_ATRAC1 = 86062, AV_CODEC_ID_BINKAUDIO_RDFT = 86063,
    AV_CODEC_ID_BINKAUDIO_DCT = 86064, AV_CODEC_ID_AAC_LATM = 86065, AV_CODEC_ID_QDMC = 86066, AV_CODEC_ID_CELT = 86067,
    AV_CODEC_ID_G723_1 = 86068, AV_CODEC_ID_G729 = 86069, AV_CODEC_ID_8SVX_EXP = 86070, AV_CODEC_ID_8SVX_FIB = 86071,
    AV_CODEC_ID_BMV_AUDIO = 86072, AV_CODEC_ID_RALF = 86073, AV_CODEC_ID_IAC = 86074, AV_CODEC_ID_ILBC = 86075,
    AV_CODEC_ID_OPUS = 86076, AV_CODEC_ID_COMFORT_NOISE = 86077, AV_CODEC_ID_TAK = 86078, AV_CODEC_ID_METASOUND = 86079,
    AV_CODEC_ID_PAF_AUDIO = 86080, AV_CODEC_ID_ON2AVC = 86081, AV_CODEC_ID_DSS_SP = 86082, AV_CODEC_ID_CODEC2 = 86083,
    AV_CODEC_ID_FFWAVESYNTH = 88064, AV_CODEC_ID_SONIC = 88065, AV_CODEC_ID_SONIC_LS = 88066, AV_CODEC_ID_EVRC = 88067,
    AV_CODEC_ID_SMV = 88068, AV_CODEC_ID_DSD_LSBF = 88069, AV_CODEC_ID_DSD_MSBF = 88070, AV_CODEC_ID_DSD_LSBF_PLANAR = 88071,
    AV_CODEC_ID_DSD_MSBF_PLANAR = 88072, AV_CODEC_ID_4GV = 88073, AV_CODEC_ID_INTERPLAY_ACM = 88074, AV_CODEC_ID_XMA1 = 88075,
    AV_CODEC_ID_XMA2 = 88076, AV_CODEC_ID_DST = 88077, AV_CODEC_ID_ATRAC3AL = 88078, AV_CODEC_ID_ATRAC3PAL = 88079,
    AV_CODEC_ID_DOLBY_E = 88080, AV_CODEC_ID_APTX = 88081, AV_CODEC_ID_APTX_HD = 88082, AV_CODEC_ID_SBC = 88083,
    AV_CODEC_ID_ATRAC9 = 88084, AV_CODEC_ID_FIRST_SUBTITLE = 94208, AV_CODEC_ID_DVB_SUBTITLE = 94209, AV_CODEC_ID_TEXT = 94210,
    AV_CODEC_ID_XSUB = 94211, AV_CODEC_ID_SSA = 94212, AV_CODEC_ID_MOV_TEXT = 94213, AV_CODEC_ID_HDMV_PGS_SUBTITLE = 94214,
    AV_CODEC_ID_DVB_TELETEXT = 94215, AV_CODEC_ID_SRT = 94216, AV_CODEC_ID_MICRODVD = 96256, AV_CODEC_ID_EIA_608 = 96257,
    AV_CODEC_ID_JACOSUB = 96258, AV_CODEC_ID_SAMI = 96259, AV_CODEC_ID_REALTEXT = 96260, AV_CODEC_ID_STL = 96261,
    AV_CODEC_ID_SUBVIEWER1 = 96262, AV_CODEC_ID_SUBVIEWER = 96263, AV_CODEC_ID_SUBRIP = 96264, AV_CODEC_ID_WEBVTT = 96265,
    AV_CODEC_ID_MPL2 = 96266, AV_CODEC_ID_VPLAYER = 96267, AV_CODEC_ID_PJS = 96268, AV_CODEC_ID_ASS = 96269,
    AV_CODEC_ID_HDMV_TEXT_SUBTITLE = 96270, AV_CODEC_ID_TTML = 96271, AV_CODEC_ID_FIRST_UNKNOWN = 98304, AV_CODEC_ID_SCTE_35 = 98305,
    AV_CODEC_ID_BINTEXT = 100352, AV_CODEC_ID_XBIN = 100353, AV_CODEC_ID_IDF = 100354, AV_CODEC_ID_OTF = 100355,
    AV_CODEC_ID_SMPTE_KLV = 100356, AV_CODEC_ID_DVD_NAV = 100357, AV_CODEC_ID_TIMED_ID3 = 100358, AV_CODEC_ID_BIN_DATA = 100359,
    AV_CODEC_ID_PROBE = 102400, AV_CODEC_ID_MPEG2TS = 131072, AV_CODEC_ID_MPEG4SYSTEMS = 131073, AV_CODEC_ID_FFMETADATA = 135168,
    AV_CODEC_ID_WRAPPED_AVFRAME = 135169 }

/// @addtogroup lavf_decoding
/// @{
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVInputFormat {
    /// A comma separated list of short names for the format. New names
   /// may be appended with a minor bump.
    pub name: *const libc::c_char,
    /// Descriptive name for the format, meant to be more human-readable
   /// than name. You should use the NULL_IF_CONFIG_SMALL() macro
   /// to define it.
    pub long_name: *const libc::c_char,
    /// Can use flags: AVFMT_NOFILE, AVFMT_NEEDNUMBER, AVFMT_SHOW_IDS,
   /// AVFMT_GENERIC_INDEX, AVFMT_TS_DISCONT, AVFMT_NOBINSEARCH,
   /// AVFMT_NOGENSEARCH, AVFMT_NO_BYTE_SEEK, AVFMT_SEEK_TO_PTS.
    pub flags: libc::c_int,
    /// If extensions are defined, then no probe is done. You should
   /// usually not use extension format guessing because it is not
   /// reliable enough
    pub extensions: *const libc::c_char,
    pub codec_tag: *const *const AVCodecTag,
    /// < AVClass for the private context
    pub priv_class: *const AVClass,
    /// Comma-separated list of mime types.
   /// It is used check for matching mime types while probing.
   /// @see av_probe_input_format2
    pub mime_type: *const libc::c_char,
    /// No fields below this line are part of the public API. They
   /// may not be used outside of libavformat and can be changed and
   /// removed at will.
   /// New public fields should be added right above.
   ///
    pub next: *mut AVInputFormat,
    /// Raw demuxers store their codec ID here.
    pub raw_codec_id: libc::c_int,
    /// Size of private common so that it can be allocated in the wrapper.
    pub priv_data_size: libc::c_int,
    /// Tell if a given file has a chance of being parsed as this format.
   /// The buffer provided is guaranteed to be AVPROBE_PADDING_SIZE bytes
   /// big so you do not have to check for that unless you need more.
    pub read_probe: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVProbeData) -> libc::c_int>,
    /// Read the format header and initialize the AVFormatContext
   /// structure. Return 0 if OK. 'avformat_new_stream' should be
   /// called to create new streams.
    pub read_header: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext) -> libc::c_int>,
    /// Read one packet and put it in 'pkt'. pts and flags are also
   /// set. 'avformat_new_stream' can be called only if the flag
   /// AVFMTCTX_NOHEADER is used and only in the calling thread (not in a
   /// background thread).
   /// @return 0 on success, < 0 on error.
   /// When returning an error, pkt must not have been allocated
   /// or must be freed before returning
    pub read_packet: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext, pkt: *mut AVPacket) -> libc::c_int>,
    /// Close the stream. The AVFormatContext and AVStreams are not
   /// freed by this function
    pub read_close: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext) -> libc::c_int>,
    /// Seek to a given timestamp relative to the frames in
   /// stream component stream_index.
   /// @param stream_index Must not be -1.
   /// @param flags Selects which direction should be preferred if no exact
   /// match is available.
   /// @return >= 0 on success (but not necessarily the new offset)
    pub read_seek: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext, stream_index: libc::c_int, timestamp: i64, flags: libc::c_int) -> libc::c_int>,
    /// Get the next timestamp in stream[stream_index].time_base units.
   /// @return the timestamp or AV_NOPTS_VALUE if an error occurred
    pub read_timestamp: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, stream_index: libc::c_int, pos: *mut i64, pos_limit: i64) -> i64>,
    /// Start/resume playing - only meaningful if using a network-based format
   /// (RTSP).
    pub read_play: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext) -> libc::c_int>,
    /// Pause playing - only meaningful if using a network-based format
   /// (RTSP).
    pub read_pause: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext) -> libc::c_int>,
    /// Seek to timestamp ts.
   /// Seeking will be done so that the point from which all active streams
   /// can be presented successfully will be closest to ts and within min/max_ts.
   /// Active streams are all streams that have AVStream.discard < AVDISCARD_ALL.
    pub read_seek2: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, stream_index: libc::c_int, min_ts: i64, ts: i64, max_ts: i64, flags: libc::c_int) -> libc::c_int>,
    /// Returns device list with it properties.
   /// @see avdevice_list_devices() for more details.
    pub get_device_list: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, device_list: *mut AVDeviceInfoList) -> libc::c_int>,
    /// Initialize device capabilities submodule.
   /// @see avdevice_capabilities_create() for more details.
    pub create_device_capabilities: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, caps: *mut AVDeviceCapabilitiesQuery) -> libc::c_int>,
    /// Free device capabilities submodule.
   /// @see avdevice_capabilities_free() for more details.
    pub free_device_capabilities: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, caps: *mut AVDeviceCapabilitiesQuery) -> libc::c_int>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVRational {
    /// < Numerator
    pub num: libc::c_int,
    /// < Denominator
    pub den: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union AVOption__bindgen_ty_1 {
    pub i64: i64,
    pub dbl: f64,
    pub str: *const libc::c_char,
    pub q: AVRational,
    _bindgen_union_align: u64
}

#[repr(u32)]      /// @defgroup avoptions AVOptions
/// @ingroup lavu_data
/// @{
/// AVOptions provide a generic system to declare options on arbitrary structs
/// ("objects"). An option can have a help text, a type and a range of possible
/// values. Options may then be enumerated, read and written to.
///
/// @section avoptions_implement Implementing AVOptions
/// This section describes how to add AVOptions capabilities to a struct.
///
/// All AVOptions-related information is stored in an AVClass. Therefore
/// the first member of the struct should be a pointer to an AVClass describing it.
/// The option field of the AVClass must be set to a NULL-terminated static array
/// of AVOptions. Each AVOption must have a non-empty name, a type, a default
/// value and for number-type AVOptions also a range of allowed values. It must
/// also declare an offset in bytes from the start of the struct, where the field
/// associated with this AVOption is located. Other fields in the AVOption struct
/// should also be set when applicable, but are not required.
///
/// The following example illustrates an AVOptions-enabled struct:
/// @code
/// typedef struct test_struct {
/// const AVClass *class;
/// int      int_opt;
/// char    *str_opt;
/// uint8_t *bin_opt;
/// int      bin_len;
/// } test_struct;
///
/// static const AVOption test_options[] = {
/// { "test_int", "This is a test option of int type.", offsetof(test_struct, int_opt),
/// AV_OPT_TYPE_INT, { .i64 = -1 }, INT_MIN, INT_MAX },
/// { "test_str", "This is a test option of string type.", offsetof(test_struct, str_opt),
/// AV_OPT_TYPE_STRING },
/// { "test_bin", "This is a test option of binary type.", offsetof(test_struct, bin_opt),
/// AV_OPT_TYPE_BINARY },
/// { NULL },
/// };
///
/// static const AVClass test_class = {
/// .class_name = "test class",
/// .item_name  = av_default_item_name,
/// .option     = test_options,
/// .version    = LIBAVUTIL_VERSION_INT,
/// };
/// @endcode
///
/// Next, when allocating your struct, you must ensure that the AVClass pointer
/// is set to the correct value. Then, av_opt_set_defaults() can be called to
/// initialize defaults. After that the struct is ready to be used with the
/// AVOptions API.
///
/// When cleaning up, you may use the av_opt_free() function to automatically
/// free all the allocated string and binary options.
///
/// Continuing with the above example:
///
/// @code
/// test_struct *alloc_test_struct(void)
/// {
/// test_struct *ret = av_mallocz(sizeof(*ret));
/// ret->class = &test_class;
/// av_opt_set_defaults(ret);
/// return ret;
/// }
/// void free_test_struct(test_struct **foo)
/// {
/// av_opt_free(*foo);
/// av_freep(foo);
/// }
/// @endcode
///
/// @subsection avoptions_implement_nesting Nesting
/// It may happen that an AVOptions-enabled struct contains another
/// AVOptions-enabled struct as a member (e.g. AVCodecContext in
/// libavcodec exports generic options, while its priv_data field exports
/// codec-specific options). In such a case, it is possible to set up the
/// parent struct to export a child's options. To do that, simply
/// implement AVClass.child_next() and AVClass.child_class_next() in the
/// parent struct's AVClass.
/// Assuming that the test_struct from above now also contains a
/// child_struct field:
///
/// @code
/// typedef struct child_struct {
/// AVClass *class;
/// int flags_opt;
/// } child_struct;
/// static const AVOption child_opts[] = {
/// { "test_flags", "This is a test option of flags type.",
/// offsetof(child_struct, flags_opt), AV_OPT_TYPE_FLAGS, { .i64 = 0 }, INT_MIN, INT_MAX },
/// { NULL },
/// };
/// static const AVClass child_class = {
/// .class_name = "child class",
/// .item_name  = av_default_item_name,
/// .option     = child_opts,
/// .version    = LIBAVUTIL_VERSION_INT,
/// };
///
/// void *child_next(void *obj, void *prev)
/// {
/// test_struct *t = obj;
/// if (!prev && t->child_struct)
/// return t->child_struct;
/// return NULL
/// }
/// const AVClass child_class_next(const AVClass *prev)
/// {
/// return prev ? NULL : &child_class;
/// }
/// @endcode
/// Putting child_next() and child_class_next() as defined above into
/// test_class will now make child_struct's options accessible through
/// test_struct (again, proper setup as described above needs to be done on
/// child_struct right after it is created).
///
/// From the above example it might not be clear why both child_next()
/// and child_class_next() are needed. The distinction is that child_next()
/// iterates over actually existing objects, while child_class_next()
/// iterates over all possible child classes. E.g. if an AVCodecContext
/// was initialized to use a codec which has private options, then its
/// child_next() will return AVCodecContext.priv_data and finish
/// iterating. OTOH child_class_next() on AVCodecContext.av_class will
/// iterate over all available codecs with private options.
///
/// @subsection avoptions_implement_named_constants Named constants
/// It is possible to create named constants for options. Simply set the unit
/// field of the option the constants should apply to a string and
/// create the constants themselves as options of type AV_OPT_TYPE_CONST
/// with their unit field set to the same string.
/// Their default_val field should contain the value of the named
/// constant.
/// For example, to add some named constants for the test_flags option
/// above, put the following into the child_opts array:
/// @code
/// { "test_flags", "This is a test option of flags type.",
/// offsetof(child_struct, flags_opt), AV_OPT_TYPE_FLAGS, { .i64 = 0 }, INT_MIN, INT_MAX, "test_unit" },
/// { "flag1", "This is a flag with value 16", 0, AV_OPT_TYPE_CONST, { .i64 = 16 }, 0, 0, "test_unit" },
/// @endcode
///
/// @section avoptions_use Using AVOptions
/// This section deals with accessing options in an AVOptions-enabled struct.
/// Such structs in FFmpeg are e.g. AVCodecContext in libavcodec or
/// AVFormatContext in libavformat.
///
/// @subsection avoptions_use_examine Examining AVOptions
/// The basic functions for examining options are av_opt_next(), which iterates
/// over all options defined for one object, and av_opt_find(), which searches
/// for an option with the given name.
///
/// The situation is more complicated with nesting. An AVOptions-enabled struct
/// may have AVOptions-enabled children. Passing the AV_OPT_SEARCH_CHILDREN flag
/// to av_opt_find() will make the function search children recursively.
///
/// For enumerating there are basically two cases. The first is when you want to
/// get all options that may potentially exist on the struct and its children
/// (e.g.  when constructing documentation). In that case you should call
/// av_opt_child_class_next() recursively on the parent struct's AVClass.  The
/// second case is when you have an already initialized struct with all its
/// children and you want to get all options that can be actually written or read
/// from it. In that case you should call av_opt_child_next() recursively (and
/// av_opt_next() on each result).
///
/// @subsection avoptions_use_get_set Reading and writing AVOptions
/// When setting options, you often have a string read directly from the
/// user. In such a case, simply passing it to av_opt_set() is enough. For
/// non-string type options, av_opt_set() will parse the string according to the
/// option type.
///
/// Similarly av_opt_get() will read any option type and convert it to a string
/// which will be returned. Do not forget that the string is allocated, so you
/// have to free it with av_free().
///
/// In some cases it may be more convenient to put all options into an
/// AVDictionary and call av_opt_set_dict() on it. A specific case of this
/// are the format/codec open functions in lavf/lavc which take a dictionary
/// filled with option as a parameter. This makes it possible to set some options
/// that cannot be set otherwise, since e.g. the input file format is not known
/// before the file is actually opened.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVOptionType {
    AV_OPT_TYPE_FLAGS = 0,
    AV_OPT_TYPE_INT = 1,
    AV_OPT_TYPE_INT64 = 2,
    AV_OPT_TYPE_DOUBLE = 3,
    AV_OPT_TYPE_FLOAT = 4,
    AV_OPT_TYPE_STRING = 5,
    AV_OPT_TYPE_RATIONAL = 6,
    AV_OPT_TYPE_BINARY = 7,
    AV_OPT_TYPE_DICT = 8,
    AV_OPT_TYPE_UINT64 = 9,
    AV_OPT_TYPE_CONST = 10,
    AV_OPT_TYPE_IMAGE_SIZE = 11,
    AV_OPT_TYPE_PIXEL_FMT = 12,
    AV_OPT_TYPE_SAMPLE_FMT = 13,
    AV_OPT_TYPE_VIDEO_RATE = 14,
    AV_OPT_TYPE_DURATION = 15,
    AV_OPT_TYPE_COLOR = 16,
    AV_OPT_TYPE_CHANNEL_LAYOUT = 17,
    AV_OPT_TYPE_BOOL = 18
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVClassCategory {
    AV_CLASS_CATEGORY_NA = 0, AV_CLASS_CATEGORY_INPUT = 1, AV_CLASS_CATEGORY_OUTPUT = 2,
    AV_CLASS_CATEGORY_MUXER = 3, AV_CLASS_CATEGORY_DEMUXER = 4, AV_CLASS_CATEGORY_ENCODER = 5,
    AV_CLASS_CATEGORY_DECODER = 6, AV_CLASS_CATEGORY_FILTER = 7, AV_CLASS_CATEGORY_BITSTREAM_FILTER = 8,
    AV_CLASS_CATEGORY_SWSCALER = 9, AV_CLASS_CATEGORY_SWRESAMPLER = 10, AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT = 40,
    AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT = 41, AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT = 42, AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT = 43,
    AV_CLASS_CATEGORY_DEVICE_OUTPUT = 44, AV_CLASS_CATEGORY_DEVICE_INPUT = 45, AV_CLASS_CATEGORY_NB = 46
}

/// AVOption
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AVOption {
    pub name: *const libc::c_char,
    /// short English help text
   /// @todo What about other languages?
    pub help: *const libc::c_char,
    /// The offset relative to the context structure where the option
   /// value is stored. It should be 0 for named constants.
    pub offset: libc::c_int,
    pub type_: AVOptionType,
    pub default_val: AVOption__bindgen_ty_1,
    /// < minimum valid value for the option
    pub min: f64,
    /// < maximum valid value for the option
    pub max: f64,
    pub flags: libc::c_int,
    /// The logical unit to which the option belongs. Non-constant
   /// options and corresponding named constants share the same
   /// unit. May be NULL.
    pub unit: *const libc::c_char,
}

/// A single allowed range of values, or a single allowed value.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AVOptionRange {
    pub str: *const libc::c_char,
    /// Value range.
   /// For string ranges this represents the min/max length.
   /// For dimensions this represents the min/max pixel count or width/height in multi-component case.
    pub value_min: f64,
    /// Value range.
   /// For string ranges this represents the min/max length.
   /// For dimensions this represents the min/max pixel count or width/height in multi-component case.
    pub value_max: f64,
    /// Value's component range.
   /// For string this represents the unicode range for chars, 0-127 limits to ASCII.
    pub component_min: f64,
    /// Value's component range.
   /// For string this represents the unicode range for chars, 0-127 limits to ASCII.
    pub component_max: f64,
    /// Range flag.
   /// If set to 1 the struct encodes a range, if set to 0 a single value.
    pub is_range: libc::c_int,
}

/// List of AVOptionRange structs.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVOptionRanges {
    /// Array of option ranges.
   ///
   /// Most of option types use just one component.
   /// Following describes multi-component option types:
   ///
   /// AV_OPT_TYPE_IMAGE_SIZE:
   /// component index 0: range of pixel count (width * height).
   /// component index 1: range of width.
   /// component index 2: range of height.
   ///
   /// @note To obtain multi-component version of this structure, user must
   /// provide AV_OPT_MULTI_COMPONENT_RANGE to av_opt_query_ranges or
   /// av_opt_query_ranges_default function.
   ///
   /// Multi-component range can be read as in following example:
   ///
   /// @code
   /// int range_index, component_index;
   /// AVOptionRanges *ranges;
   /// AVOptionRange *range[3]; //may require more than 3 in the future.
   /// av_opt_query_ranges(&ranges, obj, key, AV_OPT_MULTI_COMPONENT_RANGE);
   /// for (range_index = 0; range_index < ranges->nb_ranges; range_index++) {
   /// for (component_index = 0; component_index < ranges->nb_components; component_index++)
   /// range[component_index] = ranges->range[ranges->nb_ranges * component_index + range_index];
   /// //do something with range here.
   /// }
   /// av_opt_freep_ranges(&ranges);
   /// @endcode
    pub range: *mut *mut AVOptionRange,
    /// Number of ranges per component.
    pub nb_ranges: libc::c_int,
    /// Number of componentes.
    pub nb_components: libc::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVClass {
    /// The name of the class; usually it is the same name as the
   /// context structure type to which the AVClass is associated.
    pub class_name: *const libc::c_char,
    /// A pointer to a function which returns the name of a context
   /// instance ctx associated with the class.
    pub item_name: ::std::option::Option<unsafe extern "C" fn(ctx: *mut libc::c_void) -> *const libc::c_char>,
    /// a pointer to the first option specified in the class if any or NULL
   ///
   /// @see av_set_default_options()
    pub option: *const AVOption,
    /// LIBAVUTIL_VERSION with which this structure was created.
   /// This is used to allow fields to be added without requiring major
   /// version bumps everywhere.
    pub version: libc::c_int,
    /// Offset in the structure where log_level_offset is stored.
   /// 0 means there is no such variable
    pub log_level_offset_offset: libc::c_int,
    /// Offset in the structure where a pointer to the parent context for
   /// logging is stored. For example a decoder could pass its AVCodecContext
   /// to eval as such a parent context, which an av_log() implementation
   /// could then leverage to display the parent context.
   /// The offset can be NULL.
    pub parent_log_context_offset: libc::c_int,
    /// Return next AVOptions-enabled child or NULL
    pub child_next: ::std::option::Option<unsafe extern "C" fn(obj: *mut libc::c_void, prev: *mut libc::c_void) -> *mut libc::c_void>,
    /// Return an AVClass corresponding to the next potential
   /// AVOptions-enabled child.
   ///
   /// The difference between child_next and this is that
   /// child_next iterates over _already existing_ objects, while
   /// child_class_next iterates over _all possible_ children.
    pub child_class_next: ::std::option::Option<unsafe extern "C" fn(prev: *const AVClass) -> *const AVClass>,
    /// Category used for visualization (like color)
   /// This is only set if the category is equal for all objects using this class.
   /// available since version (51 << 16 | 56 << 8 | 100)
    pub category: AVClassCategory,
    /// Callback to return the category.
   /// available since version (51 << 16 | 59 << 8 | 100)
    pub get_category: ::std::option::Option<unsafe extern "C" fn(ctx: *mut libc::c_void) -> AVClassCategory>,
    /// Callback to return the supported/allowed ranges.
   /// available since version (52.12)
    pub query_ranges: ::std::option::Option<unsafe extern "C" fn(arg1: *mut *mut AVOptionRanges, obj: *mut libc::c_void, key: *const libc::c_char, flags: libc::c_int) -> libc::c_int>,
}

/// @addtogroup lavf_encoding
/// @{
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVOutputFormat {
    pub name: *const libc::c_char,
    /// Descriptive name for the format, meant to be more human-readable
   /// than name. You should use the NULL_IF_CONFIG_SMALL() macro
   /// to define it.
    pub long_name: *const libc::c_char,
    pub mime_type: *const libc::c_char,
    /// < comma-separated filename extensions
    pub extensions: *const libc::c_char,
    /// < default audio codec
    pub audio_codec: AVCodecID,
    /// < default video codec
    pub video_codec: AVCodecID,
    /// < default subtitle codec
    pub subtitle_codec: AVCodecID,
    /// can use flags: AVFMT_NOFILE, AVFMT_NEEDNUMBER,
   /// AVFMT_GLOBALHEADER, AVFMT_NOTIMESTAMPS, AVFMT_VARIABLE_FPS,
   /// AVFMT_NODIMENSIONS, AVFMT_NOSTREAMS, AVFMT_ALLOW_FLUSH,
   /// AVFMT_TS_NONSTRICT, AVFMT_TS_NEGATIVE
    pub flags: libc::c_int,
    /// List of supported codec_id-codec_tag pairs, ordered by "better
   /// choice first". The arrays are all terminated by AV_CODEC_ID_NONE.
    pub codec_tag: *const *const AVCodecTag,
    /// < AVClass for the private context
    pub priv_class: *const AVClass,
    /// No fields below this line are part of the public API. They
   /// may not be used outside of libavformat and can be changed and
   /// removed at will.
   /// New public fields should be added right above.
   ///
    pub next: *mut AVOutputFormat,
    /// size of private common so that it can be allocated in the wrapper
    pub priv_data_size: libc::c_int,
    pub write_header: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext) -> libc::c_int>,
    /// Write a packet. If AVFMT_ALLOW_FLUSH is set in flags,
   /// pkt can be NULL in order to flush common buffered in the muxer.
   /// When flushing, return 0 if there still is more common to flush,
   /// or 1 if everything was flushed and there is no more buffered
   /// common.
    pub write_packet: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext, pkt: *mut AVPacket) -> libc::c_int>,
    pub write_trailer: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext) -> libc::c_int>,
    /// Currently only used to set pixel format if not YUV420P.
    pub interleave_packet: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext, out: *mut AVPacket, in_: *mut AVPacket, flush: libc::c_int) -> libc::c_int>,
    /// Test if the given codec can be stored in this container.
   ///
   /// @return 1 if the codec is supported, 0 if it is not.
   /// A negative number if unknown.
   /// MKTAG('A', 'P', 'I', 'C') if the codec is only supported as AV_DISPOSITION_ATTACHED_PIC
    pub query_codec: ::std::option::Option<unsafe extern "C" fn(id: AVCodecID, std_compliance: libc::c_int) -> libc::c_int>,
    pub get_output_timestamp: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, stream: libc::c_int, dts: *mut i64, wall: *mut i64)>,
    /// Allows sending messages from application to device.
    pub control_message: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, type_: libc::c_int, data: *mut libc::c_void, data_size: usize) -> libc::c_int>,
    /// Write an uncoded AVFrame.
   ///
   /// See av_write_uncoded_frame() for details.
   ///
   /// The library will free *frame afterwards, but the muxer can prevent it
   /// by setting the pointer to NULL.
    pub write_uncoded_frame: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext, stream_index: libc::c_int, frame: *mut *mut AVFrame, flags: libc::c_uint) -> libc::c_int>,
    /// Returns device list with it properties.
   /// @see avdevice_list_devices() for more details.
    pub get_device_list: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, device_list: *mut AVDeviceInfoList) -> libc::c_int>,
    /// Initialize device capabilities submodule.
   /// @see avdevice_capabilities_create() for more details.
    pub create_device_capabilities: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, caps: *mut AVDeviceCapabilitiesQuery) -> libc::c_int>,
    /// Free device capabilities submodule.
   /// @see avdevice_capabilities_free() for more details.
    pub free_device_capabilities: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, caps: *mut AVDeviceCapabilitiesQuery) -> libc::c_int>,
    /// < default common codec
    pub data_codec: AVCodecID,
    /// Initialize format. May allocate common here, and set any AVFormatContext or
   /// AVStream parameters that need to be set before packets are sent.
   /// This method must not write output.
   ///
   /// Return 0 if streams were fully configured, 1 if not, negative AVERROR on failure
   ///
   /// Any allocations made here must be freed in deinit().
    pub init: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext) -> libc::c_int>,
    /// Deinitialize format. If present, this is called whenever the muxer is being
   /// destroyed, regardless of whether or not the header has been written.
   ///
   /// If a trailer is being written, this is called after write_trailer().
   ///
   /// This is called if init() fails as well.
    pub deinit: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext)>,
    /// Set up any necessary bitstream filtering and extract any extra common needed
   /// for the global header.
   /// Return 0 if more packets from this stream must be checked; 1 if not.
    pub check_bitstream: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext, pkt: *const AVPacket) -> libc::c_int>,
}

#[repr(u32)]      /// Different common types that can be returned via the AVIO
/// write_data_type callback.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVIODataMarkerType {
    AVIO_DATA_MARKER_HEADER = 0, AVIO_DATA_MARKER_SYNC_POINT = 1,
    AVIO_DATA_MARKER_BOUNDARY_POINT = 2, AVIO_DATA_MARKER_UNKNOWN = 3,
    AVIO_DATA_MARKER_TRAILER = 4, AVIO_DATA_MARKER_FLUSH_POINT = 5
}

/// Bytestream IO Context.
/// New fields can be added to the end with minor version bumps.
/// Removal, reordering and changes to existing fields require a major
/// version bump.
/// sizeof(AVIOContext) must not be used outside libav*.
///
/// @note None of the function pointers in AVIOContext should be called
/// directly, they should only be set by the client application
/// when implementing custom I/O. Normally these are set to the
/// function pointers specified in avio_alloc_context()
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVIOContext {
    /// A class for private options.
   ///
   /// If this AVIOContext is created by avio_open2(), av_class is set and
   /// passes the options down to protocols.
   ///
   /// If this AVIOContext is manually allocated, then av_class may be set by
   /// the caller.
   ///
   /// warning -- this field can be NULL, be sure to not pass this AVIOContext
   /// to any av_opt_* functions in that case.
    pub av_class: *const AVClass,
    /// < Start of the buffer.
    pub buffer: *mut libc::c_uchar,
    /// < Maximum buffer size
    pub buffer_size: libc::c_int,
    /// < Current position in the buffer
    pub buf_ptr: *mut libc::c_uchar,
    /// < End of the common, may be less than
   /// buffer+buffer_size if the read function returned
   /// less common than requested, e.g. for streams where
   /// no more common has been received yet.
    pub buf_end: *mut libc::c_uchar,
    /// < A private pointer, passed to the read/write/seek/...
   /// functions.
    pub opaque: *mut libc::c_void,
    pub read_packet: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void, buf: *mut u8, buf_size: libc::c_int) -> libc::c_int>,
    pub write_packet: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void, buf: *mut u8, buf_size: libc::c_int) -> libc::c_int>,
    pub seek: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void, offset: i64, whence: libc::c_int) -> i64>,
    /// < position in the file of the current buffer
    pub pos: i64,
    /// < true if eof reached
    pub eof_reached: libc::c_int,
    /// < true if open for writing
    pub write_flag: libc::c_int,
    pub max_packet_size: libc::c_int,
    pub checksum: libc::c_ulong,
    pub checksum_ptr: *mut libc::c_uchar,
    pub update_checksum: ::std::option::Option<unsafe extern "C" fn(checksum: libc::c_ulong, buf: *const u8, size: libc::c_uint) -> libc::c_ulong>,
    /// < contains the error code or 0 if no error happened
    pub error: libc::c_int,
    /// Pause or resume playback for network streaming protocols - e.g. MMS.
    pub read_pause: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void, pause: libc::c_int) -> libc::c_int>,
    /// Seek to a given timestamp in stream with the specified stream_index.
   /// Needed for some network streaming protocols which don't support seeking
   /// to byte position.
    pub read_seek: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void, stream_index: libc::c_int, timestamp: i64, flags: libc::c_int) -> i64>,
    /// A combination of AVIO_SEEKABLE_ flags or 0 when the stream is not seekable.
    pub seekable: libc::c_int,
    /// max filesize, used to limit allocations
   /// This field is internal to libavformat and access from outside is not allowed.
    pub maxsize: i64,
    /// avio_read and avio_write should if possible be satisfied directly
   /// instead of going through a buffer, and avio_seek will always
   /// call the underlying seek function directly.
    pub direct: libc::c_int,
    /// Bytes read statistic
   /// This field is internal to libavformat and access from outside is not allowed.
    pub bytes_read: i64,
    /// seek statistic
   /// This field is internal to libavformat and access from outside is not allowed.
    pub seek_count: libc::c_int,
    /// writeout statistic
   /// This field is internal to libavformat and access from outside is not allowed.
    pub writeout_count: libc::c_int,
    /// Original buffer size
   /// used internally after probing and ensure seekback to reset the buffer size
   /// This field is internal to libavformat and access from outside is not allowed.
    pub orig_buffer_size: libc::c_int,
    /// Threshold to favor readahead over seek.
   /// This is current internal only, do not use from outside.
    pub short_seek_threshold: libc::c_int,
    /// ',' separated list of allowed protocols.
    pub protocol_whitelist: *const libc::c_char,
    /// ',' separated list of disallowed protocols.
    pub protocol_blacklist: *const libc::c_char,
    /// A callback that is used instead of write_packet.
    pub write_data_type: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void, buf: *mut u8, buf_size: libc::c_int, type_: AVIODataMarkerType, time: i64) -> libc::c_int>,
    /// If set, don't call write_data_type separately for AVIO_DATA_MARKER_BOUNDARY_POINT,
   /// but ignore them and treat them as AVIO_DATA_MARKER_UNKNOWN (to avoid needlessly
   /// small chunks of common returned from the callback).
    pub ignore_boundary_point: libc::c_int,
    /// Internal, not meant to be used from outside of AVIOContext.
    pub current_type: AVIODataMarkerType,
    pub last_time: i64,
    /// A callback that is used instead of short_seek_threshold.
   /// This is current internal only, do not use from outside.
    pub short_seek_get: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void) -> libc::c_int>,
    pub written: i64,
    /// Maximum reached position before a backward seek in the write buffer,
   /// used keeping track of already written common for a later flush.
    pub buf_ptr_max: *mut libc::c_uchar,
    /// Try to buffer at least this amount of common before flushing it
    pub min_packet_size: libc::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVStreamInternal {
    _unused: [u8; 0]
}

#[repr(i32)]      /// @addtogroup lavu_media Media Type
/// @brief Media Type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVMediaType {
    AVMEDIA_TYPE_UNKNOWN = -1,
    AVMEDIA_TYPE_VIDEO = 0,
    AVMEDIA_TYPE_AUDIO = 1,
    AVMEDIA_TYPE_DATA = 2,
    AVMEDIA_TYPE_SUBTITLE = 3,
    AVMEDIA_TYPE_ATTACHMENT = 4,
    AVMEDIA_TYPE_NB = 5
}

/// AVProfile.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVProfile {
    pub profile: libc::c_int,
    /// < short name for the profile
    pub name: *const libc::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecDefault {
    _unused: [u8; 0]
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVSubtitleType {
    SUBTITLE_NONE = 0,
    SUBTITLE_BITMAP = 1,
    SUBTITLE_TEXT = 2,
    SUBTITLE_ASS = 3
}

/// Picture common structure.
///
/// Up to four components can be stored into it, the last component is
/// alpha.
/// @deprecated use AVFrame or imgutils functions instead
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVPicture {
    /// < pointers to the image common planes
    pub data: [*mut u8; 8usize],
    /// < number of bytes per line
    pub linesize: [libc::c_int; 8usize],
}


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVSubtitleRect {
    /// < top left corner  of pict, undefined when pict is not set
    pub x: libc::c_int,
    /// < top left corner  of pict, undefined when pict is not set
    pub y: libc::c_int,
    /// < width            of pict, undefined when pict is not set
    pub w: libc::c_int,
    /// < height           of pict, undefined when pict is not set
    pub h: libc::c_int,
    /// < number of colors in pict, undefined when pict is not set
    pub nb_colors: libc::c_int,
    /// @deprecated unused
    pub pict: AVPicture,
    /// common+linesize for the bitmap of this subtitle.
   /// Can be set for text/ass as well once they are rendered.
    pub data: [*mut u8; 4usize],
    pub linesize: [libc::c_int; 4usize],
    pub type_: AVSubtitleType,
    /// < 0 terminated plain UTF-8 text
    pub text: *mut libc::c_char,
    /// 0 terminated ASS/SSA compatible event line.
   /// The presentation of this is unaffected by the other values in this
   /// struct.
    pub ass: *mut libc::c_char,
    pub flags: libc::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVSubtitle {
    pub format: u16,
    pub start_display_time: u32,
    pub end_display_time: u32,
    pub num_rects: libc::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    /// < Same as packet pts, in AV_TIME_BASE
    pub pts: i64,
}

#[repr(u32)]      /// @defgroup lavu_frame AVFrame
/// @ingroup lavu_data
///
/// @{
/// AVFrame is an abstraction for reference-counted raw multimedia common.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVFrameSideDataType {
    AV_FRAME_DATA_PANSCAN = 0, AV_FRAME_DATA_A53_CC = 1, AV_FRAME_DATA_STEREO3D = 2,
    AV_FRAME_DATA_MATRIXENCODING = 3, AV_FRAME_DATA_DOWNMIX_INFO = 4, AV_FRAME_DATA_REPLAYGAIN = 5,
    AV_FRAME_DATA_DISPLAYMATRIX = 6, AV_FRAME_DATA_AFD = 7, AV_FRAME_DATA_MOTION_VECTORS = 8,
    AV_FRAME_DATA_SKIP_SAMPLES = 9, AV_FRAME_DATA_AUDIO_SERVICE_TYPE = 10, AV_FRAME_DATA_MASTERING_DISPLAY_METADATA = 11,
    AV_FRAME_DATA_GOP_TIMECODE = 12, AV_FRAME_DATA_SPHERICAL = 13, AV_FRAME_DATA_CONTENT_LIGHT_LEVEL = 14,
    AV_FRAME_DATA_ICC_PROFILE = 15, AV_FRAME_DATA_QP_TABLE_PROPERTIES = 16, AV_FRAME_DATA_QP_TABLE_DATA = 17,
    AV_FRAME_DATA_S12M_TIMECODE = 18
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVDictionary { _unused: [u8; 0] }
/// Structure to hold side common for an AVFrame.
///
/// sizeof(AVFrameSideData) is not a part of the public ABI, so new fields may be added
/// to the end with a minor bump.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVFrameSideData {
    pub type_: AVFrameSideDataType,
    pub data: *mut u8,
    pub size: libc::c_int,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef
}

#[repr(u32)]      /// @}
/// @}
/// @defgroup lavu_picture Image related
///
/// AVPicture types, pixel formats and basic image planes manipulation.
///
/// @{
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVPictureType {
    AV_PICTURE_TYPE_NONE = 0,
    AV_PICTURE_TYPE_I = 1,
    AV_PICTURE_TYPE_P = 2,
    AV_PICTURE_TYPE_B = 3,
    AV_PICTURE_TYPE_S = 4,
    AV_PICTURE_TYPE_SI = 5,
    AV_PICTURE_TYPE_SP = 6,
    AV_PICTURE_TYPE_BI = 7
}

#[repr(u32)]      /// MPEG vs JPEG YUV range.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVColorRange {
    AVCOL_RANGE_UNSPECIFIED = 0,
    AVCOL_RANGE_MPEG = 1,
    AVCOL_RANGE_JPEG = 2,
    AVCOL_RANGE_NB = 3
}

#[repr(u32)]      /// Chromaticity coordinates of the source primaries.
/// These values match the ones defined by ISO/IEC 23001-8_2013 § 7.1.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVColorPrimaries {
    AVCOL_PRI_RESERVED0 = 0, AVCOL_PRI_BT709 = 1,
    AVCOL_PRI_UNSPECIFIED = 2, AVCOL_PRI_RESERVED = 3,
    AVCOL_PRI_BT470M = 4, AVCOL_PRI_BT470BG = 5,
    AVCOL_PRI_SMPTE170M = 6, AVCOL_PRI_SMPTE240M = 7,
    AVCOL_PRI_FILM = 8, AVCOL_PRI_BT2020 = 9,
    AVCOL_PRI_SMPTE428 = 10, AVCOL_PRI_SMPTE431 = 11,
    AVCOL_PRI_SMPTE432 = 12, AVCOL_PRI_JEDEC_P22 = 22,
    AVCOL_PRI_NB = 23
}

#[repr(u32)]      /// Color Transfer Characteristic.
/// These values match the ones defined by ISO/IEC 23001-8_2013 § 7.2.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVColorTransferCharacteristic {
    AVCOL_TRC_RESERVED0 = 0, AVCOL_TRC_BT709 = 1, AVCOL_TRC_UNSPECIFIED = 2,
    AVCOL_TRC_RESERVED = 3, AVCOL_TRC_GAMMA22 = 4, AVCOL_TRC_GAMMA28 = 5,
    AVCOL_TRC_SMPTE170M = 6, AVCOL_TRC_SMPTE240M = 7, AVCOL_TRC_LINEAR = 8,
    AVCOL_TRC_LOG = 9, AVCOL_TRC_LOG_SQRT = 10, AVCOL_TRC_IEC61966_2_4 = 11,
    AVCOL_TRC_BT1361_ECG = 12, AVCOL_TRC_IEC61966_2_1 = 13, AVCOL_TRC_BT2020_10 = 14,
    AVCOL_TRC_BT2020_12 = 15, AVCOL_TRC_SMPTE2084 = 16, AVCOL_TRC_SMPTE428 = 17,
    AVCOL_TRC_ARIB_STD_B67 = 18, AVCOL_TRC_NB = 19
}

#[repr(u32)]      /// YUV colorspace type.
/// These values match the ones defined by ISO/IEC 23001-8_2013 § 7.3.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVColorSpace {
    AVCOL_SPC_RGB = 0, AVCOL_SPC_BT709 = 1,
    AVCOL_SPC_UNSPECIFIED = 2, AVCOL_SPC_RESERVED = 3,
    AVCOL_SPC_FCC = 4, AVCOL_SPC_BT470BG = 5,
    AVCOL_SPC_SMPTE170M = 6, AVCOL_SPC_SMPTE240M = 7,
    AVCOL_SPC_YCGCO = 8, AVCOL_SPC_BT2020_NCL = 9,
    AVCOL_SPC_BT2020_CL = 10, AVCOL_SPC_SMPTE2085 = 11,
    AVCOL_SPC_CHROMA_DERIVED_NCL = 12, AVCOL_SPC_CHROMA_DERIVED_CL = 13,
    AVCOL_SPC_ICTCP = 14, AVCOL_SPC_NB = 15
}

#[repr(u32)]      /// Location of chroma samples.
///
/// Illustration showing the location of the first (top left) chroma sample of the
/// image, the left shows only luma, the right
/// shows the location of the chroma sample, the 2 could be imagined to overlay
/// each other but are drawn separately due to limitations of ASCII
///
/// 1st 2nd       1st 2nd horizontal luma sample positions
/// v   v         v   v
/// ______        ______
/// 1st luma line > |X   X ...    |3 4 X ...     X are luma samples,
/// |             |1 2           1-6 are possible chroma positions
/// 2nd luma line > |X   X ...    |5 6 X ...     0 is undefined/unknown position
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVChromaLocation {
    AVCHROMA_LOC_UNSPECIFIED = 0,
    AVCHROMA_LOC_LEFT = 1,
    AVCHROMA_LOC_CENTER = 2,
    AVCHROMA_LOC_TOPLEFT = 3,
    AVCHROMA_LOC_TOP = 4,
    AVCHROMA_LOC_BOTTOMLEFT = 5,
    AVCHROMA_LOC_BOTTOM = 6,
    AVCHROMA_LOC_NB = 7
}

/// This structure describes decoded (raw) audio or video common.
///
/// AVFrame must be allocated using av_frame_alloc(). Note that this only
/// allocates the AVFrame itself, the buffers for the common must be managed
/// through other means (see below).
/// AVFrame must be freed with av_frame_free().
///
/// AVFrame is typically allocated once and then reused multiple times to hold
/// different common (e.g. a single AVFrame to hold frames received from a
/// decoder). In such a case, av_frame_unref() will free any references held by
/// the frame and reset it to its original clean state before it
/// is reused again.
///
/// The common described by an AVFrame is usually reference counted through the
/// AVBuffer API. The underlying buffer references are stored in AVFrame.buf /
/// AVFrame.extended_buf. An AVFrame is considered to be reference counted if at
/// least one reference is set, i.e. if AVFrame.buf[0] != NULL. In such a case,
/// every single common plane must be contained in one of the buffers in
/// AVFrame.buf or AVFrame.extended_buf.
/// There may be a single buffer for all the common, or one separate buffer for
/// each plane, or anything in between.
///
/// sizeof(AVFrame) is not a part of the public ABI, so new fields may be added
/// to the end with a minor bump.
///
/// Fields can be accessed through AVOptions, the name string used, matches the
/// C structure field name for fields accessible through AVOptions. The AVClass
/// for AVFrame can be obtained from avcodec_get_frame_class()
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVFrame {
    /// pointer to the picture/channel planes.
   /// This might be different from the first allocated byte
   ///
   /// Some decoders access areas outside 0,0 - width,height, please
   /// see avcodec_align_dimensions2(). Some filters and swscale can read
   /// up to 16 bytes beyond the planes, if these filters are to be used,
   /// then 16 extra bytes must be allocated.
   ///
   /// NOTE: Except for hwaccel formats, pointers not needed by the format
   /// MUST be set to NULL.
    pub data: [*mut u8; 8usize],
    /// For video, size in bytes of each picture line.
   /// For audio, size in bytes of each plane.
   ///
   /// For audio, only linesize[0] may be set. For planar audio, each channel
   /// plane must be the same size.
   ///
   /// For video the linesizes should be multiples of the CPUs alignment
   /// preference, this is 16 or 32 for modern desktop CPUs.
   /// Some code requires such alignment other code can be slower without
   /// correct alignment, for yet other it makes no difference.
   ///
   /// @note The linesize may be larger than the size of usable common -- there
   /// may be extra padding present for performance reasons.
    pub linesize: [libc::c_int; 8usize],
    /// pointers to the common planes/channels.
   ///
   /// For video, this should simply point to common[].
   ///
   /// For planar audio, each channel has a separate common pointer, and
   /// linesize[0] contains the size of each channel buffer.
   /// For packed audio, there is just one common pointer, and linesize[0]
   /// contains the total size of the buffer for all channels.
   ///
   /// Note: Both common and extended_data should always be set in a valid frame,
   /// but for planar audio with more channels that can fit in common,
   /// extended_data must be used in order to access all channels.
    pub extended_data: *mut *mut u8,
    /// @name Video dimensions
   /// Video frames only. The coded dimensions (in pixels) of the video frame,
   /// i.e. the size of the rectangle that contains some well-defined values.
   ///
   /// @note The part of the frame intended for display/presentation is further
   /// restricted by the @ref cropping "Cropping rectangle".
   /// @{
    pub width: libc::c_int,
    /// @name Video dimensions
   /// Video frames only. The coded dimensions (in pixels) of the video frame,
   /// i.e. the size of the rectangle that contains some well-defined values.
   ///
   /// @note The part of the frame intended for display/presentation is further
   /// restricted by the @ref cropping "Cropping rectangle".
   /// @{
    pub height: libc::c_int,
    /// number of audio samples (per channel) described by this frame
    pub nb_samples: libc::c_int,
    /// format of the frame, -1 if unknown or unset
   /// Values correspond to enum AVPixelFormat for video frames,
   /// enum AVSampleFormat for audio)
    pub format: libc::c_int,
    /// 1 -> keyframe, 0-> not
    pub key_frame: libc::c_int,
    /// Picture type of the frame.
    pub pict_type: AVPictureType,
    /// Sample aspect ratio for the video frame, 0/1 if unknown/unspecified.
    pub sample_aspect_ratio: AVRational,
    /// Presentation timestamp in time_base units (time when frame should be shown to user).
    pub pts: i64,
    /// PTS copied from the AVPacket that was decoded to produce this frame.
   /// @deprecated use the pts field instead
    pub pkt_pts: i64,
    /// DTS copied from the AVPacket that triggered returning this frame. (if frame threading isn't used)
   /// This is also the Presentation time of this AVFrame calculated from
   /// only AVPacket.dts values without pts values.
    pub pkt_dts: i64,
    /// picture number in bitstream order
    pub coded_picture_number: libc::c_int,
    /// picture number in display order
    pub display_picture_number: libc::c_int,
    /// quality (between 1 (good) and FF_LAMBDA_MAX (bad))
    pub quality: libc::c_int,
    /// for some private common of the user
    pub opaque: *mut libc::c_void,
    /// @deprecated unused
    pub error: [u64; 8usize],
    /// When decoding, this signals how much the picture must be delayed.
   /// extra_delay = repeat_pict / (2*fps)
    pub repeat_pict: libc::c_int,
    /// The content of the picture is interlaced.
    pub interlaced_frame: libc::c_int,
    /// If the content is interlaced, is top field displayed first.
    pub top_field_first: libc::c_int,
    /// Tell user application that palette has changed from previous frame.
    pub palette_has_changed: libc::c_int,
    /// reordered opaque 64 bits (generally an integer or a double precision float
   /// PTS but can be anything).
   /// The user sets AVCodecContext.reordered_opaque to represent the input at
   /// that time,
   /// the decoder reorders values as needed and sets AVFrame.reordered_opaque
   /// to exactly one of the values provided by the user through AVCodecContext.reordered_opaque
   /// @deprecated in favor of pkt_pts
    pub reordered_opaque: i64,
    /// Sample rate of the audio common.
    pub sample_rate: libc::c_int,
    /// Channel layout of the audio common.
    pub channel_layout: u64,
    /// AVBuffer references backing the common for this frame. If all elements of
   /// this array are NULL, then this frame is not reference counted. This array
   /// must be filled contiguously -- if buf[i] is non-NULL then buf[j] must
   /// also be non-NULL for all j < i.
   ///
   /// There may be at most one AVBuffer per common plane, so for video this array
   /// always contains all the references. For planar audio with more than
   /// AV_NUM_DATA_POINTERS channels, there may be more buffers than can fit in
   /// this array. Then the extra AVBufferRef pointers are stored in the
   /// extended_buf array.
    pub buf: [*mut AVBufferRef; 8usize],
    /// For planar audio which requires more than AV_NUM_DATA_POINTERS
   /// AVBufferRef pointers, this array will hold all the references which
   /// cannot fit into AVFrame.buf.
   ///
   /// Note that this is different from AVFrame.extended_data, which always
   /// contains all the pointers. This array only contains the extra pointers,
   /// which cannot fit into AVFrame.buf.
   ///
   /// This array is always allocated using av_malloc() by whoever constructs
   /// the frame. It is freed in av_frame_unref().
    pub extended_buf: *mut *mut AVBufferRef,
    /// Number of elements in extended_buf.
    pub nb_extended_buf: libc::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: libc::c_int,
    /// Frame flags, a combination of @ref lavu_frame_flags
    pub flags: libc::c_int,
    /// MPEG vs JPEG YUV range.
   /// - encoding: Set by user
   /// - decoding: Set by libavcodec
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    /// YUV colorspace type.
   /// - encoding: Set by user
   /// - decoding: Set by libavcodec
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    /// frame timestamp estimated using various heuristics, in stream time base
   /// - encoding: unused
   /// - decoding: set by libavcodec, read by user.
    pub best_effort_timestamp: i64,
    /// reordered pos from the last AVPacket that has been input into the decoder
   /// - encoding: unused
   /// - decoding: Read by user.
    pub pkt_pos: i64,
    /// duration of the corresponding packet, expressed in
   /// AVStream->time_base units, 0 if unknown.
   /// - encoding: unused
   /// - decoding: Read by user.
    pub pkt_duration: i64,
    /// metadata.
   /// - encoding: Set by user.
   /// - decoding: Set by libavcodec.
    pub metadata: *mut AVDictionary,
    /// decode error flags of the frame, set to a combination of
   /// FF_DECODE_ERROR_xxx flags if the decoder produced a frame, but there
   /// were errors during the decoding.
   /// - encoding: unused
   /// - decoding: set by libavcodec, read by user.
    pub decode_error_flags: libc::c_int,
    /// number of audio channels, only used for audio.
   /// - encoding: unused
   /// - decoding: Read by user.
    pub channels: libc::c_int,
    /// size of the corresponding packet containing the compressed
   /// frame.
   /// It is set to a negative value if unknown.
   /// - encoding: unused
   /// - decoding: set by libavcodec, read by user.
    pub pkt_size: libc::c_int,
    /// QP table
    pub qscale_table: *mut i8,
    /// QP store stride
    pub qstride: libc::c_int,
    pub qscale_type: libc::c_int,
    pub qp_table_buf: *mut AVBufferRef,
    /// For hwaccel-format frames, this should be a reference to the
   /// AVHWFramesContext describing the frame.
    pub hw_frames_ctx: *mut AVBufferRef,
    /// AVBufferRef for free use by the API user. FFmpeg will never check the
   /// contents of the buffer ref. FFmpeg calls av_buffer_unref() on it when
   /// the frame is unreferenced. av_frame_copy_props() calls create a new
   /// reference with av_buffer_ref() for the target frame's opaque_ref field.
   ///
   /// This is unrelated to the opaque field, although it serves a similar
   /// purpose.
    pub opaque_ref: *mut AVBufferRef,
    /// @anchor cropping
   /// @name Cropping
   /// Video frames only. The number of pixels to discard from the the
   /// top/bottom/left/right border of the frame to obtain the sub-rectangle of
   /// the frame intended for presentation.
   /// @{
    pub crop_top: usize,
    pub crop_bottom: usize,
    pub crop_left: usize,
    pub crop_right: usize,
    /// AVBufferRef for internal use by a single libav* library.
   /// Must not be used to transfer common between libraries.
   /// Has to be NULL when ownership of the frame leaves the respective library.
   ///
   /// Code outside the FFmpeg libs should never check or change the contents of the buffer ref.
   ///
   /// FFmpeg calls av_buffer_unref() on it when the frame is unreferenced.
   /// av_frame_copy_props() calls create a new reference with av_buffer_ref()
   /// for the target frame's private_ref field.
    pub private_ref: *mut AVBufferRef,
}


/// Array of pointers to hardware configurations supported by the codec,
/// or NULL if no hardware supported.  The array is terminated by a NULL
/// pointer.
///
/// The user can only access this field via avcodec_get_hw_config().
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVCodecHWConfigInternal { pub _address: u8 }
/// AVCodec.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVCodec {
    /// Name of the codec implementation.
   /// The name is globally unique among encoders and among decoders (but an
   /// encoder and a decoder can share the same name).
   /// This is the primary way to find a codec from the user perspective.
    pub name: *const libc::c_char,
    /// Descriptive name for the codec, meant to be more human readable than name.
   /// You should use the NULL_IF_CONFIG_SMALL() macro to define it.
    pub long_name: *const libc::c_char,
    pub type_: AVMediaType,
    pub id: AVCodecID,
    /// Codec capabilities.
   /// see AV_CODEC_CAP_*
    pub capabilities: libc::c_int,
    /// < array of supported framerates, or NULL if any, array is terminated by {0,0}
    pub supported_framerates: *const AVRational,
    /// < array of supported pixel formats, or NULL if unknown, array is terminated by -1
    pub pix_fmts: *const AVPixelFormat,
    /// < array of supported audio samplerates, or NULL if unknown, array is terminated by 0
    pub supported_samplerates: *const libc::c_int,
    /// < array of supported sample formats, or NULL if unknown, array is terminated by -1
    pub sample_fmts: *const AVSampleFormat,
    /// < array of support channel layouts, or NULL if unknown. array is terminated by 0
    pub channel_layouts: *const u64,
    /// < maximum value for lowres supported by the decoder
    pub max_lowres: u8,
    /// < AVClass for the private context
    pub priv_class: *const AVClass,
    /// < array of recognized profiles, or NULL if unknown, array is terminated by {FF_PROFILE_UNKNOWN}
    pub profiles: *const AVProfile,
    /// Group name of the codec implementation.
   /// This is a short symbolic name of the wrapper backing this codec. A
   /// wrapper uses some kind of external implementation for the codec, such
   /// as an external library, or a codec implementation provided by the OS or
   /// the hardware.
   /// If this field is NULL, this is a builtin, libavcodec native codec.
   /// If non-NULL, this will be the suffix in AVCodec.name in most cases
   /// (usually AVCodec.name will be of the form "<codec_name>_<wrapper_name>").
    pub wrapper_name: *const libc::c_char,
    /// No fields below this line are part of the public API. They
   /// may not be used outside of libavcodec and can be changed and
   /// removed at will.
   /// New public fields should be added right above.
   ///
    pub priv_data_size: libc::c_int,
    pub next: *mut AVCodec,
    /// @name Frame-level threading support functions
   /// @{
   /// /
   /// /**
   /// If defined, called on thread contexts when they are created.
   /// If the codec allocates writable tables in init(), re-allocate them here.
   /// priv_data will be set to a copy of the original.
    pub init_thread_copy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVCodecContext) -> libc::c_int>,
    /// Copy necessary context variables from a previous thread context to the current one.
   /// If not defined, the next thread will start automatically; otherwise, the codec
   /// must call ff_thread_finish_setup().
   ///
   /// dst and src will (rarely) point to the same context, in which case memcpy should be skipped.
    pub update_thread_context: ::std::option::Option<unsafe extern "C" fn(dst: *mut AVCodecContext, src: *const AVCodecContext) -> libc::c_int>,
    /// Private codec-specific defaults.
    pub defaults: *const AVCodecDefault,
    /// Initialize codec static common, called from avcodec_register().
   ///
   /// This is not intended for time consuming operations as it is
   /// run for every codec regardless of that codec being used.
    pub init_static_data: ::std::option::Option<unsafe extern "C" fn(codec: *mut AVCodec)>,
    pub init: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVCodecContext) -> libc::c_int>,
    pub encode_sub: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVCodecContext, buf: *mut u8, buf_size: libc::c_int, sub: *const AVSubtitle) -> libc::c_int>,
    /// Encode common to an AVPacket.
   ///
   /// @param      avctx          codec context
   /// @param      avpkt          output AVPacket (may contain a user-provided buffer)
   /// @param[in]  frame          AVFrame containing the raw common to be encoded
   /// @param[out] got_packet_ptr encoder sets to 0 or 1 to indicate that a
   /// non-empty packet was returned in avpkt.
   /// @return 0 on success, negative error code on failure
    pub encode2: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, avpkt: *mut AVPacket, frame: *const AVFrame, got_packet_ptr: *mut libc::c_int) -> libc::c_int>,
    pub decode: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVCodecContext, outdata: *mut libc::c_void, outdata_size: *mut libc::c_int, avpkt: *mut AVPacket) -> libc::c_int>,
    pub close: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVCodecContext) -> libc::c_int>,
    /// Encode API with decoupled packet/frame dataflow. The API is the
   /// same as the avcodec_ prefixed APIs (avcodec_send_frame() etc.), except
   /// that:
   /// - never called if the codec is closed or the wrong type,
   /// - if AV_CODEC_CAP_DELAY is not set, drain frames are never sent,
   /// - only one drain frame is ever passed down,
    pub send_frame: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, frame: *const AVFrame) -> libc::c_int>,
    pub receive_packet: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, avpkt: *mut AVPacket) -> libc::c_int>,
    /// Decode API with decoupled packet/frame dataflow. This function is called
   /// to get one output frame. It should call ff_decode_get_packet() to obtain
   /// input common.
    pub receive_frame: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, frame: *mut AVFrame) -> libc::c_int>,
    /// Flush buffers.
   /// Will be called when seeking
    pub flush: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVCodecContext)>,
    /// Internal codec capabilities.
   /// See FF_CODEC_CAP_* in internal.h
    pub caps_internal: libc::c_int,
    /// Decoding only, a comma-separated list of bitstream filters to apply to
   /// packets before decoding.
    pub bsfs: *const libc::c_char,
    /// Array of pointers to hardware configurations supported by the codec,
   /// or NULL if no hardware supported.  The array is terminated by a NULL
   /// pointer.
   ///
   /// The user can only access this field via avcodec_get_hw_config().
    pub hw_configs: *mut *mut AVCodecHWConfigInternal,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecInternal { _unused: [u8; 0] }

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVFieldOrder {
    AV_FIELD_UNKNOWN = 0,
    AV_FIELD_PROGRESSIVE = 1,
    AV_FIELD_TT = 2,
    AV_FIELD_BB = 3,
    AV_FIELD_TB = 4,
    AV_FIELD_BT = 5
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVAudioServiceType {
    AV_AUDIO_SERVICE_TYPE_MAIN = 0,
    AV_AUDIO_SERVICE_TYPE_EFFECTS = 1,
    AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED = 2,
    AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED = 3,
    AV_AUDIO_SERVICE_TYPE_DIALOGUE = 4,
    AV_AUDIO_SERVICE_TYPE_COMMENTARY = 5,
    AV_AUDIO_SERVICE_TYPE_EMERGENCY = 6,
    AV_AUDIO_SERVICE_TYPE_VOICE_OVER = 7,
    AV_AUDIO_SERVICE_TYPE_KARAOKE = 8,
    AV_AUDIO_SERVICE_TYPE_NB = 9
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MpegEncContext { _unused: [u8; 0] }
/// @defgroup lavc_hwaccel AVHWAccel
///
/// @note  Nothing in this structure should be accessed by the user.  At some
/// point in future it will not be externally visible at all.
///
/// @{
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVHWAccel {
    /// Name of the hardware accelerated codec.
   /// The name is globally unique among encoders and among decoders (but an
   /// encoder and a decoder can share the same name).
    pub name: *const libc::c_char,
    /// Type of codec implemented by the hardware accelerator.
   ///
   /// See AVMEDIA_TYPE_xxx
    pub type_: AVMediaType,
    /// Codec implemented by the hardware accelerator.
   ///
   /// See AV_CODEC_ID_xxx
    pub id: AVCodecID,
    /// Supported pixel format.
   ///
   /// Only hardware accelerated formats are supported here.
    pub pix_fmt: AVPixelFormat,
    /// Hardware accelerated codec capabilities.
   /// see AV_HWACCEL_CODEC_CAP_*
    pub capabilities: libc::c_int,
    /// Allocate a custom buffer
    pub alloc_frame: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, frame: *mut AVFrame) -> libc::c_int>,
    /// Called at the beginning of each frame or field picture.
   ///
   /// Meaningful frame information (codec specific) is guaranteed to
   /// be parsed at this point. This function is mandatory.
   ///
   /// Note that buf can be NULL along with buf_size set to 0.
   /// Otherwise, this means the whole frame is available at this point.
   ///
   /// @param avctx the codec context
   /// @param buf the frame common buffer base
   /// @param buf_size the size of the frame in bytes
   /// @return zero if successful, a negative value otherwise
    pub start_frame: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, buf: *const u8, buf_size: u32) -> libc::c_int>,
    /// Callback for parameter common (SPS/PPS/VPS etc).
   ///
   /// Useful for hardware decoders which keep persistent state about the
   /// video parameters, and need to receive any changes to update that state.
   ///
   /// @param avctx the codec context
   /// @param type the nal unit type
   /// @param buf the nal unit common buffer
   /// @param buf_size the size of the nal unit in bytes
   /// @return zero if successful, a negative value otherwise
    pub decode_params: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, type_: libc::c_int, buf: *const u8, buf_size: u32) -> libc::c_int>,
    /// Callback for each slice.
   ///
   /// Meaningful slice information (codec specific) is guaranteed to
   /// be parsed at this point. This function is mandatory.
   /// The only exception is XvMC, that works on MB level.
   ///
   /// @param avctx the codec context
   /// @param buf the slice common buffer base
   /// @param buf_size the size of the slice in bytes
   /// @return zero if successful, a negative value otherwise
    pub decode_slice: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, buf: *const u8, buf_size: u32) -> libc::c_int>,
    /// Called at the end of each frame or field picture.
   ///
   /// The whole picture is parsed at this point and can now be sent
   /// to the hardware accelerator. This function is mandatory.
   ///
   /// @param avctx the codec context
   /// @return zero if successful, a negative value otherwise
    pub end_frame: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext) -> libc::c_int>,
    /// Size of per-frame hardware accelerator private common.
   ///
   /// Private common is allocated with av_mallocz() before
   /// AVCodecContext.get_buffer() and deallocated after
   /// AVCodecContext.release_buffer().
    pub frame_priv_data_size: libc::c_int,
    /// Called for every Macroblock in a slice.
   ///
   /// XvMC uses it to replace the ff_mpv_reconstruct_mb().
   /// Instead of decoding to raw picture, MB parameters are
   /// stored in an array provided by the video driver.
   ///
   /// @param s the mpeg context
    pub decode_mb: ::std::option::Option<unsafe extern "C" fn(s: *mut MpegEncContext)>,
    /// Initialize the hwaccel private common.
   ///
   /// This will be called from ff_get_format(), after hwaccel and
   /// hwaccel_context are set and the hwaccel private common in AVCodecInternal
   /// is allocated.
    pub init: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext) -> libc::c_int>,
    /// Uninitialize the hwaccel private common.
   ///
   /// This will be called from get_format() or avcodec_close(), after hwaccel
   /// and hwaccel_context are already uninitialized.
    pub uninit: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext) -> libc::c_int>,
    /// Size of the private common to allocate in
   /// AVCodecInternal.hwaccel_priv_data.
    pub priv_data_size: libc::c_int,
    /// Internal hwaccel capabilities.
    pub caps_internal: libc::c_int,
    /// Fill the given hw_frames context with current codec parameters. Called
   /// from get_format. Refer to avcodec_get_hw_frames_parameters() for
   /// details.
   ///
   /// This CAN be called before AVHWAccel.init is called, and you must assume
   /// that avctx->hwaccel_priv_data is invalid.
    pub frame_params: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, hw_frames_ctx: *mut AVBufferRef) -> libc::c_int>,
}

/// This struct describes the properties of a single codec described by an
/// AVCodecID.
/// @see avcodec_descriptor_get()
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_: AVMediaType,
    /// Name of the codec described by this descriptor. It is non-empty and
   /// unique for each codec descriptor. It should contain alphanumeric
   /// characters and '_' only.
    pub name: *const libc::c_char,
    /// A more descriptive name for this codec. May be NULL.
    pub long_name: *const libc::c_char,
    /// Codec properties, a combination of AV_CODEC_PROP_* flags.
    pub props: libc::c_int,
    /// MIME type(s) associated with the codec.
   /// May be NULL; if not, a NULL-terminated array of MIME types.
   /// The first item is always non-NULL and is the preferred MIME type.
    pub mime_types: *const *const libc::c_char,
    /// If non-NULL, an array of profiles recognized for this codec.
   /// Terminated with FF_PROFILE_UNKNOWN.
    pub profiles: *const AVProfile,
}

/// main external API structure.
/// New fields can be added to the end with minor version bumps.
/// Removal, reordering and changes to existing fields require a major
/// version bump.
/// You can use AVOptions (av_opt* / av_set/get*()) to access these fields from user
/// applications.
/// The name string for AVOptions options matches the associated command line
/// parameter name and can be found in libavcodec/options_table.h
/// The AVOption/command line parameter names differ in some cases from the C
/// structure field names for historic reasons or brevity.
/// sizeof(AVCodecContext) must not be used outside libav*.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AVCodecContext {
    /// information on struct for av_log
   /// - set by avcodec_alloc_context3
    pub av_class: *const AVClass,
    pub log_level_offset: libc::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    /// fourcc (LSB first, so "ABCD" -> ('D'<<24) + ('C'<<16) + ('B'<<8) + 'A').
   /// This is used to work around some encoder bugs.
   /// A demuxer should set this to what is stored in the field used to identify the codec.
   /// If there are multiple such fields in a container then the demuxer should choose the one
   /// which maximizes the information about the used codec.
   /// If the codec tag field in a container is larger than 32 bits then the demuxer should
   /// remap the longer ID to 32 bits with a table or other structure. Alternatively a new
   /// extra_codec_tag + size could be added but for this a clear advantage must be demonstrated
   /// first.
   /// - encoding: Set by user, if not then the default based on codec_id will be used.
   /// - decoding: Set by user, will be converted to uppercase by libavcodec during init.
    pub codec_tag: libc::c_uint,
    pub priv_data: *mut libc::c_void,
    /// Private context used for internal common.
   ///
   /// Unlike priv_data, this is not codec-specific. It is used in general
   /// libavcodec functions.
    pub internal: *mut AVCodecInternal,
    /// Private common of the user, can be used to carry app specific stuff.
   /// - encoding: Set by user.
   /// - decoding: Set by user.
    pub opaque: *mut libc::c_void,
    /// the average bitrate
   /// - encoding: Set by user; unused for constant quantizer encoding.
   /// - decoding: Set by user, may be overwritten by libavcodec
   /// if this info is available in the stream
    pub bit_rate: i64,
    /// number of bits the bitstream is allowed to diverge from the reference.
   /// the reference can be CBR (for CBR pass1) or VBR (for pass2)
   /// - encoding: Set by user; unused for constant quantizer encoding.
   /// - decoding: unused
    pub bit_rate_tolerance: libc::c_int,
    /// Global quality for codecs which cannot change it per frame.
   /// This should be proportional to MPEG-1/2/4 qscale.
   /// - encoding: Set by user.
   /// - decoding: unused
    pub global_quality: libc::c_int,
    /// - encoding: Set by user.
   /// - decoding: unused
    pub compression_level: libc::c_int,
    /// AV_CODEC_FLAG_*.
   /// - encoding: Set by user.
   /// - decoding: Set by user.
    pub flags: libc::c_int,
    /// AV_CODEC_FLAG2_*
   /// - encoding: Set by user.
   /// - decoding: Set by user.
    pub flags2: libc::c_int,
    /// some codecs need / can use extradata like Huffman tables.
   /// MJPEG: Huffman tables
   /// rv10: additional flags
   /// MPEG-4: global headers (they can be in the bitstream or here)
   /// The allocated memory should be AV_INPUT_BUFFER_PADDING_SIZE bytes larger
   /// than extradata_size to avoid problems if it is read with the bitstream reader.
   /// The bytewise contents of extradata must not depend on the architecture or CPU endianness.
   /// Must be allocated with the av_malloc() family of functions.
   /// - encoding: Set/allocated/freed by libavcodec.
   /// - decoding: Set/allocated/freed by user.
    pub extradata: *mut u8,
    pub extradata_size: libc::c_int,
    /// This is the fundamental unit of time (in seconds) in terms
   /// of which frame timestamps are represented. For fixed-fps content,
   /// timebase should be 1/framerate and timestamp increments should be
   /// identically 1.
   /// This often, but not always is the inverse of the frame rate or field rate
   /// for video. 1/time_base is not the average frame rate if the frame rate is not
   /// constant.
   ///
   /// Like containers, elementary streams also can store timestamps, 1/time_base
   /// is the unit in which these timestamps are specified.
   /// As example of such codec time base see ISO/IEC 14496-2:2001(E)
   /// vop_time_increment_resolution and fixed_vop_rate
   /// (fixed_vop_rate == 0 implies that it is different from the framerate)
   ///
   /// - encoding: MUST be set by user.
   /// - decoding: the use of this field for decoding is deprecated.
   /// Use framerate instead.
    pub time_base: AVRational,
    /// For some codecs, the time base is closer to the field rate than the frame rate.
   /// Most notably, H.264 and MPEG-2 specify time_base as half of frame duration
   /// if no telecine is used ...
   ///
   /// Set to time_base ticks per frame. Default 1, e.g., H.264/MPEG-2 set it to 2.
    pub ticks_per_frame: libc::c_int,
    /// Codec delay.
   ///
   /// Encoding: Number of frames delay there will be from the encoder input to
   /// the decoder output. (we assume the decoder matches the spec)
   /// Decoding: Number of frames delay in addition to what a standard decoder
   /// as specified in the spec would produce.
   ///
   /// Video:
   /// Number of frames the decoded output will be delayed relative to the
   /// encoded input.
   ///
   /// Audio:
   /// For encoding, this field is unused (see initial_padding).
   ///
   /// For decoding, this is the number of samples the decoder needs to
   /// output before the decoder's output is valid. When seeking, you should
   /// start decoding this many samples prior to your desired seek point.
   ///
   /// - encoding: Set by libavcodec.
   /// - decoding: Set by libavcodec.
    pub delay: libc::c_int,
    /// picture width / height.
   ///
   /// @note Those fields may not match the values of the last
   /// AVFrame output by avcodec_decode_video2 due frame
   /// reordering.
   ///
   /// - encoding: MUST be set by user.
   /// - decoding: May be set by the user before opening the decoder if known e.g.
   /// from the container. Some decoders will require the dimensions
   /// to be set by the caller. During decoding, the decoder may
   /// overwrite those values as required while parsing the common.
    pub width: libc::c_int,
    /// picture width / height.
   ///
   /// @note Those fields may not match the values of the last
   /// AVFrame output by avcodec_decode_video2 due frame
   /// reordering.
   ///
   /// - encoding: MUST be set by user.
   /// - decoding: May be set by the user before opening the decoder if known e.g.
   /// from the container. Some decoders will require the dimensions
   /// to be set by the caller. During decoding, the decoder may
   /// overwrite those values as required while parsing the common.
    pub height: libc::c_int,
    /// Bitstream width / height, may be different from width/height e.g. when
   /// the decoded frame is cropped before being output or lowres is enabled.
   ///
   /// @note Those field may not match the value of the last
   /// AVFrame output by avcodec_receive_frame() due frame
   /// reordering.
   ///
   /// - encoding: unused
   /// - decoding: May be set by the user before opening the decoder if known
   /// e.g. from the container. During decoding, the decoder may
   /// overwrite those values as required while parsing the common.
    pub coded_width: libc::c_int,
    /// Bitstream width / height, may be different from width/height e.g. when
   /// the decoded frame is cropped before being output or lowres is enabled.
   ///
   /// @note Those field may not match the value of the last
   /// AVFrame output by avcodec_receive_frame() due frame
   /// reordering.
   ///
   /// - encoding: unused
   /// - decoding: May be set by the user before opening the decoder if known
   /// e.g. from the container. During decoding, the decoder may
   /// overwrite those values as required while parsing the common.
    pub coded_height: libc::c_int,
    /// the number of pictures in a group of pictures, or 0 for intra_only
   /// - encoding: Set by user.
   /// - decoding: unused
    pub gop_size: libc::c_int,
    /// Pixel format, see AV_PIX_FMT_xxx.
   /// May be set by the demuxer if known from headers.
   /// May be overridden by the decoder if it knows better.
   ///
   /// @note This field may not match the value of the last
   /// AVFrame output by avcodec_receive_frame() due frame
   /// reordering.
   ///
   /// - encoding: Set by user.
   /// - decoding: Set by user if known, overridden by libavcodec while
   /// parsing the common.
    pub pix_fmt: AVPixelFormat,
    /// If non NULL, 'draw_horiz_band' is called by the libavcodec
   /// decoder to draw a horizontal band. It improves cache usage. Not
   /// all codecs can do that. You must check the codec capabilities
   /// beforehand.
   /// When multithreading is used, it may be called from multiple threads
   /// at the same time; threads might draw different parts of the same AVFrame,
   /// or multiple AVFrames, and there is no guarantee that slices will be drawn
   /// in order.
   /// The function is also used by hardware acceleration APIs.
   /// It is called at least once during frame decoding to pass
   /// the common needed for hardware render.
   /// In that mode instead of pixel common, AVFrame points to
   /// a structure specific to the acceleration API. The application
   /// reads the structure and can change some fields to indicate progress
   /// or mark state.
   /// - encoding: unused
   /// - decoding: Set by user.
   /// @param height the height of the slice
   /// @param y the y position of the slice
   /// @param type 1->top field, 2->bottom field, 3->frame
   /// @param offset offset into the AVFrame.common from which the slice should be read
    pub draw_horiz_band: ::std::option::Option<unsafe extern "C" fn(s: *mut AVCodecContext, src: *const AVFrame, offset: *mut libc::c_int, y: libc::c_int, type_: libc::c_int, height: libc::c_int)>,
    /// callback to negotiate the pixelFormat
   /// @param fmt is the list of formats which are supported by the codec,
   /// it is terminated by -1 as 0 is a valid format, the formats are ordered by quality.
   /// The first is always the native one.
   /// @note The callback may be called again immediately if initialization for
   /// the selected (hardware-accelerated) pixel format failed.
   /// @warning Behavior is undefined if the callback returns a value not
   /// in the fmt list of formats.
   /// @return the chosen format
   /// - encoding: unused
   /// - decoding: Set by user, if not set the native format will be chosen.
    pub get_format: ::std::option::Option<unsafe extern "C" fn(s: *mut AVCodecContext, fmt: *const AVPixelFormat) -> AVPixelFormat>,
    /// maximum number of B-frames between non-B-frames
   /// Note: The output will be delayed by max_b_frames+1 relative to the input.
   /// - encoding: Set by user.
   /// - decoding: unused
    pub max_b_frames: libc::c_int,
    /// qscale factor between IP and B-frames
   /// If > 0 then the last P-frame quantizer will be used (q= lastp_q*factor+offset).
   /// If < 0 then normal ratecontrol will be done (q= -normal_q*factor+offset).
   /// - encoding: Set by user.
   /// - decoding: unused
    pub b_quant_factor: f32,
    /// @deprecated use encoder private options instead
    pub b_frame_strategy: libc::c_int,
    /// qscale offset between IP and B-frames
   /// - encoding: Set by user.
   /// - decoding: unused
    pub b_quant_offset: f32,
    /// Size of the frame reordering buffer in the decoder.
   /// For MPEG-2 it is 1 IPB or 0 low delay IP.
   /// - encoding: Set by libavcodec.
   /// - decoding: Set by libavcodec.
    pub has_b_frames: libc::c_int,
    /// @deprecated use encoder private options instead
    pub mpeg_quant: libc::c_int,
    /// qscale factor between P- and I-frames
   /// If > 0 then the last P-frame quantizer will be used (q = lastp_q * factor + offset).
   /// If < 0 then normal ratecontrol will be done (q= -normal_q*factor+offset).
   /// - encoding: Set by user.
   /// - decoding: unused
    pub i_quant_factor: f32,
    /// qscale offset between P and I-frames
   /// - encoding: Set by user.
   /// - decoding: unused
    pub i_quant_offset: f32,
    /// luminance masking (0-> disabled)
   /// - encoding: Set by user.
   /// - decoding: unused
    pub lumi_masking: f32,
    /// temporary complexity masking (0-> disabled)
   /// - encoding: Set by user.
   /// - decoding: unused
    pub temporal_cplx_masking: f32,
    /// spatial complexity masking (0-> disabled)
   /// - encoding: Set by user.
   /// - decoding: unused
    pub spatial_cplx_masking: f32,
    /// p block masking (0-> disabled)
   /// - encoding: Set by user.
   /// - decoding: unused
    pub p_masking: f32,
    /// darkness masking (0-> disabled)
   /// - encoding: Set by user.
   /// - decoding: unused
    pub dark_masking: f32,
    /// slice count
   /// - encoding: Set by libavcodec.
   /// - decoding: Set by user (or 0).
    pub slice_count: libc::c_int,
    /// @deprecated use encoder private options instead
    pub prediction_method: libc::c_int,
    /// slice offsets in the frame in bytes
   /// - encoding: Set/allocated by libavcodec.
   /// - decoding: Set/allocated by user (or NULL).
    pub slice_offset: *mut libc::c_int,
    /// sample aspect ratio (0 if unknown)
   /// That is the width of a pixel divided by the height of the pixel.
   /// Numerator and denominator must be relatively prime and smaller than 256 for some video standards.
   /// - encoding: Set by user.
   /// - decoding: Set by libavcodec.
    pub sample_aspect_ratio: AVRational,
    /// motion estimation comparison function
   /// - encoding: Set by user.
   /// - decoding: unused
    pub me_cmp: libc::c_int,
    /// subpixel motion estimation comparison function
   /// - encoding: Set by user.
   /// - decoding: unused
    pub me_sub_cmp: libc::c_int,
    /// macroblock comparison function (not supported yet)
   /// - encoding: Set by user.
   /// - decoding: unused
    pub mb_cmp: libc::c_int,
    /// interlaced DCT comparison function
   /// - encoding: Set by user.
   /// - decoding: unused
    pub ildct_cmp: libc::c_int,
    /// ME diamond size & shape
   /// - encoding: Set by user.
   /// - decoding: unused
    pub dia_size: libc::c_int,
    /// amount of previous MV predictors (2a+1 x 2a+1 square)
   /// - encoding: Set by user.
   /// - decoding: unused
    pub last_predictor_count: libc::c_int,
    /// @deprecated use encoder private options instead
    pub pre_me: libc::c_int,
    /// motion estimation prepass comparison function
   /// - encoding: Set by user.
   /// - decoding: unused
    pub me_pre_cmp: libc::c_int,
    /// ME prepass diamond size & shape
   /// - encoding: Set by user.
   /// - decoding: unused
    pub pre_dia_size: libc::c_int,
    /// subpel ME quality
   /// - encoding: Set by user.
   /// - decoding: unused
    pub me_subpel_quality: libc::c_int,
    /// maximum motion estimation search range in subpel units
   /// If 0 then no limit.
   ///
   /// - encoding: Set by user.
   /// - decoding: unused
    pub me_range: libc::c_int,
    /// slice flags
   /// - encoding: unused
   /// - decoding: Set by user.
    pub slice_flags: libc::c_int,
    /// macroblock decision mode
   /// - encoding: Set by user.
   /// - decoding: unused
    pub mb_decision: libc::c_int,
    /// custom intra quantization matrix
   /// - encoding: Set by user, can be NULL.
   /// - decoding: Set by libavcodec.
    pub intra_matrix: *mut u16,
    /// custom inter quantization matrix
   /// - encoding: Set by user, can be NULL.
   /// - decoding: Set by libavcodec.
    pub inter_matrix: *mut u16,
    /// @deprecated use encoder private options instead
    pub scenechange_threshold: libc::c_int,
    /// @deprecated use encoder private options instead
    pub noise_reduction: libc::c_int,
    /// precision of the intra DC coefficient - 8
   /// - encoding: Set by user.
   /// - decoding: Set by libavcodec
    pub intra_dc_precision: libc::c_int,
    /// Number of macroblock rows at the top which are skipped.
   /// - encoding: unused
   /// - decoding: Set by user.
    pub skip_top: libc::c_int,
    /// Number of macroblock rows at the bottom which are skipped.
   /// - encoding: unused
   /// - decoding: Set by user.
    pub skip_bottom: libc::c_int,
    /// minimum MB Lagrange multiplier
   /// - encoding: Set by user.
   /// - decoding: unused
    pub mb_lmin: libc::c_int,
    /// maximum MB Lagrange multiplier
   /// - encoding: Set by user.
   /// - decoding: unused
    pub mb_lmax: libc::c_int,
    /// @deprecated use encoder private options instead
    pub me_penalty_compensation: libc::c_int,
    /// - encoding: Set by user.
   /// - decoding: unused
    pub bidir_refine: libc::c_int,
    /// @deprecated use encoder private options instead
    pub brd_scale: libc::c_int,
    /// minimum GOP size
   /// - encoding: Set by user.
   /// - decoding: unused
    pub keyint_min: libc::c_int,
    /// number of reference frames
   /// - encoding: Set by user.
   /// - decoding: Set by lavc.
    pub refs: libc::c_int,
    /// @deprecated use encoder private options instead
    pub chromaoffset: libc::c_int,
    /// Note: Value depends upon the compare function used for fullpel ME.
   /// - encoding: Set by user.
   /// - decoding: unused
    pub mv0_threshold: libc::c_int,
    /// @deprecated use encoder private options instead
    pub b_sensitivity: libc::c_int,
    /// Chromaticity coordinates of the source primaries.
   /// - encoding: Set by user
   /// - decoding: Set by libavcodec
    pub color_primaries: AVColorPrimaries,
    /// Color Transfer Characteristic.
   /// - encoding: Set by user
   /// - decoding: Set by libavcodec
    pub color_trc: AVColorTransferCharacteristic,
    /// YUV colorspace type.
   /// - encoding: Set by user
   /// - decoding: Set by libavcodec
    pub colorspace: AVColorSpace,
    /// MPEG vs JPEG YUV range.
   /// - encoding: Set by user
   /// - decoding: Set by libavcodec
    pub color_range: AVColorRange,
    /// This defines the location of chroma samples.
   /// - encoding: Set by user
   /// - decoding: Set by libavcodec
    pub chroma_sample_location: AVChromaLocation,
    /// Number of slices.
   /// Indicates number of picture subdivisions. Used for parallelized
   /// decoding.
   /// - encoding: Set by user
   /// - decoding: unused
    pub slices: libc::c_int,
    /// Field order
   /// - encoding: set by libavcodec
   /// - decoding: Set by user.
    pub field_order: AVFieldOrder,
    /// < samples per second
    pub sample_rate: libc::c_int,
    /// < number of audio channels
    pub channels: libc::c_int,
    /// < sample format
    pub sample_fmt: AVSampleFormat,
    /// Number of samples per channel in an audio frame.
   ///
   /// - encoding: set by libavcodec in avcodec_open2(). Each submitted frame
   /// except the last must contain exactly frame_size samples per channel.
   /// May be 0 when the codec has AV_CODEC_CAP_VARIABLE_FRAME_SIZE set, then the
   /// frame size is not restricted.
   /// - decoding: may be set by some decoders to indicate constant frame size
    pub frame_size: libc::c_int,
    /// Frame counter, set by libavcodec.
   ///
   /// - decoding: total number of frames returned from the decoder so far.
   /// - encoding: total number of frames passed to the encoder so far.
   ///
   /// @note the counter is not incremented if encoding/decoding resulted in
   /// an error.
    pub frame_number: libc::c_int,
    /// number of bytes per packet if constant and known or 0
   /// Used by some WAV based audio codecs.
    pub block_align: libc::c_int,
    /// Audio cutoff bandwidth (0 means "automatic")
   /// - encoding: Set by user.
   /// - decoding: unused
    pub cutoff: libc::c_int,
    /// Audio channel layout.
   /// - encoding: set by user.
   /// - decoding: set by user, may be overwritten by libavcodec.
    pub channel_layout: u64,
    /// Request decoder to use this channel layout if it can (0 for default)
   /// - encoding: unused
   /// - decoding: Set by user.
    pub request_channel_layout: u64,
    /// Type of service that the audio stream conveys.
   /// - encoding: Set by user.
   /// - decoding: Set by libavcodec.
    pub audio_service_type: AVAudioServiceType,
    /// desired sample format
   /// - encoding: Not used.
   /// - decoding: Set by user.
   /// Decoder will decode to this format if it can.
    pub request_sample_fmt: AVSampleFormat,
    /// This callback is called at the beginning of each frame to get common
   /// buffer(s) for it. There may be one contiguous buffer for all the common or
   /// there may be a buffer per each common plane or anything in between. What
   /// this means is, you may set however many entries in buf[] you feel necessary.
   /// Each buffer must be reference-counted using the AVBuffer API (see description
   /// of buf[] below).
   ///
   /// The following fields will be set in the frame before this callback is
   /// called:
   /// - format
   /// - width, height (video only)
   /// - sample_rate, channel_layout, nb_samples (audio only)
   /// Their values may differ from the corresponding values in
   /// AVCodecContext. This callback must use the frame values, not the codec
   /// context values, to calculate the required buffer size.
   ///
   /// This callback must fill the following fields in the frame:
   /// - common[]
   /// - linesize[]
   /// - extended_data:
   /// * if the common is planar audio with more than 8 channels, then this
   /// callback must allocate and fill extended_data to contain all pointers
   /// to all common planes. common[] must hold as many pointers as it can.
   /// extended_data must be allocated with av_malloc() and will be freed in
   /// av_frame_unref().
   /// * otherwise extended_data must point to common
   /// - buf[] must contain one or more pointers to AVBufferRef structures. Each of
   /// the frame's common and extended_data pointers must be contained in these. That
   /// is, one AVBufferRef for each allocated chunk of memory, not necessarily one
   /// AVBufferRef per common[] entry. See: av_buffer_create(), av_buffer_alloc(),
   /// and av_buffer_ref().
   /// - extended_buf and nb_extended_buf must be allocated with av_malloc() by
   /// this callback and filled with the extra buffers if there are more
   /// buffers than buf[] can hold. extended_buf will be freed in
   /// av_frame_unref().
   ///
   /// If AV_CODEC_CAP_DR1 is not set then get_buffer2() must call
   /// avcodec_default_get_buffer2() instead of providing buffers allocated by
   /// some other means.
   ///
   /// Each common plane must be aligned to the maximum required by the target
   /// CPU.
   ///
   /// @see avcodec_default_get_buffer2()
   ///
   /// Video:
   ///
   /// If AV_GET_BUFFER_FLAG_REF is set in flags then the frame may be reused
   /// (read and/or written to if it is writable) later by libavcodec.
   ///
   /// avcodec_align_dimensions2() should be used to find the required width and
   /// height, as they normally need to be rounded up to the next multiple of 16.
   ///
   /// Some decoders do not support linesizes changing between frames.
   ///
   /// If frame multithreading is used and thread_safe_callbacks is set,
   /// this callback may be called from a different thread, but not from more
   /// than one at once. Does not need to be reentrant.
   ///
   /// @see avcodec_align_dimensions2()
   ///
   /// Audio:
   ///
   /// Decoders request a buffer of a particular size by setting
   /// AVFrame.nb_samples prior to calling get_buffer2(). The decoder may,
   /// however, utilize only part of the buffer by setting AVFrame.nb_samples
   /// to a smaller value in the output frame.
   ///
   /// As a convenience, av_samples_get_buffer_size() and
   /// av_samples_fill_arrays() in libavutil may be used by custom get_buffer2()
   /// functions to find the required common size and to fill common pointers and
   /// linesize. In AVFrame.linesize, only linesize[0] may be set for audio
   /// since all planes must be the same size.
   ///
   /// @see av_samples_get_buffer_size(), av_samples_fill_arrays()
   ///
   /// - encoding: unused
   /// - decoding: Set by libavcodec, user can override.
    pub get_buffer2: ::std::option::Option<unsafe extern "C" fn(s: *mut AVCodecContext, frame: *mut AVFrame, flags: libc::c_int) -> libc::c_int>,
    /// If non-zero, the decoded audio and video frames returned from
   /// avcodec_decode_video2() and avcodec_decode_audio4() are reference-counted
   /// and are valid indefinitely. The caller must free them with
   /// av_frame_unref() when they are not needed anymore.
   /// Otherwise, the decoded frames must not be freed by the caller and are
   /// only valid until the next decode call.
   ///
   /// This is always automatically enabled if avcodec_receive_frame() is used.
   ///
   /// - encoding: unused
   /// - decoding: set by the caller before avcodec_open2().
    pub refcounted_frames: libc::c_int,
    /// < amount of qscale change between easy & hard scenes (0.0-1.0)
    pub qcompress: f32,
    /// < amount of qscale smoothing over time (0.0-1.0)
    pub qblur: f32,
    /// minimum quantizer
   /// - encoding: Set by user.
   /// - decoding: unused
    pub qmin: libc::c_int,
    /// maximum quantizer
   /// - encoding: Set by user.
   /// - decoding: unused
    pub qmax: libc::c_int,
    /// maximum quantizer difference between frames
   /// - encoding: Set by user.
   /// - decoding: unused
    pub max_qdiff: libc::c_int,
    /// decoder bitstream buffer size
   /// - encoding: Set by user.
   /// - decoding: unused
    pub rc_buffer_size: libc::c_int,
    /// ratecontrol override, see RcOverride
   /// - encoding: Allocated/set/freed by user.
   /// - decoding: unused
    pub rc_override_count: libc::c_int,
    pub rc_override: *mut RcOverride,
    /// maximum bitrate
   /// - encoding: Set by user.
   /// - decoding: Set by user, may be overwritten by libavcodec.
    pub rc_max_rate: i64,
    /// minimum bitrate
   /// - encoding: Set by user.
   /// - decoding: unused
    pub rc_min_rate: i64,
    /// Ratecontrol attempt to use, at maximum, <value> of what can be used without an underflow.
   /// - encoding: Set by user.
   /// - decoding: unused.
    pub rc_max_available_vbv_use: f32,
    /// Ratecontrol attempt to use, at least, <value> times the amount needed to prevent a vbv overflow.
   /// - encoding: Set by user.
   /// - decoding: unused.
    pub rc_min_vbv_overflow_use: f32,
    /// Number of bits which should be loaded into the rc buffer before decoding starts.
   /// - encoding: Set by user.
   /// - decoding: unused
    pub rc_initial_buffer_occupancy: libc::c_int,
    /// @deprecated use encoder private options instead
    pub coder_type: libc::c_int,
    /// @deprecated use encoder private options instead
    pub context_model: libc::c_int,
    /// @deprecated use encoder private options instead
    pub frame_skip_threshold: libc::c_int,
    /// @deprecated use encoder private options instead
    pub frame_skip_factor: libc::c_int,
    /// @deprecated use encoder private options instead
    pub frame_skip_exp: libc::c_int,
    /// @deprecated use encoder private options instead
    pub frame_skip_cmp: libc::c_int,
    /// trellis RD quantization
   /// - encoding: Set by user.
   /// - decoding: unused
    pub trellis: libc::c_int,
    /// @deprecated use encoder private options instead
    pub min_prediction_order: libc::c_int,
    /// @deprecated use encoder private options instead
    pub max_prediction_order: libc::c_int,
    /// @deprecated use encoder private options instead
    pub timecode_frame_start: i64,
    /// @deprecated unused
    pub rtp_callback: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, data: *mut libc::c_void, size: libc::c_int, mb_nb: libc::c_int)>,
    /// @deprecated use encoder private options instead
    pub rtp_payload_size: libc::c_int,
    pub mv_bits: libc::c_int,
    pub header_bits: libc::c_int,
    pub i_tex_bits: libc::c_int,
    pub p_tex_bits: libc::c_int,
    pub i_count: libc::c_int,
    pub p_count: libc::c_int,
    pub skip_count: libc::c_int,
    pub misc_bits: libc::c_int,
    /// @deprecated this field is unused
    pub frame_bits: libc::c_int,
    /// pass1 encoding statistics output buffer
   /// - encoding: Set by libavcodec.
   /// - decoding: unused
    pub stats_out: *mut libc::c_char,
    /// pass2 encoding statistics input buffer
   /// Concatenated stuff from stats_out of pass1 should be placed here.
   /// - encoding: Allocated/set/freed by user.
   /// - decoding: unused
    pub stats_in: *mut libc::c_char,
    /// Work around bugs in encoders which sometimes cannot be detected automatically.
   /// - encoding: Set by user
   /// - decoding: Set by user
    pub workaround_bugs: libc::c_int,
    /// strictly follow the standard (MPEG-4, ...).
   /// - encoding: Set by user.
   /// - decoding: Set by user.
   /// Setting this to STRICT or higher means the encoder and decoder will
   /// generally do stupid things, whereas setting it to unofficial or lower
   /// will mean the encoder might produce output that is not supported by all
   /// spec-compliant decoders. Decoders don't differentiate between normal,
   /// unofficial and experimental (that is, they always try to decode things
   /// when they can) unless they are explicitly asked to behave stupidly
   /// (=strictly conform to the specs)
    pub strict_std_compliance: libc::c_int,
    /// error concealment flags
   /// - encoding: unused
   /// - decoding: Set by user.
    pub error_concealment: libc::c_int,
    /// debug
   /// - encoding: Set by user.
   /// - decoding: Set by user.
    pub debug: libc::c_int,
    /// Error recognition; may misdetect some more or less valid parts as errors.
   /// - encoding: unused
   /// - decoding: Set by user.
    pub err_recognition: libc::c_int,
    /// opaque 64-bit number (generally a PTS) that will be reordered and
   /// output in AVFrame.reordered_opaque
   /// - encoding: unused
   /// - decoding: Set by user.
    pub reordered_opaque: i64,
    /// Hardware accelerator in use
   /// - encoding: unused.
   /// - decoding: Set by libavcodec
    pub hwaccel: *const AVHWAccel,
    /// Hardware accelerator context.
   /// For some hardware accelerators, a global context needs to be
   /// provided by the user. In that case, this holds display-dependent
   /// common FFmpeg cannot instantiate itself. Please refer to the
   /// FFmpeg HW accelerator documentation to know how to fill this
   /// is. e.g. for VA API, this is a struct vaapi_context.
   /// - encoding: unused
   /// - decoding: Set by user
    pub hwaccel_context: *mut libc::c_void,
    /// error
   /// - encoding: Set by libavcodec if flags & AV_CODEC_FLAG_PSNR.
   /// - decoding: unused
    pub error: [u64; 8usize],
    /// DCT algorithm, see FF_DCT_* below
   /// - encoding: Set by user.
   /// - decoding: unused
    pub dct_algo: libc::c_int,
    /// IDCT algorithm, see FF_IDCT_* below.
   /// - encoding: Set by user.
   /// - decoding: Set by user.
    pub idct_algo: libc::c_int,
    /// bits per sample/pixel from the demuxer (needed for huffyuv).
   /// - encoding: Set by libavcodec.
   /// - decoding: Set by user.
    pub bits_per_coded_sample: libc::c_int,
    /// Bits per sample/pixel of internal libavcodec pixel/sample format.
   /// - encoding: set by user.
   /// - decoding: set by libavcodec.
    pub bits_per_raw_sample: libc::c_int,
    /// low resolution decoding, 1-> 1/2 size, 2->1/4 size
   /// - encoding: unused
   /// - decoding: Set by user.
    pub lowres: libc::c_int,
    /// the picture in the bitstream
   /// - encoding: Set by libavcodec.
   /// - decoding: unused
   ///
   /// @deprecated use the quality factor packet side common instead
    pub coded_frame: *mut AVFrame,
    /// thread count
   /// is used to decide how many independent tasks should be passed to execute()
   /// - encoding: Set by user.
   /// - decoding: Set by user.
    pub thread_count: libc::c_int,
    /// Which multithreading methods to use.
   /// Use of FF_THREAD_FRAME will increase decoding delay by one frame per thread,
   /// so clients which cannot provide future frames should not use it.
   ///
   /// - encoding: Set by user, otherwise the default is used.
   /// - decoding: Set by user, otherwise the default is used.
    pub thread_type: libc::c_int,
    /// Which multithreading methods are in use by the codec.
   /// - encoding: Set by libavcodec.
   /// - decoding: Set by libavcodec.
    pub active_thread_type: libc::c_int,
    /// Set by the client if its custom get_buffer() callback can be called
   /// synchronously from another thread, which allows faster multithreaded decoding.
   /// draw_horiz_band() will be called from other threads regardless of this setting.
   /// Ignored if the default get_buffer() is used.
   /// - encoding: Set by user.
   /// - decoding: Set by user.
    pub thread_safe_callbacks: libc::c_int,
    /// The codec may call this to execute several independent things.
   /// It will return only after finishing all tasks.
   /// The user may replace this with some multithreaded implementation,
   /// the default implementation will execute the parts serially.
   /// @param count the number of things to execute
   /// - encoding: Set by libavcodec, user can override.
   /// - decoding: Set by libavcodec, user can override.
    pub execute: ::std::option::Option<unsafe extern "C" fn(c: *mut AVCodecContext, func: ::std::option::Option<unsafe extern "C" fn(c2: *mut AVCodecContext, arg: *mut libc::c_void) -> libc::c_int>, arg2: *mut libc::c_void, ret: *mut libc::c_int, count: libc::c_int, size: libc::c_int) -> libc::c_int>,
    /// The codec may call this to execute several independent things.
   /// It will return only after finishing all tasks.
   /// The user may replace this with some multithreaded implementation,
   /// the default implementation will execute the parts serially.
   /// Also see avcodec_thread_init and e.g. the --enable-pthread configure option.
   /// @param c context passed also to func
   /// @param count the number of things to execute
   /// @param arg2 argument passed unchanged to func
   /// @param ret return values of executed functions, must have space for "count" values. May be NULL.
   /// @param func function that will be called count times, with jobnr from 0 to count-1.
   /// threadnr will be in the range 0 to c->thread_count-1 < MAX_THREADS and so that no
   /// two instances of func executing at the same time will have the same threadnr.
   /// @return always 0 currently, but code should handle a future improvement where when any call to func
   /// returns < 0 no further calls to func may be done and < 0 is returned.
   /// - encoding: Set by libavcodec, user can override.
   /// - decoding: Set by libavcodec, user can override.
    pub execute2: ::std::option::Option<unsafe extern "C" fn(c: *mut AVCodecContext, func: ::std::option::Option<unsafe extern "C" fn(c2: *mut AVCodecContext, arg: *mut libc::c_void, jobnr: libc::c_int, threadnr: libc::c_int) -> libc::c_int>, arg2: *mut libc::c_void, ret: *mut libc::c_int, count: libc::c_int) -> libc::c_int>,
    /// noise vs. sse weight for the nsse comparison function
   /// - encoding: Set by user.
   /// - decoding: unused
    pub nsse_weight: libc::c_int,
    /// profile
   /// - encoding: Set by user.
   /// - decoding: Set by libavcodec.
    pub profile: libc::c_int,
    /// level
   /// - encoding: Set by user.
   /// - decoding: Set by libavcodec.
    pub level: libc::c_int,
    /// Skip loop filtering for selected frames.
   /// - encoding: unused
   /// - decoding: Set by user.
    pub skip_loop_filter: AVDiscard,
    /// Skip IDCT/dequantization for selected frames.
   /// - encoding: unused
   /// - decoding: Set by user.
    pub skip_idct: AVDiscard,
    /// Skip decoding for selected frames.
   /// - encoding: unused
   /// - decoding: Set by user.
    pub skip_frame: AVDiscard,
    /// Header containing style information for text subtitles.
   /// For SUBTITLE_ASS subtitle type, it should contain the whole ASS
   /// [Script Info] and [V4+ Styles] section, plus the [Events] line and
   /// the Format line following. It shouldn't include any Dialogue line.
   /// - encoding: Set/allocated/freed by user (before avcodec_open2())
   /// - decoding: Set/allocated/freed by libavcodec (by avcodec_open2())
    pub subtitle_header: *mut u8,
    pub subtitle_header_size: libc::c_int,
    /// VBV delay coded in the last frame (in periods of a 27 MHz clock).
   /// Used for compliant TS muxing.
   /// - encoding: Set by libavcodec.
   /// - decoding: unused.
   /// @deprecated this value is now exported as a part of
   /// AV_PKT_DATA_CPB_PROPERTIES packet side common
    pub vbv_delay: u64,
    /// Encoding only and set by default. Allow encoders to output packets
   /// that do not contain any encoded common, only side common.
   ///
   /// Some encoders need to output such packets, e.g. to update some stream
   /// parameters at the end of encoding.
   ///
   /// @deprecated this field disables the default behaviour and
   /// it is kept only for compatibility.
    pub side_data_only_packets: libc::c_int,
    /// Audio only. The number of "priming" samples (padding) inserted by the
   /// encoder at the beginning of the audio. I.e. this number of leading
   /// decoded samples must be discarded by the caller to get the original audio
   /// without leading padding.
   ///
   /// - decoding: unused
   /// - encoding: Set by libavcodec. The timestamps on the output packets are
   /// adjusted by the encoder so that they always refer to the
   /// first sample of the common actually contained in the packet,
   /// including any added padding.  E.g. if the timebase is
   /// 1/samplerate and the timestamp of the first input sample is
   /// 0, the timestamp of the first output packet will be
   /// -initial_padding.
    pub initial_padding: libc::c_int,
    /// - decoding: For codecs that store a framerate value in the compressed
   /// bitstream, the decoder may export it here. { 0, 1} when
   /// unknown.
   /// - encoding: May be used to signal the framerate of CFR content to an
   /// encoder.
    pub framerate: AVRational,
    /// Nominal unaccelerated pixel format, see AV_PIX_FMT_xxx.
   /// - encoding: unused.
   /// - decoding: Set by libavcodec before calling get_format()
    pub sw_pix_fmt: AVPixelFormat,
    /// Timebase in which pkt_dts/pts and AVPacket.dts/pts are.
   /// - encoding unused.
   /// - decoding set by user.
    pub pkt_timebase: AVRational,
    /// AVCodecDescriptor
   /// - encoding: unused.
   /// - decoding: set by libavcodec.
    pub codec_descriptor: *const AVCodecDescriptor,
    /// Current statistics for PTS correction.
   /// - decoding: maintained and used by libavcodec, not intended to be used by user apps
   /// - encoding: unused
    pub pts_correction_num_faulty_pts: i64,
    /// Number of incorrect PTS values so far
    pub pts_correction_num_faulty_dts: i64,
    /// Number of incorrect DTS values so far
    pub pts_correction_last_pts: i64,
    /// PTS of the last frame
    pub pts_correction_last_dts: i64,
    /// Character encoding of the input subtitles file.
   /// - decoding: set by user
   /// - encoding: unused
    pub sub_charenc: *mut libc::c_char,
    /// Subtitles character encoding mode. Formats or codecs might be adjusting
   /// this setting (if they are doing the conversion themselves for instance).
   /// - decoding: set by libavcodec
   /// - encoding: unused
    pub sub_charenc_mode: libc::c_int,
    /// Skip processing alpha if supported by codec.
   /// Note that if the format uses pre-multiplied alpha (common with VP6,
   /// and recommended due to better video quality/compression)
   /// the image will look as if alpha-blended onto a black background.
   /// However for formats that do not use pre-multiplied alpha
   /// there might be serious artefacts (though e.g. libswscale currently
   /// assumes pre-multiplied alpha anyway).
   ///
   /// - decoding: set by user
   /// - encoding: unused
    pub skip_alpha: libc::c_int,
    /// Number of samples to skip after a discontinuity
   /// - decoding: unused
   /// - encoding: set by libavcodec
    pub seek_preroll: libc::c_int,
    /// debug motion vectors
   /// - encoding: Set by user.
   /// - decoding: Set by user.
    pub debug_mv: libc::c_int,
    /// custom intra quantization matrix
   /// - encoding: Set by user, can be NULL.
   /// - decoding: unused.
    pub chroma_intra_matrix: *mut u16,
    /// dump format separator.
   /// can be ", " or "\n      " or anything else
   /// - encoding: Set by user.
   /// - decoding: Set by user.
    pub dump_separator: *mut u8,
    /// ',' separated list of allowed decoders.
   /// If NULL then all are allowed
   /// - encoding: unused
   /// - decoding: set by user
    pub codec_whitelist: *mut libc::c_char,
    /// Properties of the stream that gets decoded
   /// - encoding: unused
   /// - decoding: set by libavcodec
    pub properties: libc::c_uint,
    /// Additional common associated with the entire coded stream.
   ///
   /// - decoding: unused
   /// - encoding: may be set by libavcodec after avcodec_open2().
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: libc::c_int,
    /// A reference to the AVHWFramesContext describing the input (for encoding)
   /// or output (decoding) frames. The reference is set by the caller and
   /// afterwards owned (and freed) by libavcodec - it should never be read by
   /// the caller after being set.
   ///
   /// - decoding: This field should be set by the caller from the get_format()
   /// callback. The previous reference (if any) will always be
   /// unreffed by libavcodec before the get_format() call.
   ///
   /// If the default get_buffer2() is used with a hwaccel pixel
   /// format, then this AVHWFramesContext will be used for
   /// allocating the frame buffers.
   ///
   /// - encoding: For hardware encoders configured to use a hwaccel pixel
   /// format, this field should be set by the caller to a reference
   /// to the AVHWFramesContext describing input frames.
   /// AVHWFramesContext.format must be equal to
   /// AVCodecContext.pix_fmt.
   ///
   /// This field should be set before avcodec_open2() is called.
    pub hw_frames_ctx: *mut AVBufferRef,
    /// Control the form of AVSubtitle.rects[N]->ass
   /// - decoding: set by user
   /// - encoding: unused
    pub sub_text_format: libc::c_int,
    /// Audio only. The amount of padding (in samples) appended by the encoder to
   /// the end of the audio. I.e. this number of decoded samples must be
   /// discarded by the caller from the end of the stream to get the original
   /// audio without any trailing padding.
   ///
   /// - decoding: unused
   /// - encoding: unused
    pub trailing_padding: libc::c_int,
    /// The number of pixels per image to maximally accept.
   ///
   /// - decoding: set by user
   /// - encoding: set by user
    pub max_pixels: i64,
    /// A reference to the AVHWDeviceContext describing the device which will
   /// be used by a hardware encoder/decoder.  The reference is set by the
   /// caller and afterwards owned (and freed) by libavcodec.
   ///
   /// This should be used if either the codec device does not require
   /// hardware frames or any that are used are to be allocated internally by
   /// libavcodec.  If the user wishes to supply any of the frames used as
   /// encoder input or decoder output then hw_frames_ctx should be used
   /// instead.  When hw_frames_ctx is set in get_format() for a decoder, this
   /// field will be ignored while decoding the associated stream segment, but
   /// may again be used on a following one after another get_format() call.
   ///
   /// For both encoders and decoders this field should be set before
   /// avcodec_open2() is called and must not be written to thereafter.
   ///
   /// Note that some decoders may require this field to be set initially in
   /// order to support hw_frames_ctx at all - in that case, all frames
   /// contexts used must be created on the same device.
    pub hw_device_ctx: *mut AVBufferRef,
    /// Bit set of AV_HWACCEL_FLAG_* flags, which affect hardware accelerated
   /// decoding (if active).
   /// - encoding: unused
   /// - decoding: Set by user (either before avcodec_open2(), or in the
   /// AVCodecContext.get_format callback)
    pub hwaccel_flags: libc::c_int,
    /// Video decoding only. Certain video codecs support cropping, meaning that
   /// only a sub-rectangle of the decoded frame is intended for display.  This
   /// option controls how cropping is handled by libavcodec.
   ///
   /// When set to 1 (the default), libavcodec will apply cropping internally.
   /// I.e. it will modify the output frame width/height fields and offset the
   /// common pointers (only by as much as possible while preserving alignment, or
   /// by the full amount if the AV_CODEC_FLAG_UNALIGNED flag is set) so that
   /// the frames output by the decoder refer only to the cropped area. The
   /// crop_* fields of the output frames will be zero.
   ///
   /// When set to 0, the width/height fields of the output frames will be set
   /// to the coded dimensions and the crop_* fields will describe the cropping
   /// rectangle. Applying the cropping is left to the caller.
   ///
   /// @warning When hardware acceleration with opaque output frames is used,
   /// libavcodec is unable to apply cropping from the top/left border.
   ///
   /// @note when this option is set to zero, the width/height fields of the
   /// AVCodecContext and output AVFrames have different meanings. The codec
   /// context fields store display dimensions (with the coded dimensions in
   /// coded_width/height), while the frame fields store the coded dimensions
   /// (with the display dimensions being determined by the crop_* fields).
    pub apply_cropping: libc::c_int,
    pub extra_hw_frames: libc::c_int,
}

#[repr(i32)]      /// @ingroup lavc_decoding
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVDiscard {
    AVDISCARD_NONE = -16,
    AVDISCARD_DEFAULT = 0,
    AVDISCARD_NONREF = 8,
    AVDISCARD_BIDIR = 16,
    AVDISCARD_NONINTRA = 24,
    AVDISCARD_NONKEY = 32,
    AVDISCARD_ALL = 48
}

/// @ingroup lavc_encoding
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RcOverride {
    pub start_frame: libc::c_int,
    pub end_frame: libc::c_int,
    pub qscale: libc::c_int,
    pub quality_factor: f32
}

/// This struct describes the properties of an encoded stream.
///
/// sizeof(AVCodecParameters) is not a part of the public ABI, this struct must
/// be allocated with avcodec_parameters_alloc() and freed with
/// avcodec_parameters_free().
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVCodecParameters {
    /// General type of the encoded common.
    pub codec_type: AVMediaType,
    /// Specific type of the encoded common (the codec used).
    pub codec_id: AVCodecID,
    /// Additional information about the codec (corresponds to the AVI FOURCC).
    pub codec_tag: u32,
    /// Extra binary common needed for initializing the decoder, codec-dependent.
   ///
   /// Must be allocated with av_malloc() and will be freed by
   /// avcodec_parameters_free(). The allocated size of extradata must be at
   /// least extradata_size + AV_INPUT_BUFFER_PADDING_SIZE, with the padding
   /// bytes zeroed.
    pub extradata: *mut u8,
    /// Size of the extradata content in bytes.
    pub extradata_size: libc::c_int,
    /// - video: the pixel format, the value corresponds to enum AVPixelFormat.
   /// - audio: the sample format, the value corresponds to enum AVSampleFormat.
    pub format: libc::c_int,
    /// The average bitrate of the encoded common (in bits per second).
    pub bit_rate: i64,
    /// The number of bits per sample in the codedwords.
   ///
   /// This is basically the bitrate per sample. It is mandatory for a bunch of
   /// formats to actually decode them. It's the number of bits for one sample in
   /// the actual coded bitstream.
   ///
   /// This could be for example 4 for ADPCM
   /// For PCM formats this matches bits_per_raw_sample
   /// Can be 0
    pub bits_per_coded_sample: libc::c_int,
    /// This is the number of valid bits in each output sample. If the
   /// sample format has more bits, the least significant bits are additional
   /// padding bits, which are always 0. Use right shifts to reduce the sample
   /// to its actual size. For example, audio formats with 24 bit samples will
   /// have bits_per_raw_sample set to 24, and format set to AV_SAMPLE_FMT_S32.
   /// To get the original sample use "(int32_t)sample >> 8"."
   ///
   /// For ADPCM this might be 12 or 16 or similar
   /// Can be 0
    pub bits_per_raw_sample: libc::c_int,
    /// Codec-specific bitstream restrictions that the stream conforms to.
    pub profile: libc::c_int,
    pub level: libc::c_int,
    /// Video only. The dimensions of the video frame in pixels.
    pub width: libc::c_int,
    pub height: libc::c_int,
    /// Video only. The aspect ratio (width / height) which a single pixel
   /// should have when displayed.
   ///
   /// When the aspect ratio is unknown / undefined, the numerator should be
   /// set to 0 (the denominator may have any value).
    pub sample_aspect_ratio: AVRational,
    /// Video only. The order of the fields in interlaced video.
    pub field_order: AVFieldOrder,
    /// Video only. Additional colorspace characteristics.
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    /// Video only. Number of delayed frames.
    pub video_delay: libc::c_int,
    /// Audio only. The channel layout bitmask. May be 0 if the channel layout is
   /// unknown or unspecified, otherwise the number of bits set must be equal to
   /// the channels field.
    pub channel_layout: u64,
    /// Audio only. The number of audio channels.
    pub channels: libc::c_int,
    /// Audio only. The number of audio samples per second.
    pub sample_rate: libc::c_int,
    /// Audio only. The number of bytes per coded audio frame, required by some
   /// formats.
   ///
   /// Corresponds to nBlockAlign in WAVEFORMATEX.
    pub block_align: libc::c_int,
    /// Audio only. Audio frame size, if known. Required by some formats to be static.
    pub frame_size: libc::c_int,
    /// Audio only. The amount of padding (in samples) inserted by the encoder at
   /// the beginning of the audio. I.e. this number of leading decoded samples
   /// must be discarded by the caller to get the original audio without leading
   /// padding.
    pub initial_padding: libc::c_int,
    /// Audio only. The amount of padding (in samples) appended by the encoder to
   /// the end of the audio. I.e. this number of decoded samples must be
   /// discarded by the caller from the end of the stream to get the original
   /// audio without any trailing padding.
    pub trailing_padding: libc::c_int,
    /// Audio only. Number of samples to skip after a discontinuity.
    pub seek_preroll: libc::c_int,
}

/// Stream information used internally by avformat_find_stream_info()
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVStream__bindgen_ty_1 {
    pub last_dts: i64,
    pub duration_gcd: i64,
    pub duration_count: libc::c_int,
    pub rfps_duration_sum: i64,
    pub duration_error: *mut [[f64; 399usize]; 2usize],
    pub codec_info_duration: i64,
    pub codec_info_duration_fields: i64,
    pub frame_delay_evidence: libc::c_int,
    /// 0  -> decoder has not been searched for yet.
   /// >0 -> decoder found
   /// <0 -> decoder with codec_id == -found_decoder has not been found
    pub found_decoder: libc::c_int,
    pub last_duration: i64,
    /// Those are used for average framerate estimation.
    pub fps_first_dts: i64,
    pub fps_first_dts_idx: libc::c_int,
    pub fps_last_dts: i64,
    pub fps_last_dts_idx: libc::c_int,
}

#[repr(u32)]      /// @}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVStreamParseType {
    AVSTREAM_PARSE_NONE = 0,
    AVSTREAM_PARSE_FULL = 1,
    AVSTREAM_PARSE_HEADERS = 2,
    AVSTREAM_PARSE_TIMESTAMPS = 3,
    AVSTREAM_PARSE_FULL_ONCE = 4,
    AVSTREAM_PARSE_FULL_RAW = 5
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVCodecParser {
    pub codec_ids: [libc::c_int; 5usize],
    pub priv_data_size: libc::c_int,
    pub parser_init: ::std::option::Option<unsafe extern "C" fn(s: *mut AVCodecParserContext) -> libc::c_int>,
    pub parser_parse: ::std::option::Option<unsafe extern "C" fn(s: *mut AVCodecParserContext, avctx: *mut AVCodecContext, poutbuf: *mut *const u8, poutbuf_size: *mut libc::c_int, buf: *const u8, buf_size: libc::c_int) -> libc::c_int>,
    pub parser_close: ::std::option::Option<unsafe extern "C" fn(s: *mut AVCodecParserContext)>, pub split: ::std::option::Option<unsafe extern "C" fn(avctx: *mut AVCodecContext, buf: *const u8, buf_size: libc::c_int) -> libc::c_int>,
    pub next: *mut AVCodecParser
}

#[repr(u32)]      /// @defgroup lavc_parsing Frame parsing
/// @{
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVPictureStructure {
    AV_PICTURE_STRUCTURE_UNKNOWN = 0,
    AV_PICTURE_STRUCTURE_TOP_FIELD = 1,
    AV_PICTURE_STRUCTURE_BOTTOM_FIELD = 2,
    AV_PICTURE_STRUCTURE_FRAME = 3
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVCodecParserContext {
    pub priv_data: *mut libc::c_void,
    pub parser: *mut AVCodecParser,
    pub frame_offset: i64,
    pub cur_offset: i64,
    pub next_frame_offset: i64,
    pub pict_type: libc::c_int,
    /// This field is used for proper frame duration computation in lavf.
   /// It signals, how much longer the frame duration of the current frame
   /// is compared to normal frame duration.
   ///
   /// frame_duration = (1 + repeat_pict) * time_base
   ///
   /// It is used by codecs like H.264 to display telecined material.
    pub repeat_pict: libc::c_int,
    pub pts: i64,
    pub dts: i64,
    pub last_pts: i64,
    pub last_dts: i64,
    pub fetch_timestamp: libc::c_int,
    pub cur_frame_start_index: libc::c_int,
    pub cur_frame_offset: [i64; 4usize],
    pub cur_frame_pts: [i64; 4usize],
    pub cur_frame_dts: [i64; 4usize],
    pub flags: libc::c_int,
    /// < byte offset from starting packet start
    pub offset: i64,
    pub cur_frame_end: [i64; 4usize],
    /// Set by parser to 1 for key frames and 0 for non-key frames.
   /// It is initialized to -1, so if the parser doesn't set this flag,
   /// old-style fallback using AV_PICTURE_TYPE_I picture type as key frames
   /// will be used.
    pub key_frame: libc::c_int,
    /// @deprecated unused
    pub convergence_duration: i64,
    /// Synchronization point for start of timestamp generation.
   ///
   /// Set to >0 for sync point, 0 for no sync point and <0 for undefined
   /// (default).
   ///
   /// For example, this corresponds to presence of H.264 buffering period
   /// SEI message.
    pub dts_sync_point: libc::c_int,
    /// Offset of the current timestamp against last timestamp sync point in
   /// units of AVCodecContext.time_base.
   ///
   /// Set to INT_MIN when dts_sync_point unused. Otherwise, it must
   /// contain a valid timestamp offset.
   ///
   /// Note that the timestamp of sync point has usually a nonzero
   /// dts_ref_dts_delta, which refers to the previous sync point. Offset of
   /// the next frame after timestamp sync point will be usually 1.
   ///
   /// For example, this corresponds to H.264 cpb_removal_delay.
    pub dts_ref_dts_delta: libc::c_int,
    /// Presentation delay of current frame in units of AVCodecContext.time_base.
   ///
   /// Set to INT_MIN when dts_sync_point unused. Otherwise, it must
   /// contain valid non-negative timestamp delta (presentation time of a frame
   /// must not lie in the past).
   ///
   /// This delay represents the difference between decoding and presentation
   /// time of the frame.
   ///
   /// For example, this corresponds to H.264 dpb_output_delay.
    pub pts_dts_delta: libc::c_int,
    /// Position of the packet in file.
   ///
   /// Analogous to cur_frame_pts/dts
    pub cur_frame_pos: [i64; 4usize],
    /// Byte position of currently parsed frame in stream.
    pub pos: i64,
    /// Previous frame byte position.
    pub last_pos: i64,
    /// Duration of the current frame.
   /// For audio, this is in units of 1 / AVCodecContext.sample_rate.
   /// For all other types, this is in units of AVCodecContext.time_base.
    pub duration: libc::c_int,
    pub field_order: AVFieldOrder,
    /// Indicate whether a picture is coded as a frame, top field or bottom field.
   ///
   /// For example, H.264 field_pic_flag equal to 0 corresponds to
   /// AV_PICTURE_STRUCTURE_FRAME. An H.264 picture with field_pic_flag
   /// equal to 1 and bottom_field_flag equal to 0 corresponds to
   /// AV_PICTURE_STRUCTURE_TOP_FIELD.
    pub picture_structure: AVPictureStructure,
    /// Picture number incremented in presentation or output order.
   /// This field may be reinitialized at the first picture of a new sequence.
   ///
   /// For example, this corresponds to H.264 PicOrderCnt.
    pub output_picture_number: libc::c_int,
    /// Dimensions of the decoded video intended for presentation.
    pub width: libc::c_int,
    pub height: libc::c_int,
    /// Dimensions of the coded video.
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    /// The format of the coded common, corresponds to enum AVPixelFormat for video
   /// and for enum AVSampleFormat for audio.
   ///
   /// Note that a decoder can have considerable freedom in how exactly it
   /// decodes the common, so the format reported here might be different from the
   /// one returned by a decoder.
    pub format: libc::c_int,
}


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVPacketList { pub pkt: AVPacket, pub next: *mut AVPacketList }

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
    where
        Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}

impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
    where
        Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
            align: [],
        }
    }

    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        byte & mask == mask
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        let mut val = 0u64;

        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                val |= 1 << i;
            }
        }

        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            self.set_bit(i + bit_offset, val_bit_is_set);
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVIndexEntry {
    pub pos: i64,
    /// <
   /// Timestamp in AVStream.time_base units, preferably the time from which on correctly decoded frames are available
   /// when seeking to this entry. That means preferable PTS on keyframe based formats.
   /// But demuxers can choose to store a different timestamp, if it is more convenient for the implementation or nothing better
   /// is known
    pub timestamp: i64,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    /// < Minimum distance between this and the previous keyframe, used to avoid unneeded searching.
    pub min_distance: libc::c_int,
}
/// Stream structure.
/// New fields can be added to the end with minor version bumps.
/// Removal, reordering and changes to existing fields require a major
/// version bump.
/// sizeof(AVStream) must not be used outside libav*.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVStream {
    /// < stream index in AVFormatContext
    pub index: libc::c_int,
    /// Format-specific stream ID.
   /// decoding: set by libavformat
   /// encoding: set by the user, replaced by libavformat if left unset
    pub id: libc::c_int,
    /// @deprecated use the codecpar struct instead
    pub codec: *mut AVCodecContext,
    pub priv_data: *mut libc::c_void,
    /// This is the fundamental unit of time (in seconds) in terms
   /// of which frame timestamps are represented.
   ///
   /// decoding: set by libavformat
   /// encoding: May be set by the caller before avformat_write_header() to
   /// provide a hint to the muxer about the desired timebase. In
   /// avformat_write_header(), the muxer will overwrite this field
   /// with the timebase that will actually be used for the timestamps
   /// written into the file (which may or may not be related to the
   /// user-provided one, depending on the format).
    pub time_base: AVRational,
    /// Decoding: pts of the first frame of the stream in presentation order, in stream time base.
   /// Only set this if you are absolutely 100% sure that the value you set
   /// it to really is the pts of the first frame.
   /// This may be undefined (AV_NOPTS_VALUE).
   /// @note The ASF header does NOT contain a correct start_time the ASF
   /// demuxer must NOT set this.
    pub start_time: i64,
    /// Decoding: duration of the stream, in stream time base.
   /// If a source file does not specify a duration, but does specify
   /// a bitrate, this value will be estimated from bitrate and file size.
   ///
   /// Encoding: May be set by the caller before avformat_write_header() to
   /// provide a hint to the muxer about the estimated duration.
    pub duration: i64,
    /// < number of frames in this stream if known or 0
    pub nb_frames: i64,
    /// < AV_DISPOSITION_* bit field
    pub disposition: libc::c_int,
    /// < Selects which packets can be discarded at will and do not need to be demuxed.
    pub discard: AVDiscard,
    /// sample aspect ratio (0 if unknown)
   /// - encoding: Set by user.
   /// - decoding: Set by libavformat.
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    /// Average framerate
   ///
   /// - demuxing: May be set by libavformat when creating the stream or in
   /// avformat_find_stream_info().
   /// - muxing: May be set by the caller before avformat_write_header().
    pub avg_frame_rate: AVRational,
    /// For streams with AV_DISPOSITION_ATTACHED_PIC disposition, this packet
   /// will contain the attached picture.
   ///
   /// decoding: set by libavformat, must not be modified by the caller.
   /// encoding: unused
    pub attached_pic: AVPacket,
    /// An array of side common that applies to the whole stream (i.e. the
   /// container does not allow it to change between packets).
   ///
   /// There may be no overlap between the side common in this array and side common
   /// in the packets. I.e. a given side common is either exported by the muxer
   /// (demuxing) / set by the caller (muxing) in this array, then it never
   /// appears in the packets, or the side common is exported / sent through
   /// the packets (always in the first packet where the value becomes known or
   /// changes), then it does not appear in this array.
   ///
   /// - demuxing: Set by libavformat when the stream is created.
   /// - muxing: May be set by the caller before avformat_write_header().
   ///
   /// Freed by libavformat in avformat_free_context().
   ///
   /// @see av_format_inject_global_side_data()
    pub side_data: *mut AVPacketSideData,
    /// The number of elements in the AVStream.side_data array.
    pub nb_side_data: libc::c_int,
    /// Flags for the user to detect events happening on the stream. Flags must
   /// be cleared by the user once the event has been handled.
   /// A combination of AVSTREAM_EVENT_FLAG_*.
    pub event_flags: libc::c_int,
    /// Real base framerate of the stream.
   /// This is the lowest framerate with which all timestamps can be
   /// represented accurately (it is the least common multiple of all
   /// framerates in the stream). Note, this value is just a guess!
   /// For example, if the time base is 1/90000 and all frames have either
   /// approximately 3600 or 1800 timer ticks, then r_frame_rate will be 50/1.
    pub r_frame_rate: AVRational,
    /// String containing pairs of key and values describing recommended encoder configuration.
   /// Pairs are separated by ','.
   /// Keys are separated from values by '='.
   ///
   /// @deprecated unused
    pub recommended_encoder_configuration: *mut libc::c_char,
    /// Codec parameters associated with this stream. Allocated and freed by
   /// libavformat in avformat_new_stream() and avformat_free_context()
   /// respectively.
   ///
   /// - demuxing: filled by libavformat on stream creation or in
   /// avformat_find_stream_info()
   /// - muxing: filled by the caller before avformat_write_header()
    pub codecpar: *mut AVCodecParameters,
    pub info: *mut AVStream__bindgen_ty_1,
    /// < number of bits in pts (used for wrapping control)
    pub pts_wrap_bits: libc::c_int,
    /// Timestamp corresponding to the last dts sync point.
   ///
   /// Initialized when AVCodecParserContext.dts_sync_point >= 0 and
   /// a DTS is received from the underlying container. Otherwise set to
   /// AV_NOPTS_VALUE by default.
    pub first_dts: i64,
    pub cur_dts: i64,
    pub last_IP_pts: i64,
    pub last_IP_duration: libc::c_int,
    /// Number of packets to buffer for codec probing
    pub probe_packets: libc::c_int,
    /// Number of frames that have been demuxed during avformat_find_stream_info()
    pub codec_info_nb_frames: libc::c_int,
    pub need_parsing: AVStreamParseType,
    pub parser: *mut AVCodecParserContext,
    /// last packet in packet_buffer for this stream when muxing.
    pub last_in_packet_buffer: *mut AVPacketList,
    pub probe_data: AVProbeData,
    pub pts_buffer: [i64; 17usize],
    /// < Only used if the format does not
   /// support seeking natively.
    pub index_entries: *mut AVIndexEntry,
    pub nb_index_entries: libc::c_int,
    pub index_entries_allocated_size: libc::c_uint,
    /// Stream Identifier
   /// This is the MPEG-TS stream identifier +1
   /// 0 means unknown
    pub stream_identifier: libc::c_int,
    /// Details of the MPEG-TS program which created this stream.
    pub program_num: libc::c_int,
    pub pmt_version: libc::c_int,
    pub pmt_stream_idx: libc::c_int,
    pub interleaver_chunk_size: i64,
    pub interleaver_chunk_duration: i64,
    /// stream probing state
   /// -1   -> probing finished
   /// 0   -> no probing requested
   /// rest -> perform probing with request_probe being the minimum score to accept.
   /// NOT PART OF PUBLIC API
    pub request_probe: libc::c_int,
    /// Indicates that everything up to the next keyframe
   /// should be discarded.
    pub skip_to_keyframe: libc::c_int,
    /// Number of samples to skip at the start of the frame decoded from the next packet.
    pub skip_samples: libc::c_int,
    /// If not 0, the number of samples that should be skipped from the start of
   /// the stream (the samples are removed from packets with pts==0, which also
   /// assumes negative timestamps do not happen).
   /// Intended for use with formats such as mp3 with ad-hoc gapless audio
   /// support.
    pub start_skip_samples: i64,
    /// If not 0, the first audio sample that should be discarded from the stream.
   /// This is broken by design (needs global sample count), but can't be
   /// avoided for broken by design formats such as mp3 with ad-hoc gapless
   /// audio support.
    pub first_discard_sample: i64,
    /// The sample after last sample that is intended to be discarded after
   /// first_discard_sample. Works on frame boundaries only. Used to prevent
   /// early EOF if the gapless info is broken (considered concatenated mp3s).
    pub last_discard_sample: i64,
    /// Number of internally decoded frames, used internally in libavformat, do not access
   /// its lifetime differs from info which is why it is not in that structure.
    pub nb_decoded_frames: libc::c_int,
    /// Timestamp offset added to timestamps before muxing
   /// NOT PART OF PUBLIC API
    pub mux_ts_offset: i64,
    /// Internal common to check for wrapping of the time stamp
    pub pts_wrap_reference: i64,
    /// Options for behavior, when a wrap is detected.
   ///
   /// Defined by AV_PTS_WRAP_ values.
   ///
   /// If correction is enabled, there are two possibilities:
   /// If the first time stamp is near the wrap point, the wrap offset
   /// will be subtracted, which will create negative time stamps.
   /// Otherwise the offset will be added.
    pub pts_wrap_behavior: libc::c_int,
    /// Internal common to prevent doing update_initial_durations() twice
    pub update_initial_durations_done: libc::c_int,
    /// Internal common to generate dts from pts
    pub pts_reorder_error: [i64; 17usize],
    pub pts_reorder_error_count: [u8; 17usize],
    /// Internal common to analyze DTS and detect faulty mpeg streams
    pub last_dts_for_order_check: i64,
    pub dts_ordered: u8,
    pub dts_misordered: u8,
    /// Internal common to inject global side common
    pub inject_global_side_data: libc::c_int,
    /// display aspect ratio (0 if unknown)
   /// - encoding: unused
   /// - decoding: Set by libavformat to calculate sample_aspect_ratio internally
    pub display_aspect_ratio: AVRational,
    /// An opaque field for libavformat internal usage.
   /// Must not be accessed in any way by callers.
    pub internal: *mut AVStreamInternal,
}

/// New fields can be added to the end with minor version bumps.
/// Removal, reordering and changes to existing fields require a major
/// version bump.
/// sizeof(AVProgram) must not be used outside libav*.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVProgram {
    pub id: libc::c_int,
    pub flags: libc::c_int,
    /// < selects which program to discard and which to feed to the caller
    pub discard: AVDiscard,
    pub stream_index: *mut libc::c_uint,
    pub nb_stream_indexes: libc::c_uint,
    pub metadata: *mut AVDictionary,
    pub program_num: libc::c_int,
    pub pmt_pid: libc::c_int,
    pub pcr_pid: libc::c_int,
    pub pmt_version: libc::c_int,
    /// All fields below this line are not part of the public API. They
   /// may not be used outside of libavformat and can be changed and
   /// removed at will.
   /// New public fields should be added right above.
   ///
    pub start_time: i64,
    pub end_time: i64,
    /// < reference dts for wrap detection
    pub pts_wrap_reference: i64,
    /// < behavior on wrap detection
    pub pts_wrap_behavior: libc::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVChapter {
    /// < unique ID to identify the chapter
    pub id: libc::c_int,
    /// < time base in which the start/end timestamps are specified
    pub time_base: AVRational,
    /// < chapter start/end time in time_base units
    pub start: i64,
    /// < chapter start/end time in time_base units
    pub end: i64,
    pub metadata: *mut AVDictionary,
}

/// Callback for checking whether to abort blocking functions.
/// AVERROR_EXIT is returned in this case by the interrupted
/// function. During blocking operations, callback is called with
/// opaque as parameter. If the callback returns 1, the
/// blocking operation will be aborted.
///
/// No members can be added to this struct without a major bump, if
/// new elements have been added after this struct in AVFormatContext
/// or AVIOContext.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVIOInterruptCB {
    pub callback: ::std::option::Option<unsafe extern "C" fn(arg1: *mut libc::c_void) -> libc::c_int>,
    pub opaque: *mut libc::c_void
}

#[repr(u32)]      /// The duration of a video can be estimated through various ways, and this enum can be used
/// to know how the duration was estimated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AVDurationEstimationMethod {
    AVFMT_DURATION_FROM_PTS = 0,
    AVFMT_DURATION_FROM_STREAM = 1,
    AVFMT_DURATION_FROM_BITRATE = 2
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFormatInternal { _unused: [u8; 0] }

/// Callback used by devices to communicate with application.
pub type av_format_control_message = ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, type_: libc::c_int, data: *mut libc::c_void, data_size: usize) -> libc::c_int>;
pub type AVOpenCallback = ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, pb: *mut *mut AVIOContext, url: *const libc::c_char, flags: libc::c_int, int_cb: *const AVIOInterruptCB, options: *mut *mut AVDictionary) -> libc::c_int>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AVFormatContext {
    /// A class for logging and @ref avoptions. Set by avformat_alloc_context().
   /// Exports (de)muxer private options if they exist.
    pub av_class: *const AVClass,
    /// The input container format.
   ///
   /// Demuxing only, set by avformat_open_input().
    pub iformat: *mut AVInputFormat,
    /// The output container format.
   ///
   /// Muxing only, must be set by the caller before avformat_write_header().
    pub oformat: *mut AVOutputFormat,
    /// Format private common. This is an AVOptions-enabled struct
   /// if and only if iformat/oformat.priv_class is not NULL.
   ///
   /// - muxing: set by avformat_write_header()
   /// - demuxing: set by avformat_open_input()
    pub priv_data: *mut libc::c_void,
    /// I/O context.
   ///
   /// - demuxing: either set by the user before avformat_open_input() (then
   /// the user must close it manually) or set by avformat_open_input().
   /// - muxing: set by the user before avformat_write_header(). The caller must
   /// take care of closing / freeing the IO context.
   ///
   /// Do NOT set this field if AVFMT_NOFILE flag is set in
   /// iformat/oformat.flags. In such a case, the (de)muxer will handle
   /// I/O in some other way and this field will be NULL.
    pub pb: *mut AVIOContext,
    /// Flags signalling stream properties. A combination of AVFMTCTX_*.
   /// Set by libavformat.
    pub ctx_flags: libc::c_int,
    /// Number of elements in AVFormatContext.streams.
   ///
   /// Set by avformat_new_stream(), must not be modified by any other code.
    pub nb_streams: libc::c_uint,
    /// A list of all streams in the file. New streams are created with
   /// avformat_new_stream().
   ///
   /// - demuxing: streams are created by libavformat in avformat_open_input().
   /// If AVFMTCTX_NOHEADER is set in ctx_flags, then new streams may also
   /// appear in av_read_frame().
   /// - muxing: streams are created by the user before avformat_write_header().
   ///
   /// Freed by libavformat in avformat_free_context().
    pub streams: *mut *mut AVStream,
    /// input or output filename
   ///
   /// - demuxing: set by avformat_open_input()
   /// - muxing: may be set by the caller before avformat_write_header()
   ///
   /// @deprecated Use url instead.
    pub filename: [libc::c_char; 1024usize],
    /// input or output URL. Unlike the old filename field, this field has no
   /// length restriction.
   ///
   /// - demuxing: set by avformat_open_input(), initialized to an empty
   /// string if url parameter was NULL in avformat_open_input().
   /// - muxing: may be set by the caller before calling avformat_write_header()
   /// (or avformat_init_output() if that is called first) to a string
   /// which is freeable by av_free(). Set to an empty string if it
   /// was NULL in avformat_init_output().
   ///
   /// Freed by libavformat in avformat_free_context().
    pub url: *mut libc::c_char,
    /// Position of the first frame of the component, in
   /// AV_TIME_BASE fractional seconds. NEVER set this value directly:
   /// It is deduced from the AVStream values.
   ///
   /// Demuxing only, set by libavformat.
    pub start_time: i64,
    /// Duration of the stream, in AV_TIME_BASE fractional
   /// seconds. Only set this value if you know none of the individual stream
   /// durations and also do not set any of them. This is deduced from the
   /// AVStream values if not set.
   ///
   /// Demuxing only, set by libavformat.
    pub duration: i64,
    /// Total stream bitrate in bit/s, 0 if not
   /// available. Never set it directly if the file_size and the
   /// duration are known as FFmpeg can compute it automatically.
    pub bit_rate: i64,
    pub packet_size: libc::c_uint,
    pub max_delay: libc::c_int,
    /// Flags modifying the (de)muxer behaviour. A combination of AVFMT_FLAG_*.
   /// Set by the user before avformat_open_input() / avformat_write_header().
    pub flags: libc::c_int,
    /// Maximum size of the common read from input for determining
   /// the input container format.
   /// Demuxing only, set by the caller before avformat_open_input().
    pub probesize: i64,
    /// Maximum duration (in AV_TIME_BASE units) of the common read
   /// from input in avformat_find_stream_info().
   /// Demuxing only, set by the caller before avformat_find_stream_info().
   /// Can be set to 0 to let avformat choose using a heuristic.
    pub max_analyze_duration: i64,
    pub key: *const u8,
    pub keylen: libc::c_int,
    pub nb_programs: libc::c_uint,
    pub programs: *mut *mut AVProgram,
    /// Forced video codec_id.
   /// Demuxing: Set by user.
    pub video_codec_id: AVCodecID,
    /// Forced audio codec_id.
   /// Demuxing: Set by user.
    pub audio_codec_id: AVCodecID,
    /// Forced subtitle codec_id.
   /// Demuxing: Set by user.
    pub subtitle_codec_id: AVCodecID,
    /// Maximum amount of memory in bytes to use for the index of each stream.
   /// If the index exceeds this size, entries will be discarded as
   /// needed to maintain a smaller size. This can lead to slower or less
   /// accurate seeking (depends on demuxer).
   /// Demuxers for which a full in-memory index is mandatory will ignore
   /// this.
   /// - muxing: unused
   /// - demuxing: set by user
    pub max_index_size: libc::c_uint,
    /// Maximum amount of memory in bytes to use for buffering frames
   /// obtained from realtime capture devices.
    pub max_picture_buffer: libc::c_uint,
    /// Number of chapters in AVChapter array.
   /// When muxing, chapters are normally written in the file header,
   /// so nb_chapters should normally be initialized before write_header
   /// is called. Some muxers (e.g. mov and mkv) can also write chapters
   /// in the trailer.  To write chapters in the trailer, nb_chapters
   /// must be zero when write_header is called and non-zero when
   /// write_trailer is called.
   /// - muxing: set by user
   /// - demuxing: set by libavformat
    pub nb_chapters: libc::c_uint,
    pub chapters: *mut *mut AVChapter,
    /// Metadata that applies to the whole file.
   ///
   /// - demuxing: set by libavformat in avformat_open_input()
   /// - muxing: may be set by the caller before avformat_write_header()
   ///
   /// Freed by libavformat in avformat_free_context().
    pub metadata: *mut AVDictionary,
    /// Start time of the stream in real world time, in microseconds
   /// since the Unix epoch (00:00 1st January 1970). That is, pts=0 in the
   /// stream was captured at this real world time.
   /// - muxing: Set by the caller before avformat_write_header(). If set to
   /// either 0 or AV_NOPTS_VALUE, then the current wall-time will
   /// be used.
   /// - demuxing: Set by libavformat. AV_NOPTS_VALUE if unknown. Note that
   /// the value may become known after some number of frames
   /// have been received.
    pub start_time_realtime: i64,
    /// The number of frames used for determining the framerate in
   /// avformat_find_stream_info().
   /// Demuxing only, set by the caller before avformat_find_stream_info().
    pub fps_probe_size: libc::c_int,
    /// Error recognition; higher values will detect more errors but may
   /// misdetect some more or less valid parts as errors.
   /// Demuxing only, set by the caller before avformat_open_input().
    pub error_recognition: libc::c_int,
    /// Custom interrupt callbacks for the I/O layer.
   ///
   /// demuxing: set by the user before avformat_open_input().
   /// muxing: set by the user before avformat_write_header()
   /// (mainly useful for AVFMT_NOFILE formats). The callback
   /// should also be passed to avio_open2() if it's used to
   /// open the file.
    pub interrupt_callback: AVIOInterruptCB,
    /// Flags to enable debugging.
    pub debug: libc::c_int,
    /// Maximum buffering duration for interleaving.
   ///
   /// To ensure all the streams are interleaved correctly,
   /// av_interleaved_write_frame() will wait until it has at least one packet
   /// for each stream before actually writing any packets to the output file.
   /// When some streams are "sparse" (i.e. there are large gaps between
   /// successive packets), this can result in excessive buffering.
   ///
   /// This field specifies the maximum difference between the timestamps of the
   /// first and the last packet in the muxing queue, above which libavformat
   /// will output a packet regardless of whether it has queued a packet for all
   /// the streams.
   ///
   /// Muxing only, set by the caller before avformat_write_header().
    pub max_interleave_delta: i64,
    /// Allow non-standard and experimental extension
   /// @see AVCodecContext.strict_std_compliance
    pub strict_std_compliance: libc::c_int,
    /// Flags for the user to detect events happening on the file. Flags must
   /// be cleared by the user once the event has been handled.
   /// A combination of AVFMT_EVENT_FLAG_*.
    pub event_flags: libc::c_int,
    /// Maximum number of packets to read while waiting for the first timestamp.
   /// Decoding only.
    pub max_ts_probe: libc::c_int,
    /// Avoid negative timestamps during muxing.
   /// Any value of the AVFMT_AVOID_NEG_TS_* constants.
   /// Note, this only works when using av_interleaved_write_frame. (interleave_packet_per_dts is in use)
   /// - muxing: Set by user
   /// - demuxing: unused
    pub avoid_negative_ts: libc::c_int,
    /// Transport stream id.
   /// This will be moved into demuxer private options. Thus no API/ABI compatibility
    pub ts_id: libc::c_int,
    /// Audio preload in microseconds.
   /// Note, not all formats support this and unpredictable things may happen if it is used when not supported.
   /// - encoding: Set by user
   /// - decoding: unused
    pub audio_preload: libc::c_int,
    /// Max chunk time in microseconds.
   /// Note, not all formats support this and unpredictable things may happen if it is used when not supported.
   /// - encoding: Set by user
   /// - decoding: unused
    pub max_chunk_duration: libc::c_int,
    /// Max chunk size in bytes
   /// Note, not all formats support this and unpredictable things may happen if it is used when not supported.
   /// - encoding: Set by user
   /// - decoding: unused
    pub max_chunk_size: libc::c_int,
    /// forces the use of wallclock timestamps as pts/dts of packets
   /// This has undefined results in the presence of B frames.
   /// - encoding: unused
   /// - decoding: Set by user
    pub use_wallclock_as_timestamps: libc::c_int,
    /// avio flags, used to force AVIO_FLAG_DIRECT.
   /// - encoding: unused
   /// - decoding: Set by user
    pub avio_flags: libc::c_int,
    /// The duration field can be estimated through various ways, and this field can be used
   /// to know how the duration was estimated.
   /// - encoding: unused
   /// - decoding: Read by user
    pub duration_estimation_method: AVDurationEstimationMethod,
    /// Skip initial bytes when opening stream
   /// - encoding: unused
   /// - decoding: Set by user
    pub skip_initial_bytes: i64,
    /// Correct single timestamp overflows
   /// - encoding: unused
   /// - decoding: Set by user
    pub correct_ts_overflow: libc::c_uint,
    /// Force seeking to any (also non key) frames.
   /// - encoding: unused
   /// - decoding: Set by user
    pub seek2any: libc::c_int,
    /// Flush the I/O context after each packet.
   /// - encoding: Set by user
   /// - decoding: unused
    pub flush_packets: libc::c_int,
    /// format probing score.
   /// The maximal score is AVPROBE_SCORE_MAX, its set when the demuxer probes
   /// the format.
   /// - encoding: unused
   /// - decoding: set by avformat, read by user
    pub probe_score: libc::c_int,
    /// number of bytes to read maximally to identify format.
   /// - encoding: unused
   /// - decoding: set by user
    pub format_probesize: libc::c_int,
    /// ',' separated list of allowed decoders.
   /// If NULL then all are allowed
   /// - encoding: unused
   /// - decoding: set by user
    pub codec_whitelist: *mut libc::c_char,
    /// ',' separated list of allowed demuxers.
   /// If NULL then all are allowed
   /// - encoding: unused
   /// - decoding: set by user
    pub format_whitelist: *mut libc::c_char,
    /// An opaque field for libavformat internal usage.
   /// Must not be accessed in any way by callers.
    pub internal: *mut AVFormatInternal,
    /// IO repositioned flag.
   /// This is set by avformat when the underlaying IO context read pointer
   /// is repositioned, for example when doing byte based seeking.
   /// Demuxers can use the flag to detect such changes.
    pub io_repositioned: libc::c_int,
    /// Forced video codec.
   /// This allows forcing a specific decoder, even when there are multiple with
   /// the same codec_id.
   /// Demuxing: Set by user
    pub video_codec: *mut AVCodec,
    /// Forced audio codec.
   /// This allows forcing a specific decoder, even when there are multiple with
   /// the same codec_id.
   /// Demuxing: Set by user
    pub audio_codec: *mut AVCodec,
    /// Forced subtitle codec.
   /// This allows forcing a specific decoder, even when there are multiple with
   /// the same codec_id.
   /// Demuxing: Set by user
    pub subtitle_codec: *mut AVCodec,
    /// Forced common codec.
   /// This allows forcing a specific decoder, even when there are multiple with
   /// the same codec_id.
   /// Demuxing: Set by user
    pub data_codec: *mut AVCodec,
    /// Number of bytes to be written as padding in a metadata header.
   /// Demuxing: Unused.
   /// Muxing: Set by user via av_format_set_metadata_header_padding.
    pub metadata_header_padding: libc::c_int,
    /// User common.
   /// This is a place for some private common of the user.
    pub opaque: *mut libc::c_void,
    /// Callback used by devices to communicate with application.
    pub control_message_cb: av_format_control_message,
    /// Output timestamp offset, in microseconds.
   /// Muxing: set by user
    pub output_ts_offset: i64,
    /// dump format separator.
   /// can be ", " or "\n      " or anything else
   /// - muxing: Set by user.
   /// - demuxing: Set by user.
    pub dump_separator: *mut u8,
    /// Forced Data codec_id.
   /// Demuxing: Set by user.
    pub data_codec_id: AVCodecID,
    /// Called to open further IO contexts when needed for demuxing.
   ///
   /// This can be set by the user application to perform security checks on
   /// the URLs before opening them.
   /// The function should behave like avio_open2(), AVFormatContext is provided
   /// as contextual information and to reach AVFormatContext.opaque.
   ///
   /// If NULL then some simple checks are used together with avio_open2().
   ///
   /// Must not be accessed directly from outside avformat.
   /// @See av_format_set_open_cb()
   ///
   /// Demuxing: Set by user.
   ///
   /// @deprecated Use io_open and io_close.
    pub open_cb: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, p: *mut *mut AVIOContext, url: *const libc::c_char, flags: libc::c_int, int_cb: *const AVIOInterruptCB, options: *mut *mut AVDictionary) -> libc::c_int>,
    /// ',' separated list of allowed protocols.
   /// - encoding: unused
   /// - decoding: set by user
    pub protocol_whitelist: *mut libc::c_char,
    /// A callback for opening new IO streams.
   ///
   /// Whenever a muxer or a demuxer needs to open an IO stream (typically from
   /// avformat_open_input() for demuxers, but for certain formats can happen at
   /// other times as well), it will call this callback to obtain an IO context.
   ///
   /// @param s the format context
   /// @param pb on success, the newly opened IO context should be returned here
   /// @param url the url to open
   /// @param flags a combination of AVIO_FLAG_*
   /// @param options a dictionary of additional options, with the same
   /// semantics as in avio_open2()
   /// @return 0 on success, a negative AVERROR code on failure
   ///
   /// @note Certain muxers and demuxers do nesting, i.e. they open one or more
   /// additional internal format contexts. Thus the AVFormatContext pointer
   /// passed to this callback may be different from the one facing the caller.
   /// It will, however, have the same 'opaque' field.
    pub io_open: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, pb: *mut *mut AVIOContext, url: *const libc::c_char, flags: libc::c_int, options: *mut *mut AVDictionary) -> libc::c_int>,
    /// A callback for closing the streams opened with AVFormatContext.io_open().
    pub io_close: ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, pb: *mut AVIOContext)>,
    /// ',' separated list of disallowed protocols.
   /// - encoding: unused
   /// - decoding: set by user
    pub protocol_blacklist: *mut libc::c_char,
    /// The maximum number of streams.
   /// - encoding: unused
   /// - decoding: set by user
    pub max_streams: libc::c_int,
    /// Skip duration calcuation in estimate_timings_from_pts.
   /// - encoding: unused
   /// - decoding: set by user
    pub skip_estimate_duration_from_pts: libc::c_int,
}

#[link(name = "avformat", kind = "static")]
extern "C" {
    pub fn av_register_all();
    pub fn avformat_open_input(ps: *mut *mut AVFormatContext, url: *const libc::c_char, fmt: *mut AVInputFormat, options: *mut *mut AVDictionary) -> libc::c_int;
    pub fn avio_open2(s: *mut *mut AVIOContext, url: *const libc::c_char, flags: libc::c_int, int_cb: *const AVIOInterruptCB, options: *mut *mut AVDictionary) -> libc::c_int;
    pub fn avio_close(s: *mut AVIOContext) -> libc::c_int;
    pub fn avio_alloc_context(buffer: *mut libc::c_uchar, buffer_size: libc::c_int, write_flag: libc::c_int, opaque1: *mut libc::c_void, read_packet: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void, buf: *mut u8, buf_size: libc::c_int) -> libc::c_int>, write_packet: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void, buf: *mut u8, buf_size: libc::c_int) -> libc::c_int>, seek: ::std::option::Option<unsafe extern "C" fn(opaque: *mut libc::c_void, offset: i64, whence: libc::c_int) -> i64>) -> *mut AVIOContext;
    pub fn avformat_new_stream(s: *mut AVFormatContext, c: *const AVCodec) -> *mut AVStream;
    pub fn av_probe_input_buffer2(pb: *mut AVIOContext, fmt: *mut *mut AVInputFormat, url: *const libc::c_char, logctx: *mut libc::c_void, offset: libc::c_uint, max_probe_size: libc::c_uint) -> libc::c_int;
    pub fn avio_read(s: *mut AVIOContext, buf: *mut libc::c_uchar, size: libc::c_int) -> libc::c_int;
    pub fn av_probe_input_format2(pd: *mut AVProbeData, is_opened: libc::c_int, score_max: *mut libc::c_int) -> *mut AVInputFormat;
    pub fn av_probe_input_format3(pd: *mut AVProbeData, is_opened: libc::c_int, score_ret: *mut libc::c_int) -> *mut AVInputFormat;
    pub fn av_iformat_next(f: *const AVInputFormat) -> *mut AVInputFormat;
    pub fn av_match_ext(filename: *const libc::c_char, extensions: *const libc::c_char) -> libc::c_int;
    pub fn avformat_find_stream_info(ic: *mut AVFormatContext, options: *mut *mut AVDictionary) -> libc::c_int;
    pub fn av_read_frame(s: *mut AVFormatContext, pkt: *mut AVPacket) -> libc::c_int;
    pub fn avformat_close_input(s: *mut *mut AVFormatContext);
    pub fn avformat_free_context(s: *mut AVFormatContext);
}

#[link(name = "avcodec", kind = "static")]
extern "C" {
    pub fn avcodec_register(codec: *mut AVCodec);
    pub fn avcodec_open2(avctx: *mut AVCodecContext, codec: *const AVCodec, options: *mut *mut AVDictionary) -> libc::c_int;
    pub fn avcodec_close(avctx: *mut AVCodecContext) -> libc::c_int;
    pub fn avcodec_find_decoder(id: AVCodecID) -> *mut AVCodec;
    pub fn avcodec_find_decoder_by_name(name: *const libc::c_char) -> *mut AVCodec;
    pub fn avcodec_decode_video2(avctx: *mut AVCodecContext, picture: *mut AVFrame, got_picture_ptr: *mut libc::c_int, avpkt: *const AVPacket) -> libc::c_int;
    pub fn av_packet_split_side_data(pkt: *mut AVPacket) -> libc::c_int;
    pub fn av_codec_next(c: *const AVCodec) -> *mut AVCodec;
    pub fn avcodec_find_encoder(id: AVCodecID) -> *mut AVCodec;
    pub fn avcodec_find_encoder_by_name(name: *const libc::c_char) -> *mut AVCodec;
    pub fn av_init_packet(pkt: *mut AVPacket);
    pub fn av_packet_alloc() -> *mut AVPacket;
    pub fn av_new_packet(pkt: *mut AVPacket, size: libc::c_int) -> libc::c_int;
    pub fn avcodec_register_all();
    pub fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
    pub fn avcodec_encode_video2(avctx: *mut AVCodecContext, avpkt: *mut AVPacket, frame: *const AVFrame, got_packet_ptr: *mut libc::c_int) -> libc::c_int;
}

#[link(name = "avutil", kind = "static")]
extern "C" {
    pub fn av_frame_set_pkt_pos(frame: *mut AVFrame, val: i64);
    pub fn av_frame_set_best_effort_timestamp(frame: *mut AVFrame, val: i64);
    pub fn av_frame_alloc() -> *mut AVFrame;
    pub fn av_image_alloc(pointers: *mut *mut u8, linesizes: *mut libc::c_int, w: libc::c_int, h: libc::c_int, pix_fmt: AVPixelFormat, align: libc::c_int) -> libc::c_int;
}