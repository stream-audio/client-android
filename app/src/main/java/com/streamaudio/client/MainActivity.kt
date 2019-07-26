package com.streamaudio.client

import android.Manifest
import android.content.pm.PackageManager
import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.support.v4.app.ActivityCompat
import android.support.v4.content.ContextCompat
import android.view.WindowManager
import android.widget.*
import java.util.*
import kotlin.collections.ArrayList
import kotlin.concurrent.timerTask

class MainActivity : AppCompatActivity() {
    private var mRustWrapper : RustWrapper? = null
    private lateinit var mTvDelay: TextView
    private var mTvDelayTimer: Timer? = null

    companion object {
        private const val PERMISSION_ID: Int = 18616;
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        mRustWrapper = RustWrapper()
        mTvDelay = findViewById(R.id.tvDelay)

        val hello = mRustWrapper!!.greeting("Anton")

        findViewById<TextView>(R.id.greeting).text = hello
        findViewById<Button>(R.id.btn_play).setOnClickListener {
            if (mRustWrapper!!.isPlaying()) {
                stop()
            } else {
                play()
            }
        }
        findViewById<ImageButton>(R.id.btn_delay_increase).setOnClickListener {increaseDelay()}
        findViewById<ImageButton>(R.id.btn_delay_decrease).setOnClickListener {decreaseDelay()}

        requestPermissions()
    }

    override fun onDestroy() {
        mRustWrapper?.destroy()
        mRustWrapper = null
        super.onDestroy()
    }

    private fun getPlayingRustWrapper(): RustWrapper? {
        return if (mRustWrapper?.isPlaying() == true) mRustWrapper else null
    }

    private fun play() {
        val address =  findViewById<EditText>(R.id.etRemoteAddress).text.toString()
        if (address.isEmpty()) {
            Toast.makeText(this, "Please fill the address field.", Toast.LENGTH_LONG).show()
            return;
        }

        mRustWrapper!!.play(address)
        //mRustWrapper!!.play(Environment.getExternalStorageDirectory().path + "/Music/audio.example.wav")

        val btn = findViewById<Button>(R.id.btn_play)
        btn.text = this.getText(R.string.stop_button)
        startSoundDelayThread()
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
    }

    private fun stop() {
        stopSoundDelayThread()

        mRustWrapper!!.stop()

        val btn = findViewById<Button>(R.id.btn_play)
        btn.text = this.getText(R.string.play_button)

        window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
    }

    private fun startSoundDelayThread() {
        stopSoundDelayThread()
        val timer = Timer()
        timer.schedule(timerTask{
            runOnUiThread {
                updateSoundDelay()
            }
        },0, 1000)
        mTvDelayTimer = timer
    }

    private fun updateSoundDelay() {
        val rustWrapper = mRustWrapper ?: return
        if (!rustWrapper.isPlaying()) return

        setSoundDelay(rustWrapper.getDelayMs())
    }

    private fun setSoundDelay(delayMs: Long) {
        mTvDelay.text = getString(R.string.audioDelay, delayMs)
    }

    private fun stopSoundDelayThread() {
        mTvDelayTimer?.cancel()
        mTvDelayTimer = null
    }

    private fun increaseDelay() {
        val newDelayMs = getPlayingRustWrapper()?.increaseDelay() ?: return
        setSoundDelay(newDelayMs)
    }

    private fun decreaseDelay() {
        val newDelayMs = getPlayingRustWrapper()?.decreaseDelay() ?: return
        setSoundDelay(newDelayMs)
    }

    override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
        assert(requestCode == PERMISSION_ID)
        if (grantResults.any { it == PackageManager.PERMISSION_DENIED }) {
            finish()
        }
    }

    private fun requestPermissions() {
        requestPermissions(arrayOf(Manifest.permission.READ_EXTERNAL_STORAGE, Manifest.permission.INTERNET))
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

        if (permissionsToRequest.isEmpty()) return;

        ActivityCompat.requestPermissions(this, permissionsToRequest.toTypedArray(),
            PERMISSION_ID
        )
    }
}
