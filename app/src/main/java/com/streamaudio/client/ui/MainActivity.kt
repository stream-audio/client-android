package com.streamaudio.client.ui

import android.Manifest
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.content.pm.PackageManager
import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.os.IBinder
import android.support.v4.app.ActivityCompat
import android.support.v4.content.ContextCompat
import android.widget.*
import com.streamaudio.client.service.PlayService
import com.streamaudio.client.R
import java.util.*
import kotlin.collections.ArrayList
import kotlin.concurrent.timerTask

class MainActivity : AppCompatActivity() {
    companion object {
        private const val PERMISSION_ID: Int = 18616
        private const val UPDATE_AUDIO_DELAY_INTERVAL_MS: Long = 1000
    }

    private lateinit var mTvDelay: TextView
    private lateinit var mBtnStartStop: Button
    private var mTvDelayTimer: Timer? = null
    private var mServiceBound: PlayService.LocalBinder? = null

    private val connection = object : ServiceConnection {
        override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
            mServiceBound = service as PlayService.LocalBinder?
            displayCurrentState()
        }

        override fun onServiceDisconnected(name: ComponentName?) {
            mServiceBound = null
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        mTvDelay = findViewById(R.id.tvDelay)

        mBtnStartStop = findViewById(R.id.btn_play)
        mBtnStartStop.setOnClickListener {
            if (isPlaying()) {
                stop()
            } else {
                play()
            }
        }
        findViewById<ImageButton>(R.id.btn_delay_increase).setOnClickListener { increaseDelay() }
        findViewById<ImageButton>(R.id.btn_delay_decrease).setOnClickListener { decreaseDelay() }

        requestPermissions()
    }

    override fun onStart() {
        super.onStart()

        Intent(this, PlayService::class.java).also { intent ->
            bindService(intent, connection, Context.BIND_AUTO_CREATE)
        }
    }

    override fun onStop() {
        super.onStop()
        unbindService(connection)
        mServiceBound = null  // just in case
    }

    private fun isPlaying(): Boolean {
        return mServiceBound?.isPlaying() == true
    }

    private fun play() {
        val address = findViewById<EditText>(R.id.etRemoteAddress).text.toString()
        if (address.isEmpty()) {
            Toast.makeText(this, "Please fill the address field.", Toast.LENGTH_LONG).show()
            return
        }

        Intent(this, PlayService::class.java).also { intent ->
            intent.putExtra(PlayService.INTENT_ARG_URL, address)
            intent.putExtra(
                PlayService.INTENT_ARG_TYPE,
                PlayService.Type.PLAY
            )

            ContextCompat.startForegroundService(this, intent)
        }

        displayPlayingState()
    }

    private fun stop() {
        Intent(this, PlayService::class.java).also { intent ->
            intent.putExtra(
                PlayService.INTENT_ARG_TYPE,
                PlayService.Type.STOP
            )
            startService(intent)
        }

        displayStoppedState()
    }

    private fun displayCurrentState() {
        if (mServiceBound?.isPlaying() == true) {
            displayPlayingState()
        } else {
            displayStoppedState()
        }
    }

    private fun displayPlayingState() {
        mBtnStartStop.text = getText(R.string.stop_button)
        startSoundDelayThread()
    }

    private fun displayStoppedState() {
        stopSoundDelayThread()
        mBtnStartStop.text = getText(R.string.play_button)
        mTvDelay.text = getString(R.string.tv_default_delay)
    }

    private fun startSoundDelayThread() {
        stopSoundDelayThread()

        mTvDelayTimer = Timer().apply {
            schedule(timerTask {
                runOnUiThread {
                    updateSoundDelay()
                }
            }, 0, UPDATE_AUDIO_DELAY_INTERVAL_MS)
        }
    }

    private fun updateSoundDelay() {
        mServiceBound?.getDelayMs()?.let { delay -> setSoundDelay(delay) }
    }

    private fun setSoundDelay(delayMs: Long) {
        mTvDelay.text = getString(R.string.audioDelay, delayMs)
    }

    private fun stopSoundDelayThread() {
        mTvDelayTimer?.cancel()
        mTvDelayTimer = null
    }

    private fun increaseDelay() {
        mServiceBound?.increaseDelay()?.let { newDelay -> setSoundDelay(newDelay) }
    }

    private fun decreaseDelay() {
        mServiceBound?.decreaseDelay()?.let { newDelay -> setSoundDelay(newDelay) }
    }

    override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
        assert(requestCode == PERMISSION_ID)
        if (grantResults.any { it == PackageManager.PERMISSION_DENIED }) {
            finish()
        }
    }

    private fun requestPermissions() {
        requestPermissions(
            arrayOf(Manifest.permission.READ_EXTERNAL_STORAGE)
        )
    }

    private fun requestPermissions(permissions: Array<String>) {
        val permissionsToRequest = ArrayList<String>()
        for (permission in permissions) {
            val isGranted = ContextCompat.checkSelfPermission(this, permission) ==
                    PackageManager.PERMISSION_GRANTED
            if (!isGranted) {
                permissionsToRequest.add(permission)
            }
        }

        if (permissionsToRequest.isEmpty()) return

        ActivityCompat.requestPermissions(
            this, permissionsToRequest.toTypedArray(),
            PERMISSION_ID
        )
    }
}
