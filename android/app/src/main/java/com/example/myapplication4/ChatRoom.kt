package com.example.myapplication4

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication4.network.ChatRoom

class ChatRoomActivity : AppCompatActivity(){

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        var intent =  Intent(this, ChatActivity::class.java)
        startActivity(intent)
    }
}