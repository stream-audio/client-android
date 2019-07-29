package com.streamaudio.client.service.rust

import android.util.Log

class RustCb {
    companion object {
        const val TAG: String = "StreamAudio"
    }

    fun onDelayChangedMs(delay: Long) {
        Log.d(TAG, "Delay: $delay")
    }
}
