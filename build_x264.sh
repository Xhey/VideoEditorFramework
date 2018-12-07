#!/bin/bash
export WD=/Users/zhaoliangtai/Desktop/build-x264/x264-snapshot-20181204-2245
export PLATFORM_VERSION=android-19
export ANDROID_NDK=/Users/zhaoliangtai/Downloads/android-sdk-macosx/ndk-bundle
export TOOLCHAIN=$ANDROID_NDK/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64
export SYSROOT=$ANDROID_NDK/platforms/android-19/arch-arm/
export PLATFORM=$TOOLCHAIN/sysroot
export PREFIX=$WD/android-lib
export OUT_PREFIX=$WD/../264fflib
export ISYSROOT=$ANDROID_NDK/sysroot
export ASM=$ISYSROOT/usr/include/arm-linux-androideabi

FF_CFLAGS="-I$ASM -isysroot $ISYSROOT -D__ANDROID_API__=19 -U_FILE_OFFSET_BITS -DANDROID"

./configure \
--prefix=$PREFIX \
--enable-static \
--enable-shared \
--enable-pic \
--disable-asm \
--disable-cli \
--host=arm-linux \
--cross-prefix=$TOOLCHAIN/bin/arm-linux-androideabi- \
--sysroot=$SYSROOT \
--extra-cflags="$FF_CFLAGS" \
--extra-ldflags=""

make -j8
make install
