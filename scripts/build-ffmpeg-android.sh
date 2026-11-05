#!/usr/bin/env bash
set -Eeuo pipefail

FFMPEG_VERSION=9.0.2
FFMPEG_FINGERPRINT=FCF986EA15E6E293A5644F10B4322F04D67658D8
ANDROID_API=${ANDROID_API:-26}
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK="${FFMPEG_WORK_DIR:-$ROOT/.cache/ffmpeg}"
OUT_ROOT="${FFMPEG_OUTPUT_DIR:-$ROOT/native/ffmpeg/prebuilt/android}"
JOBS="${BUILD_JOBS:-2}"

[[ $# -eq 1 ]] || { echo "Usage: $0 <arm64-v8a|x86_64>" >&2; exit 2; }
case "$1" in
  arm64-v8a) ABI=arm64-v8a; ARCH=aarch64; TRIPLE=aarch64-linux-android ;;
  x86_64) ABI=x86_64; ARCH=x86_64; TRIPLE=x86_64-linux-android ;;
  *) echo "Unsupported ABI: $1" >&2; exit 2 ;;
esac

: "${ANDROID_NDK_HOME:?Set ANDROID_NDK_HOME}"
TOOLCHAIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64"
[[ -d "$TOOLCHAIN" ]] || { echo "NDK Linux toolchain not found" >&2; exit 1; }
mkdir -p "$WORK" "$OUT_ROOT/$ABI"
cd "$WORK"

TARBALL="ffmpeg-$FFMPEG_VERSION.tar.xz"
BASE=https://ffmpeg.org/releases
curl --fail --location --retry 3 "$BASE/$TARBALL" -o "$TARBALL"
curl --fail --location --retry 3 "$BASE/$TARBALL.asc" -o "$TARBALL.asc"
curl --fail --location --retry 3 https://ffmpeg.org/ffmpeg-devel.asc -o ffmpeg-devel.asc

gpg --batch --import ffmpeg-devel.asc
ACTUAL="$(gpg --batch --with-colons --fingerprint "$FFMPEG_FINGERPRINT" | awk -F: '$1 == "fpr" {print $10; exit}')"
[[ "$ACTUAL" == "$FFMPEG_FINGERPRINT" ]] || { echo "Signing-key fingerprint mismatch" >&2; exit 1; }
gpg --batch --verify "$TARBALL.asc" "$TARBALL"

rm -rf "ffmpeg-$FFMPEG_VERSION"
tar -xJf "$TARBALL"
cd "ffmpeg-$FFMPEG_VERSION"

PREFIX="$OUT_ROOT/$ABI"
CC="$TOOLCHAIN/bin/$TRIPLE$ANDROID_API-clang"
CXX="$TOOLCHAIN/bin/$TRIPLE$ANDROID_API-clang++"
for tool in "$CC" "$CXX" "$TOOLCHAIN/bin/llvm-ar" "$TOOLCHAIN/bin/llvm-ranlib"; do
  [[ -x "$tool" ]] || { echo "Missing NDK tool: $tool" >&2; exit 1; }
done

ARGS=(
  "--prefix=$PREFIX" "--target-os=android" "--arch=$ARCH"
  "--enable-cross-compile" "--sysroot=$TOOLCHAIN/sysroot"
  "--cc=$CC" "--cxx=$CXX" "--ar=$TOOLCHAIN/bin/llvm-ar"
  "--ranlib=$TOOLCHAIN/bin/llvm-ranlib" "--strip=$TOOLCHAIN/bin/llvm-strip"
  "--nm=$TOOLCHAIN/bin/llvm-nm"
  "--enable-shared" "--disable-static" "--enable-pic" "--enable-small"
  "--disable-autodetect" "--disable-programs" "--disable-doc" "--disable-debug"
  "--disable-network" "--disable-devices" "--disable-avdevice"
  "--disable-avfilter" "--disable-postproc"
  "--enable-mediacodec" "--enable-jni"
  "--extra-cflags=-fPIC"
  "--extra-ldflags=-Wl,-z,max-page-size=16384"
)
printf '%q ' ./configure "${ARGS[@]}" > "$PREFIX/configure-command.txt"
printf '\n' >> "$PREFIX/configure-command.txt"
./configure "${ARGS[@]}"

if grep -Eq '^#define CONFIG_(GPL|NONFREE) 1$' config.h; then
  echo "Refusing GPL/nonfree FFmpeg configuration" >&2; exit 1
fi
grep -Eq '^#define CONFIG_SHARED 1$' config.h || { echo "Shared build missing" >&2; exit 1; }
grep -Eq '^#define CONFIG_STATIC 0$' config.h || { echo "Static build unexpectedly enabled" >&2; exit 1; }

make -j"$JOBS"
make install
for library in avutil avcodec avformat swresample swscale; do
  compgen -G "$PREFIX/lib/lib$library.so*" >/dev/null || {
    echo "Missing shared library: lib$library.so" >&2; exit 1;
  }
done

cp LICENSE.md COPYING.LGPLv2.1 "$PREFIX/"
cp "$WORK/$TARBALL" "$PREFIX/"
cp "$WORK/$TARBALL.asc" "$PREFIX/"
cp ffbuild/config.log "$PREFIX/ffmpeg-config.log"
printf '%s\n' "$FFMPEG_VERSION" > "$PREFIX/FFMPEG_VERSION"
printf '%s\n' "$ABI" > "$PREFIX/ANDROID_ABI"
echo "Built LGPL-profile FFmpeg $FFMPEG_VERSION for $ABI at $PREFIX"
