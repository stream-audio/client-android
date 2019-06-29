#/bin/bash

# arm64 to jniLibs/arm64-v8a:

export CC=/home/willir/Android/Sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang
export CXX=/home/willir/Android/Sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang++

./configure --enable-cross-compile --enable-shared --disable-static --disable-programs --arch=arm64 --cpu=armv8.1-a \
    --cc=$CC --cxx=$CXX \
    --disable-debug \
    --disable-network \
    --disable-avfilter \
    --disable-encoders --disable-decoders --enable-encoder=aac --enable-decoder=aac --enable-encoder=mp2 --enable-decoder=mp2 \
    --disable-muxers --disable-demuxers --enable-muxer=adts --enable-muxer=data --enable-demuxer=data \
    --disable-parsers --enable-parser=aac --enable-parser=aac_latm \
    --disable-protocols --disable-bsfs --disable-indevs --disable-outdevs

# arm to jniLibs/armeabi-v7a:

export CC=/home/willir/Android/Sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang
export CXX=/home/willir/Android/Sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang++

./configure --enable-cross-compile --enable-shared --disable-static --disable-programs --arch=arm \
    --cc=$CC --cxx=$CXX \
    --disable-debug \
    --disable-neon --disable-vfp \
    --disable-network \
    --disable-avfilter \
    --disable-encoders --disable-decoders --enable-encoder=aac --enable-decoder=aac --enable-encoder=mp2 --enable-decoder=mp2 \
    --disable-muxers --disable-demuxers --enable-muxer=adts --enable-muxer=data --enable-demuxer=data \
    --disable-parsers --enable-parser=aac --enable-parser=aac_latm \
    --disable-protocols --disable-bsfs --disable-indevs --disable-outdevs

# x86 to jniLibs/x86/:

export CC=/home/willir/Android/Sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang
export CXX=/home/willir/Android/Sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang++

./configure --enable-cross-compile --enable-shared --disable-static --disable-programs \
    --cc=$CC --cxx=$CXX \
    --disable-debug \
    --disable-network \
    --disable-avfilter \
    --disable-encoders --disable-decoders --enable-encoder=aac --enable-decoder=aac --enable-encoder=mp2 --enable-decoder=mp2 \
    --disable-muxers --disable-demuxers --enable-muxer=adts --enable-muxer=data --enable-demuxer=data \
    --disable-parsers --enable-parser=aac --enable-parser=aac_latm \
    --disable-protocols --disable-bsfs --disable-indevs --disable-outdevs
