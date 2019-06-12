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
        rustObj = createPlayerNative()
    }

    fun destroy() {
        destroyPlayerNative(rustObj)
        rustObj = 0
    }

    protected fun finalize() {
        if (rustObj != 0L) {
            destroy()
        }
    }

    fun play(fPath: String) {
        playNative(rustObj, fPath)
    }

    fun stop() {
        stopNative(rustObj)
    }

    fun isPlaying(): Boolean {
        return isPlayingNative(rustObj)
    }

    fun connect(addr: String) {
        connectNative(rustObj, addr)
    }

    external fun greeting(pattern: String): String

    private external fun createPlayerNative(): Long
    private external fun destroyPlayerNative(rustObj: Long): Long
    private external fun playNative(rustObj: Long, fPath: String)
    private external fun stopNative(rustObj: Long)
    private external fun isPlayingNative(rustObj: Long): Boolean
    private external fun connectNative(rustObj: Long, addr: String): Boolean
}
