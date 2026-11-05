package com.fazora.media

import android.app.Activity

class MediaEnginePlugin(private val activity: Activity) {
    fun verifyLibraries(): Boolean = try {
        // Load dependencies in dependency order. These are packaged in this
        // plugin's jniLibs/<abi> directory by the Android CI build.
        System.loadLibrary("avutil")
        System.loadLibrary("swresample")
        System.loadLibrary("swscale")
        System.loadLibrary("avcodec")
        System.loadLibrary("avformat")
        true
    } catch (_: UnsatisfiedLinkError) {
        false
    }
}
