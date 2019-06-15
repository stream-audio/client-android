package com.willir.audiosharing

class RustWrapper {
    companion object {
        private const val STATE_PLAYING = 1
        private const val STATE_STOPPED = 1

        init {
            System.loadLibrary("audio_sharing_android")
        }
    }

    private var rustObj: Long = 0;

    init {
        rustObj = createObjectNative()
    }

    fun destroy() {
        destroyObjectNative(rustObj)
        rustObj = 0
    }

    protected fun finalize() {
        if (rustObj != 0L) {
            destroy()
        }
    }

    fun play(addr: String) {
        playNative(rustObj, addr)
    }

    fun stop() {
        stopNative(rustObj)
    }

    fun isPlaying(): Boolean {
        return isPlayingNative(rustObj)
    }

    external fun greeting(pattern: String): String

    private external fun createObjectNative(): Long
    private external fun destroyObjectNative(rustObj: Long): Long
    private external fun playNative(rustObj: Long, addr: String)
    private external fun stopNative(rustObj: Long)
    private external fun isPlayingNative(rustObj: Long): Boolean
}
