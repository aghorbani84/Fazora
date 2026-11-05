package com.fazora.media

import android.app.Activity
import android.content.Intent
import android.os.Bundle

/**
 * Isolated SAF picker host. This avoids requiring the Dioxus/Wry host Activity
 * to extend ComponentActivity or expose an ActivityResultRegistry.
 */
class AudioPickerActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val picker = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "audio/*"
            addFlags(
                Intent.FLAG_GRANT_READ_URI_PERMISSION or
                    Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION
            )
        }

        try {
            startActivityForResult(picker, REQUEST_AUDIO)
        } catch (_: Throwable) {
            finish()
        }
    }

    @Deprecated("Deprecated in Android API; retained for compatibility with the helper Activity result flow")
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode != REQUEST_AUDIO) return

        if (resultCode == RESULT_OK) {
            val uri = data?.data
            if (uri != null) {
                try {
                    val granted = data.flags and Intent.FLAG_GRANT_READ_URI_PERMISSION
                    contentResolver.takePersistableUriPermission(uri, granted)
                } catch (_: SecurityException) {
                    // Some document providers grant temporary read access only.
                }
                PickedAudioStore.put(uri.toString())
            }
        }
        finish()
    }

    companion object {
        private const val REQUEST_AUDIO = 7042
    }
}

internal object PickedAudioStore {
    @Volatile private var pickedUri: String? = null

    @Synchronized
    fun put(uri: String) {
        pickedUri = uri
    }

    @Synchronized
    fun take(): String? {
        val result = pickedUri
        pickedUri = null
        return result
    }
}
