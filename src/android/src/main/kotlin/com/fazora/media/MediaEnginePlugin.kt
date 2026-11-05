package com.fazora.media

import android.app.Activity

class MediaEnginePlugin(private val activity: Activity) {
    fun verifyLibraries(): String = try {
        // Load dependencies in dependency order. These are packaged in this
        // plugin's jniLibs/<abi> directory by the Android CI build.
        System.loadLibrary("avutil")
        System.loadLibrary("swresample")
        System.loadLibrary("swscale")
        System.loadLibrary("avcodec")
        System.loadLibrary("avformat")
        "ok"
    } catch (error: UnsatisfiedLinkError) {
        error.message ?: "Unable to load FFmpeg native libraries"
    }
}
