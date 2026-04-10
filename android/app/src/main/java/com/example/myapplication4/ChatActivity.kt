package com.example.myapplication4

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.TextView
import androidx.navigation.ui.AppBarConfiguration
import com.example.myapplication4.databinding.ActivityChatBinding
import androidx.navigation.findNavController
import androidx.navigation.ui.navigateUp
import androidx.navigation.ui.setupActionBarWithNavController
import androidx.navigation.ui.setupWithNavController
import com.google.android.material.navigation.NavigationView
import com.example.myapplication4.databinding.ActivityMainBinding
import androidx.drawerlayout.widget.DrawerLayout
import com.example.myapplication4.network.chattyApi
import androidx.lifecycle.viewModelScope

import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.launch


class
ChatActivity : AppCompatActivity() {
    private lateinit var binding: ActivityChatBinding
    private lateinit var appBarConfiguration: AppBarConfiguration
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityChatBinding.inflate(layoutInflater)
        val view = binding.root
        val text =  TextView(this)
        text.text ="HAKUNA MATATATAAA e lindo dizerrr"
        view.addView(text)
        setContentView(view)
        val toolbar = binding.chatLayout.toolbar
        val button = binding.chatLayout.toolbar.LeftIcon
        val drawer_layout = binding.drawerLayout

        button.setOnClickListener{
            drawer_layout.open()
        }

       // getPhotos()



    }

    fun getPhotos(){
        lifecycleScope.launch {
            try {
                val listResult = chattyApi.retrofitService.getUser()
                binding.chatLayout.toolbar.toolbarName.text = listResult.name

            }
            catch(e: Exception){
                binding.chatLayout.toolbar.toolbarName.text = e.message
            }
        }

    }

  //  override fun onSupportNavigateUp(): Boolean {
   //     val navController = findNavController(R.id.LeftIcon)

   //     return navController.navigateUp(appBarConfiguration) || super.onSupportNavigateUp()
   // }



}