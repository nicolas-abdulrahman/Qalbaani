package com.example.myapplication4

import SocketClient
import android.content.Intent
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.view.ViewGroup
import android.widget.RelativeLayout
import android.widget.Toast
import androidx.lifecycle.lifecycleScope
import com.example.myapplication4.databinding.ActivityMainBinding
import com.example.myapplication4.network.LoginBody
import com.example.myapplication4.network.chattyApi
import com.example.myapplication4.network.connect_to_ws
import com.example.myapplication4.network.idToken
import com.example.myapplication4.network.socky
import kotlinx.coroutines.launch

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding
    val TAG = "Activity"

    fun go_to_main_menu(){
        var intent =  Intent(this,MainMenuActivity::class.java)
        intent.addFlags(Intent.FLAG_ACTIVITY_CLEAR_TASK or Intent.FLAG_ACTIVITY_NEW_TASK)
        startActivity(intent)
    }

    fun send_form(): idToken?{
        try{
            var id_token: idToken? = null
            lifecycleScope.launch {
                val username = binding.login.usernameField.toString()
                val password = binding.login.passwordField.toString()
                val body = LoginBody(username, password)
                id_token = chattyApi.retrofitService.logIn(body)

            }
            return id_token
        }catch (e: Exception){
            Log.e(TAG, e.toString())
            return null
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        val view = binding.root
        if (socky.socketClient != null){
            go_to_main_menu()
            return
        }
        val sharedPref = applicationContext.getSharedPreferences("login_data", MODE_PRIVATE)
        val token = sharedPref.getString("token", null)
        val id = sharedPref.getString("id", null) as UInt
        if (token!= null && id!= null) {

            var socketClient = SocketClient(id, token)
            socketClient?.execute()
            socky.socketClient = socketClient
            go_to_main_menu()
            return
        }

        //val params = view.layoutParams as ViewGroup.MarginLayoutParams
      //  params.setMargins(0,50,200,200)
        setContentView(view)

        //view.setOnClickListener()
        view.setOnClickListener {
            binding.login.texty.text = "goodbye worldddd"
        }

        binding.login.loginButton.setOnClickListener{
            val id_token = send_form()
            if (id_token == null){
                Toast.makeText(this, "couldnt login \uD83D\uDE2D", Toast.LENGTH_LONG)
            }else {
                connect_to_ws(id_token)
            }
            //startActivity(Intent(this,ChatActivity::class.java) )
        }
        binding.login.OrRegister.setOnClickListener{
            startActivity(Intent(this,Register::class.java))
        }
        //startActivity(Intent(this,Register::class.java) )
    }


}