package com.iskander.sendtextadb

import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.view.KeyEvent
import android.view.View
import android.widget.Button
import android.widget.EditText

private const val TAG = "sendtextadb"
class MainActivity : AppCompatActivity() {
    private lateinit var editText: EditText;
    private lateinit var sendText: Button;
    private lateinit var sendCtrlEnter: Button;

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        editText = findViewById(R.id.editText)
        sendText = findViewById(R.id.sendText)
        sendCtrlEnter = findViewById(R.id.sendCtrlEnter)

        editText.setOnKeyListener(View.OnKeyListener { _, keyCode, event ->
            if(keyCode == KeyEvent.KEYCODE_ENTER && event.action == KeyEvent.ACTION_UP) {
                sendTextToAdb()
                return@OnKeyListener true
            }
            false
        })
        sendText.setOnClickListener {
            sendTextToAdb()
        }
        sendCtrlEnter.setOnClickListener {
            Log.i(TAG, "Ctrl+Enter")
        }
    }

    private fun sendTextToAdb() {
        if(editText.text.isNotEmpty()) {
            Log.i(TAG, "text:${editText.text}")
            editText.text.clear()
        }
    }
}