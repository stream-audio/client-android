package com.streamaudio.client.service.rust

class RustWrapper {
    companion object {
        init {
            System.loadLibrary("avutil")
            System.loadLibrary("swresample")
            System.loadLibrary("avcodec")
            System.loadLibrary("avformat")

            System.loadLibrary("audio_sharing_android")
        }
    }

    private var rustObj: Long = 0
    private var rustCb = RustCb()

    init {
        rustObj = createObjectNative(rustCb)
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

    fun getDelayMs(): Long {
        return getDelayMsNative(rustObj)
    }

    fun increaseDelay(): Long {
        return increaseDelayNative(rustObj)
    }

    fun decreaseDelay(): Long {
        return decreaseDelayNative(rustObj)
    }

    external fun greeting(pattern: String): String

    private external fun createObjectNative(cb: RustCb): Long
    private external fun destroyObjectNative(rustObj: Long)
    private external fun playNative(rustObj: Long, addr: String)
    private external fun stopNative(rustObj: Long)
    private external fun isPlayingNative(rustObj: Long): Boolean
    private external fun getDelayMsNative(rustObj: Long): Long
    private external fun increaseDelayNative(rustObj: Long): Long
    private external fun decreaseDelayNative(rustObj: Long): Long
}
