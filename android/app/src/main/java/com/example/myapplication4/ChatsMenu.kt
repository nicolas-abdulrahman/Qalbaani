package com.example.myapplication4

import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.util.Log
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.LinearLayout
import com.example.myapplication4.databinding.ChatsBinding
import com.example.myapplication4.databinding.MenuProfilePlaceholderBinding
import com.example.myapplication4.network.ChatRoom
import com.example.myapplication4.network.MyDatabase
import com.example.myapplication4.network.MyResult
import com.example.myapplication4.network.ResponseFuncBind
import com.example.myapplication4.network.SocketResponses
import com.example.myapplication4.network.get_current_user_id
import com.example.myapplication4.network.set_socket_event
import com.example.myapplication4.network.socky
import com.squareup.moshi.Moshi
import com.squareup.moshi.kotlin.reflect.KotlinJsonAdapterFactory
import kotlinx.coroutines.runBlocking

private const val ARG_PARAM1 = "param1"
private const val ARG_PARAM2 = "param2"



class ChatsMenu(var contextt: Context,var  inflater: LayoutInflater) : Fragment() {
    // TODO: Rename and change types of parameters
    var chats_bind : ChatsBinding? = null
    var responsee  = false
    val TAG = "sock"
    var views : MutableList<View> = mutableListOf()
    var been_here = false

    init{

       // chats_bind = ChatsBinding.inflate(layoutInflater)
        val json =  "{\"T\":\"gcr\",\"data\":\"\"}"


        var func : (Any?)-> Unit = { chat_rooms ->
            get_chat_rooms(chat_rooms!! as Array<ChatRoom>)
            responsee = true
        }

        val bind = ResponseFuncBind(SocketResponses.get_chat_rooms, func )
        socky.expcted_functions_promises.add(bind)

        socky.socketClient?.webSocket?.send(json)
    //    Log.d(TAG, "sent da json")
        offline_get_chat_room(inflater)
    }

    fun offline_get_chat_room(inflater: LayoutInflater){
        chats_bind = ChatsBinding.inflate(inflater)
        responsee = true
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
    }


    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        // Log.d(TAG, "gonna block..")
       // while(responsee == false){
       //     Thread.sleep(100)
      //  }




        return chats_bind!!.root
       // return inflater.inflate(R.layout.chats, container, false)
    }

    fun get_chat_rooms( chat_rooms: Array<ChatRoom>): MyResult {
        try {
            var db = MyDatabase(contextt)
            for (room in chat_rooms) {

                 //   Log.d(TAG, "da json value is ${room}")
                fill_room(room, db)

            }
            chats_bind = ChatsBinding.inflate(inflater)
            for (view in views){
                chats_bind!!.chatLayout.addView(view)
            }

            views.clear()
            requireView().invalidate()

            return MyResult.Ok
            //   var value = adapter.fromJson(response)
        } catch (e: Exception) {
            Log.e(TAG, e.toString())
            return MyResult.Err
        }
    }

    fun fill_room(room : ChatRoom, db: MyDatabase){
        var template = MenuProfilePlaceholderBinding.inflate(inflater)
        template.menuProfileName.text = room.name
        template.menuProfileStatus.text = room.description
        var current_user = get_current_user_id(contextt)
        for (user in room.users) {
            if (user != current_user) {
                var table = db.get_user(user)
                if (table == null) {
                    val json = "{\"T\":\"gu\",\"data\":\"$user\"}"
                    socky.socketClient?.webSocket?.send(json)
                }
                var a: (Any?) -> Unit = {
                    table = db.get_user(user)
                    if (table?.local_image_path == null) {
                        val json = "{\"T\":\"gi\",\"data\":\"${table?.image_path}\"}"

                        var b : (Any?) -> Unit = {image_path->
                            template.profileImageView.setImageURI(image_path as Uri)
                        }
                        var bind = ResponseFuncBind(SocketResponses.get_image, b)
                        socky.expcted_functions_promises.add(bind)
                        socky?.socketClient?.webSocket?.send(json)
                    }else
                        template.profileImageView.setImageURI(table?.local_image_path as Uri)
                }
                var bind = ResponseFuncBind(SocketResponses.get_user, a)
                socky.expcted_functions_promises.add(bind)
            }
        }
        views.add(template.root)
        template.root.setOnClickListener {
            var chat_room = ChatRoomActivity()
            var intent =  Intent(context,ChatRoom::class.java)
            startActivity(intent)
        }
    }
}