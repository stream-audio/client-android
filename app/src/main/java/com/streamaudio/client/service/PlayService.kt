package com.streamaudio.client.service

import android.app.PendingIntent
import android.app.Service
import android.content.Intent
import android.os.Binder
import android.os.IBinder
import android.support.v4.app.NotificationCompat
import com.streamaudio.client.R
import com.streamaudio.client.service.rust.RustWrapper
import com.streamaudio.client.ui.MainActivity
import java.lang.NullPointerException

class PlayService : Service() {
    inner class LocalBinder : Binder() {
        fun isPlaying(): Boolean = mRustWrapper.isPlaying()
        fun getDelayMs(): Long = mRustWrapper.getDelayMs()
        fun increaseDelay(): Long = mRustWrapper.increaseDelay()
        fun decreaseDelay(): Long = mRustWrapper.decreaseDelay()

        fun isDelayFixed(): Boolean = mRustWrapper.isDelayFixed()
        fun fixDelayAt(delayMs: Long) = mRustWrapper.fixDelayAt(delayMs)
        fun unfixDelay() = mRustWrapper.unfixDelay()
    }

    internal enum class Type { PLAY, STOP }

    companion object {
        const val INTENT_ARG_TYPE = "type"
        const val INTENT_ARG_URL = "url"

        private const val NOTIFICATION_ID: Int = 23509
    }

    private lateinit var mRustWrapper: RustWrapper
    private var mBinder = LocalBinder()

    override fun onCreate() {
        mRustWrapper = RustWrapper()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        if (intent == null) throw NullPointerException("Intent cannot be null")

        when (intent.getSerializableExtra(INTENT_ARG_TYPE) as Type) {
            Type.PLAY -> startPlaying(intent)
            Type.STOP -> stopPlaying()
        }

        return START_NOT_STICKY
    }

    override fun onBind(intent: Intent?): IBinder? {
        return mBinder
    }

    override fun onDestroy() {
        if (mRustWrapper.isPlaying()) {
            mRustWrapper.stop()
        }
        mRustWrapper.destroy()
    }

    private fun startPlaying(intent: Intent) {
        if (mRustWrapper.isPlaying()) return

        val url = intent.getStringExtra(INTENT_ARG_URL)
        mRustWrapper.play(url)
        toForeground()
    }

    private fun stopPlaying() {
        if (mRustWrapper.isPlaying()) {
            mRustWrapper.stop()
        }

        stopForegroundNotification()
        stopSelf()
    }

    private fun toForeground() {
        val pendingIntent = Intent(this, MainActivity::class.java).let { intent ->
            PendingIntent.getActivity(this, 0, intent, 0)
        }

        val notification = NotificationCompat.Builder(this, StreamAudioApplication.CHANNEL_ID)
            .setContentTitle(getText(R.string.notification_title))
            .setContentText(getText(R.string.notification_text))
            .setSmallIcon(R.drawable.notification_small_icon)
            .setContentIntent(pendingIntent)
            .setTicker(getText(R.string.notification_ticker))
            .build()

        startForeground(NOTIFICATION_ID, notification)
    }

    private fun stopForegroundNotification() {
        stopForeground(true)
    }
}
