package com.example.myapplication4

import android.content.Intent
import android.net.Uri
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.view.ViewGroup
import android.widget.RelativeLayout
import androidx.fragment.app.Fragment
import com.example.myapplication4.databinding.ActivityMainBinding
import com.example.myapplication4.databinding.MainMenuLayoutBinding
import com.example.myapplication4.network.ControlFlow
import com.example.myapplication4.network.get_current_user_id
import com.example.myapplication4.network.get_profile_pic_path
import com.example.myapplication4.network.set_socket_event

class MainMenuActivity : AppCompatActivity(){

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        var binding = MainMenuLayoutBinding.inflate(layoutInflater)
        val view = binding.root
        var pfp = get_profile_pic_path(this, get_current_user_id(this))
        if (!(pfp is ControlFlow.DefaultBehavior)) {
            binding.profileIcon.setImageURI(pfp as Uri)
        }
        setContentView(view)
        set_socket_event(this)

        var views : Array<Fragment> = arrayOf(ChatsMenu(this, layoutInflater), UsersMenu(this, layoutInflater), ProfileMenu.newInstance("a","b"))
       // views[1].offline_fill_view()
        binding.chatVec.setOnClickListener{
            supportFragmentManager.beginTransaction().replace(binding.container.id,views[0]).commit()
        }
        binding.peopleVec.setOnClickListener{
            supportFragmentManager.beginTransaction().replace(binding.container.id, views[1]).commit()
        }
        binding.profileIcon.setOnClickListener{
            supportFragmentManager.beginTransaction().replace(binding.container.id, views[2])
                .commit()
        }

        supportFragmentManager.beginTransaction().replace(binding.container.id, views[0]).commit()
    }
}