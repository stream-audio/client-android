package com.streamaudio.client

import android.Manifest
import android.content.pm.PackageManager
import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.support.v4.app.ActivityCompat
import android.support.v4.content.ContextCompat
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import android.widget.Toast

class MainActivity : AppCompatActivity() {
    var rustWrapper : RustWrapper? = null

    companion object {
        private const val PERMISSION_ID: Int = 18616;
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        rustWrapper = RustWrapper()
        val hello = rustWrapper!!.greeting("Anton")

        findViewById<TextView>(R.id.greeting).text = hello
        findViewById<Button>(R.id.btn_play).setOnClickListener {
            if (rustWrapper!!.isPlaying()) {
                stop()
            } else {
                play()
            }
        }

        requestPermissions()
    }

    override fun onDestroy() {
        rustWrapper?.destroy()
        rustWrapper = null
        super.onDestroy()
    }

    private fun play() {
        val address =  findViewById<EditText>(R.id.etRemoteAddress).text.toString()
        if (address.isEmpty()) {
            Toast.makeText(this, "Please fill the address field.", Toast.LENGTH_LONG).show()
            return;
        }

        rustWrapper!!.play(address)
        //rustWrapper!!.play(Environment.getExternalStorageDirectory().path + "/Music/audio.example.wav")

        val btn = findViewById<Button>(R.id.btn_play)
        btn.text = this.getText(R.string.stop_button)
    }

    private fun stop() {
        rustWrapper!!.stop()

        val btn = findViewById<Button>(R.id.btn_play)
        btn.text = this.getText(R.string.play_button)
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