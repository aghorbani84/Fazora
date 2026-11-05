# Third-party notices

## FFmpeg 9.0.2

FAZORA's Android native media backend uses FFmpeg 9.0.2, built as shared libraries under the GNU Lesser General Public License, version 2.1 or later (LGPL-2.1-or-later).

The FAZORA Android build profile:
- enables shared libraries and disables static libraries;
- does not enable GPL, version-3, or nonfree components;
- disables external-library autodetection;
- retains the exact upstream source archive, release signature, license files, configure command, and build log alongside each CI artifact.

The corresponding FFmpeg source archive and build metadata are included with Android CI artifacts. No claim is made that every codec/profile/container is supported on every device.

FFmpeg upstream:
- https://ffmpeg.org/
- https://ffmpeg.org/legal.html
- https://ffmpeg.org/download.html

Codec patent rights are separate from copyright licensing. This notice is not legal advice.
