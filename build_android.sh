#!/bin/bash
SHELL_PATH=`pwd`
FF_PATH=$SHELL_PATH/ffmpeg_source
NDK=/Users/zhaoliangtai/Downloads/android-ndk-r16b
PREFIX=$SHELL_PATH/FFmpeg_android
ANDROID_API=19

x264=$FF_PATH/android-lib
ARCH=arm
TRIPLES=arm-linux-androideabi
TRIPLES_PATH=arm-linux-androideabi-4.9

FF_CONFIGURE_FLAGS="--enable-static --disable-shared --enable-pic --enable-gpl --enable-postproc --disable-stripping --enable-small --enable-version3"

sh $SHELL_PATH/build-ffmpeg-patch.sh $FF_PATH

cd $FF_PATH
export TMPDIR=$FF_PATH/tmpdir
mkdir $TMPDIR

TOOLCHAIN=$NDK/toolchains/$TRIPLES_PATH/prebuilt/darwin-x86_64
SYSROOT=$NDK/platforms/android-$ANDROID_API/arch-$ARCH
ISYSROOT=$NDK/sysroot
ASM=$ISYSROOT/usr/include/$TRIPLES
CROSS_PREFIX=$TOOLCHAIN/bin/$TRIPLES-
PREFIX_ARCH=$PREFIX/$ARCH

FF_EXTRA_CONFIGURE_FLAGS="--disable-asm"
FF_EXTRA_CFLAGS="-fpic -ffunction-sections -funwind-tables -fstack-protector -march=armv7-a -mfloat-abi=softfp -mfpu=vfpv3-d16 -fomit-frame-pointer -fstrict-aliasing -funswitch-loops -finline-limit-300"

FF_EXTRA_CONFIGURE_FLAGS="$FF_EXTRA_CONFIGURE_FLAGS --enable-libx264 --enable-encoder=libx264"
FF_EXTRA_CFLAGS="$FF_EXTRA_CFLAGS -I$x264/include"
FF_LDFLAGS="-L$x264/lib"

FF_CFLAGS="-I$ASM -isysroot $ISYSROOT -D__ANDROID_API__=$ANDROID_API -U_FILE_OFFSET_BITS -O3 -pipe -Wall -ffast-math -fstrict-aliasing -Wno-psabi -Wa,--noexecstack -DANDROID"

./configure \
--prefix=$PREFIX_ARCH \
--sysroot=$SYSROOT \
--target-os=android \
--arch=$ARCH \
--cc=$TOOLCHAIN/bin/arm-linux-androideabi-gcc \
--cross-prefix=$CROSS_PREFIX \
--enable-cross-compile \
--disable-runtime-cpudetect \
--disable-doc \
--disable-debug \
--disable-ffmpeg \
--disable-ffprobe \
--disable-ffserver \
--cpu=cortex-a8 \
$FF_CONFIGURE_FLAGS \
$FF_EXTRA_CONFIGURE_FLAGS \
--extra-cflags="$FF_EXTRA_CFLAGS $FF_CFLAGS" \
--extra-ldflags="$FF_LDFLAGS" \

$ADDITIONAL_CONFIGURE_FLAG

make clean
make -j4
make install

$TOOLCHAIN/bin/arm-linux-androideabi-ld -rpath-link=$PLATFORM/usr/lib -L$PLATFORM/usr/lib -L$OUT_PREFIX/lib -soname libffmpeg.so -shared -nostdlib -Bsymbolic --whole-archive --no-undefined -o $OUT_PREFIX/libffmpeg.so \
android-lib/lib/libx264.a \
libavcodec/libavcodec.a \
libavfilter/libavfilter.a \
libswresample/libswresample.a \
libavformat/libavformat.a \
libavutil/libavutil.a \
libswscale/libswscale.a \
libpostproc/libpostproc.a \
libavdevice/libavdevice.a \
-lc -lm -lz -ldl -llog --dynamic-linker=/system/bin/linker $TOOLCHAIN/lib/gcc/arm-linux-androideabi/4.9.x/libgcc.a

