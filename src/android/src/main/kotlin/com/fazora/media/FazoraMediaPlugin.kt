package com.fazora.media

import android.app.Activity
import android.net.Uri
import android.os.Looper
import android.content.Intent
import androidx.activity.ComponentActivity
import androidx.activity.result.contract.ActivityResultContracts
import androidx.media3.common.MediaItem
import androidx.media3.common.PlaybackException
import androidx.media3.common.Player
import androidx.media3.exoplayer.ExoPlayer
import org.json.JSONObject
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Thin synchronous JNI facade over Media3.
 *
 * All ExoPlayer access is marshalled onto Android's main thread. Rust passes
 * scalar values as strings to avoid known numeric-argument issues in some
 * Manganis FFI releases. State is returned as JSON for a stable bridge ABI.
 */
class FazoraMediaPlugin(private val activity: Activity) {
    private var player: ExoPlayer? = null
    private var lastError: String? = null
    private var pickedUri: Uri? = null
    private val componentActivity = activity as ComponentActivity
    private val audioPicker = componentActivity.activityResultRegistry.register(
        "fazora-audio-picker-${activity.hashCode()}",
        ActivityResultContracts.OpenDocument()
    ) { uri ->
        if (uri != null) {
            try {
                activity.contentResolver.takePersistableUriPermission(
                    uri,
                    Intent.FLAG_GRANT_READ_URI_PERMISSION
                )
            } catch (_: SecurityException) {
                // Some providers grant only temporary access; the URI remains usable
                // for the current process but may need to be selected again later.
            }
            pickedUri = uri
        }
    }

    private fun <T> onMainThread(block: () -> T): T {
        if (Looper.myLooper() == Looper.getMainLooper()) return block()

        val latch = CountDownLatch(1)
        var result: Any? = null
        var failure: Throwable? = null
        activity.runOnUiThread {
            try {
                result = block()
            } catch (error: Throwable) {
                failure = error
            } finally {
                latch.countDown()
            }
        }
        if (!latch.await(10, TimeUnit.SECONDS)) {
            throw IllegalStateException("Timed out waiting for Android main thread")
        }
        failure?.let { throw it }
        @Suppress("UNCHECKED_CAST")
        return result as T
    }

    private fun ensurePlayer(): ExoPlayer {
        return player ?: ExoPlayer.Builder(activity.applicationContext).build().also { created ->
            created.addListener(object : Player.Listener {
                override fun onPlayerError(error: PlaybackException) {
                    lastError = error.message ?: "Media3 playback error"
                }

                override fun onPlaybackStateChanged(playbackState: Int) {
                    if (playbackState == Player.STATE_READY) lastError = null
                }
            })
            player = created
        }
    }

    private fun response(ok: Boolean, message: String? = null): String {
        val json = JSONObject()
        json.put("ok", ok)
        if (message != null) json.put("message", message)
        return json.toString()
    }

    fun openAudioPickerJson(): String = onMainThread {
        try {
            audioPicker.launch(arrayOf("audio/*"))
            response(true)
        } catch (error: Throwable) {
            response(false, error.message ?: "Unable to open audio picker")
        }
    }

    fun takePickedUriJson(): String = onMainThread {
        val uri = pickedUri
        pickedUri = null
        val json = JSONObject()
        json.put("uri", uri?.toString())
        json.toString()
    }

    fun initializeJson(): String = onMainThread {
        try {
            ensurePlayer()
            response(true)
        } catch (error: Throwable) {
            response(false, error.message ?: "Unable to initialize Media3")
        }
    }

    fun setMediaUriJson(uriValue: String): String = onMainThread {
        try {
            val value = uriValue.trim()
            if (value.isEmpty()) return@onMainThread response(false, "Media URI is empty")
            val uri = Uri.parse(value)
            if (uri.scheme != "content" && uri.scheme != "file") {
                return@onMainThread response(false, "Unsupported media URI scheme")
            }
            val mediaPlayer = ensurePlayer()
            lastError = null
            mediaPlayer.setMediaItem(MediaItem.fromUri(uri))
            mediaPlayer.prepare()
            response(true)
        } catch (error: Throwable) {
            response(false, error.message ?: "Unable to prepare media")
        }
    }

    fun playJson(): String = onMainThread {
        try {
            ensurePlayer().play()
            response(true)
        } catch (error: Throwable) {
            response(false, error.message ?: "Unable to start playback")
        }
    }

    fun pauseJson(): String = onMainThread {
        try {
            player?.pause()
            response(true)
        } catch (error: Throwable) {
            response(false, error.message ?: "Unable to pause playback")
        }
    }

    fun stopJson(): String = onMainThread {
        try {
            player?.stop()
            player?.clearMediaItems()
            response(true)
        } catch (error: Throwable) {
            response(false, error.message ?: "Unable to stop playback")
        }
    }

    fun seekToJson(positionMsValue: String): String = onMainThread {
        try {
            val positionMs = positionMsValue.toLongOrNull()
                ?: return@onMainThread response(false, "Invalid seek position")
            if (positionMs < 0L) return@onMainThread response(false, "Seek position must be non-negative")
            player?.seekTo(positionMs)
            response(true)
        } catch (error: Throwable) {
            response(false, error.message ?: "Unable to seek")
        }
    }

    fun setVolumeJson(volumeValue: String): String = onMainThread {
        try {
            val volume = volumeValue.toFloatOrNull()
                ?: return@onMainThread response(false, "Invalid volume")
            if (!volume.isFinite()) return@onMainThread response(false, "Volume must be finite")
            ensurePlayer().volume = volume.coerceIn(0f, 1f)
            response(true)
        } catch (error: Throwable) {
            response(false, error.message ?: "Unable to set volume")
        }
    }

    fun getStateJson(): String = onMainThread {
        val mediaPlayer = player
        val json = JSONObject()
        json.put("engineAvailable", mediaPlayer != null)
        json.put("isPlaying", mediaPlayer?.isPlaying ?: false)
        json.put("playWhenReady", mediaPlayer?.playWhenReady ?: false)
        json.put("playbackState", mediaPlayer?.playbackState ?: Player.STATE_IDLE)
        json.put("positionMs", mediaPlayer?.currentPosition ?: 0L)
        json.put("durationMs", (mediaPlayer?.duration ?: 0L).coerceAtLeast(0L))
        json.put("volume", mediaPlayer?.volume ?: 1f)
        val error = lastError
        if (error != null) json.put("error", error)
        json.toString()
    }

    /**
     * Single JSON entry point used by the Rust bridge. Keeping one JNI method
     * avoids repeated method lookups and allows class loading through the
     * Activity's application ClassLoader on Android.
     */
    fun invokeJson(requestJson: String): String {
        return try {
            val request = JSONObject(requestJson)
            val method = request.getString("method")
            when (method) {
                "initialize" -> initializeJson()
                "openAudioPicker" -> openAudioPickerJson()
                "takePickedUri" -> takePickedUriJson()
                "setMediaUri" -> setMediaUriJson(request.getString("value"))
                "play" -> playJson()
                "pause" -> pauseJson()
                "stop" -> stopJson()
                "seekTo" -> seekToJson(request.getString("value"))
                "setVolume" -> setVolumeJson(request.getString("value"))
                "getState" -> getStateJson()
                else -> response(false, "Unknown native command: $method")
            }
        } catch (error: Throwable) {
            response(false, error.message ?: "Invalid native command")
        }
    }

    fun releaseJson(): String = onMainThread {
        try {
            player?.release()
            player = null
            lastError = null
            response(true)
        } catch (error: Throwable) {
            response(false, error.message ?: "Unable to release Media3")
        }
    }
}
