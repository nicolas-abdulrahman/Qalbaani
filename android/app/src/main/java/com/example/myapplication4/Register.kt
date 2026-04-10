package com.example.myapplication4

import SocketClient
import SocketEventListener
import android.R
import android.annotation.SuppressLint
import android.app.ActionBar.LayoutParams
import android.app.Activity
import android.content.Intent
import android.content.SharedPreferences
import android.graphics.Color
import android.graphics.drawable.GradientDrawable
import android.graphics.drawable.ShapeDrawable
import android.graphics.drawable.shapes.Shape
import android.net.Uri
import android.os.Bundle
import android.util.Log
import android.util.MutableInt
import android.view.MotionEvent
import android.view.View
import android.widget.LinearLayout
import android.widget.Toast
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.constraintlayout.widget.ConstraintLayout
import androidx.constraintlayout.widget.ConstraintSet
import androidx.lifecycle.lifecycleScope
import com.example.myapplication4.databinding.ActivityRegisterBinding
import com.example.myapplication4.network.BASE_URL
import com.example.myapplication4.network.ImageData
import com.example.myapplication4.network.MyDatabase
import com.example.myapplication4.network.MyResult
import com.example.myapplication4.network.RegisterBody
import com.example.myapplication4.network.RegisterResponse
import com.example.myapplication4.network.chattyApi
import com.example.myapplication4.network.idToken
import com.example.myapplication4.network.set_socket_event
import com.example.myapplication4.network.socky
import com.skydoves.colorpickerview.ColorPickerView
import com.skydoves.colorpickerview.listeners.ColorListener
import com.skydoves.colorpickerview.sliders.BrightnessSlideBar
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import retrofit2.http.Body
import java.io.File
import java.net.Socket
import java.net.SocketAddress
import java.util.Objects
import kotlin.reflect.jvm.internal.impl.load.kotlin.JvmType
import kotlin.reflect.jvm.internal.impl.types.TypeCheckerState.SupertypesPolicy.None


class Register : AppCompatActivity() {
    var mScrollable = true
    lateinit var binding: ActivityRegisterBinding
    var  color_wheel_id: Int = -1
    var wheel_is_open = false
    lateinit var color_wheel_layout: LinearLayout
    lateinit var color_wheel: ColorPickerView
    var name_color: Int = -1
    lateinit var context: Register
    var socketClient : SocketClient? = null
    val TAG = "socky"

    class ViewBind(var view: View, var func: (Int)-> Unit){

    }


    @SuppressLint("ResourceType")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityRegisterBinding.inflate(layoutInflater)
        var view  = binding.constraint
       // val layoutParams = LayoutParams(
    //        500,
      //      500
      //  )
        var color_binding = com.example.myapplication4.databinding.ColorPickerTemplateBinding.inflate(layoutInflater)
        color_wheel_layout   = color_binding.root
        color_wheel = color_binding.colorPickerView
        var bright_slider = color_binding.brightnessSlide
        color_wheel.attachBrightnessSlider(bright_slider)
        var resultLauncher =
            registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
            if (result.resultCode == Activity.RESULT_OK) {
                // There are no request codes
                val data: Intent = result.data as Intent
                val uri = data.data
              //binding.profileImageViewIcon.setImageURI(uri)

                binding.profileImageView.setImageURI(uri)
                binding.profileImageView.setTag(uri)
                binding.fragy.imageView.setImageURI(uri)
            }
        }
        context = this
        binding.profileImageViewIcon.setOnClickListener{
            val intent = Intent(Intent.ACTION_GET_CONTENT)
            intent.type = "image/*" // Limiting to image types only
            resultLauncher.launch(intent)
        }
      //  name_color =  Color.parseColor("#FF00FF")
     //   layoutParams.topMargin = 100
       // view.layoutParams = layoutParams
        val myLambda: (Int) -> Unit = { number: Int ->
            println("Received number: $number")
        }
        var viewBinds = arrayOf(
            ViewBind(binding.nameColorSquare, { number: Int ->
                binding.fragy.name.setTextColor(number)
                binding.fragy.imageView.borderColor = number
                binding.profileImageView.borderColor = number
            }),
            ViewBind(binding.bubbleColorSquare, {number: Int ->
               binding.fragy.relativeLayout3.setBackgroundColor(number)

            }),
            ViewBind(binding.textColorSquare, {number: Int ->
                binding.fragy.textMessage.setTextColor(number)
               binding.fragy.textMessage.redraw()
               binding.fragy.textMessage.postInvalidate()
            }),
            ViewBind(binding.rpColorSquare, {number: Int ->
                binding.fragy.textMessage.custom_color = number
                binding.fragy.textMessage.redraw()
                // binding.fragy.textMessage.refreshDrawableState()
            })
        )
        for (viewBind in viewBinds)  {
            viewBind.view.setOnClickListener {
                if (wheel_is_open) {
                    view.removeView(color_wheel_layout)
                    wheel_is_open = false
                } else {
                    var v: View = viewBind.view.parent as View
                    spawn_color(v, viewBind)
                    wheel_is_open = true
                }
                // binding.root.mScrollable = !binding.root.mScrollable
            }
        }


        binding.registerButton.setOnClickListener{
            handle_connection()
        }

        binding.OrLogin.setOnClickListener{
            startActivity(Intent(this,MainActivity::class.java))

        }


        setContentView(binding.root)
    }

    fun send_form(): RegisterResponse?{
        var name: String? = binding.nameField.text.toString()
        if (name == ""){
            name = null
        }

        val username = binding.usernameField.text.toString()
        var image_data: ImageData? = null
        if(binding.profileImageView.getTag()!= null){
            try {
                val image_uri = binding.profileImageView.getTag() as Uri
                val file = File(image_uri.toString())
                var image_bytes = file.readBytes()
                image_data = ImageData(image_uri.lastPathSegment.toString(), image_bytes)
            }catch (e:Exception){
                Log.e("socky", e.toString())
            }
        }
        binding.profileImageView.getTag()
        val password = binding.passwordField.text.toString()
        val body = RegisterBody(username, password, name, image_data)
        var register_user : RegisterResponse? = null
        runBlocking {
            try {
                register_user = chattyApi.retrofitService.signIn(body)

            }
            catch(e:Exception){
                Log.e(TAG, e.toString())
            }
        }
        return register_user
    }

    fun handle_connection(){
            try {
                var register_response = send_form() ?: return
                val id = register_response.id_token.id
                val token = register_response.id_token.token
                val sharedPref = applicationContext.getSharedPreferences("login_data", MODE_PRIVATE)
                var sharedEdit = sharedPref.edit()
                sharedEdit.putString("id", id.toString())
                sharedEdit.putString("token", token)
                sharedEdit.commit()
                Toast.makeText(context, "signed in $id, $token", Toast.LENGTH_LONG).show()
               // val sharedPref2 = applicationContext.getSharedPreferences("stuff", MODE_WORLD_WRITEABLE)
                socketClient = SocketClient(id, token)
                socketClient?.execute()
                set_socket_event(context)
                val local_image_path = binding.profileImageView.getTag() as String?
                val img_path = register_response.img_path
                var db = MyDatabase(this)
                db.register_to_database(id, register_response.username, register_response.name, img_path, local_image_path)


                Toast.makeText(context, "connected to socket?", Toast.LENGTH_LONG).show()
                socky.socketClient = socketClient
                var intent =  Intent(context,MainMenuActivity::class.java)
                intent.addFlags(Intent.FLAG_ACTIVITY_CLEAR_TASK or Intent.FLAG_ACTIVITY_NEW_TASK)
                startActivity(intent)
              //  val listResult = chattyApi.retrofitService.getUser()
             //   print(listResult)
            }
            catch(e: Exception){
                print(e)
            }


    }

    fun spawn_color(at: View, target: ViewBind){
        var view  = binding.constraint
        color_wheel.setColorListener(object : ColorListener{
            override fun onColorSelected(color: Int, fromUser: Boolean) {
                //val drawable = target.background as Drawa
                target.func(color)
                target.view.setBackgroundColor(color)

                // drawable.setColor(color)
            }
        })
        view.addView(color_wheel_layout)
        val set: ConstraintSet = ConstraintSet()
        set.clone(view)
        set.connect(color_wheel_layout.id, ConstraintSet.TOP, at.id, ConstraintSet.BOTTOM, 25)
        set.connect(color_wheel_layout.id, ConstraintSet.LEFT, view.id, ConstraintSet.LEFT, 0)
        set.connect(color_wheel_layout.id, ConstraintSet.RIGHT, view.id, ConstraintSet.RIGHT, 0)
        //  set.connect(binding2.imageView.id, ConstraintSet.TOP, binding.imageView.id, ConstraintSet.BOTTOM)
        set.applyTo(view)
    }


    override fun onTouchEvent(event: MotionEvent?): Boolean {
        return super.onTouchEvent(event)
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
    }
}