#!/bin/bash

PREFIX="$(pwd)/lib/x86_64-unknown-linux-gnu"
LD_LIBRARY_PATH="${PREFIX}/lib:${LD_LIBRARY_PATH}"
CFLAGS="-fPIE"

# Build dav1d
mkdir -p ./build && cd ./build
BASEDIR="$(pwd)"

git clone --depth=1 https://code.videolan.org/videolan/dav1d.git
git clone --depth=1 https://chromium.googlesource.com/webm/libvpx
git clone --depth=1 git://source.ffmpeg.org/ffmpeg.git
wget https://sourceforge.net/projects/lame/files/lame/3.100/lame-3.100.tar.gz
wget https://downloads.xiph.org/releases/opus/opus-1.4.tar.gz
wget http://downloads.xiph.org/releases/ogg/libogg-1.3.0.tar.gz
wget http://downloads.xiph.org/releases/vorbis/libvorbis-1.3.3.tar.gz

for file in $(ls -1 | grep .tar.gz); do
    tar zxvf "${file}"
    rm "${file}"
done

cd "${BASEDIR}/dav1d"
mkdir build
cd build
meson setup .. \
    --default-library=static \
    --buildtype plain \
    --prefer-static \
    --prefix="${PREFIX}"
ninja
ninja install

cd "${PREFIX}"
mv lib64 lib

cd "${BASEDIR}/lame-3.100"
./configure \
    --disable-frontend \
    --with-pic --enable-pic \
    --disable-shared \
    --enable-static \
    --prefix="${PREFIX}"
make
make install

cd "${BASEDIR}/opus-1.4"
./configure  \
    --with-pic --enable-pic \
    --disable-shared \
    --enable-static \
    --prefix="${PREFIX}"
make
make install

cd "${BASEDIR}/libogg-1.3.0"
./configure \
    --with-pic --enable-pic \
    --disable-shared \
    --enable-static \
    --prefix="${PREFIX}"
make
make install

cd "${BASEDIR}/libvorbis-1.3.3"
./configure \
    --with-pic --enable-pic \
    --disable-shared \
    --enable-static \
    --prefix="${PREFIX}"
make
make install

cd "${BASEDIR}/libvpx"
./configure \
    --with-pic --enable-pic \
    --enable-vp9 \
    --disable-shared \
    --enable-static \
    --prefix="${PREFIX}"
make
make install

cd "${BASEDIR}/ffmpeg"
./configure \
    --disable-debug \
    --enable-stripping \
    --enable-static \
    --pkg-config-flags="--static" \
    --extra-cflags="-I${PREFIX}/include" \
    --extra-ldflags="-L${PREFIX}/lib" \
    --extra-libs=-lpthread \
    --extra-libs=-lm \
    --disable-shared \
    --disable-autodetect \
    --disable-programs \
    --enable-pic \
    --prefix="${PREFIX}" \
    --enable-libvpx \
    --enable-libvorbis \
    --enable-libdav1d \
    --enable-libmp3lame \
    --enable-libopus
make
make install

echo "FINISHED"
