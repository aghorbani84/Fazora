#include <libavutil/avutil.h>

/*
 * Narrow C ABI boundary for FAZORA.
 * Keep FFmpeg headers and structs inside native code; Rust receives only
 * stable primitive/string results from this shim.
 */
const char *fazora_ffmpeg_version(void) {
    return av_version_info();
}
