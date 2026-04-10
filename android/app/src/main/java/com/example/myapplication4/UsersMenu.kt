package com.example.myapplication4

import android.content.Context
import android.net.Uri
import android.os.Bundle
import android.util.AttributeSet
import android.util.Log
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import com.example.myapplication4.databinding.MenuProfilePlaceholderBinding
import com.example.myapplication4.databinding.UsersBinding
import com.example.myapplication4.network.AbstractViewClass
import com.example.myapplication4.network.ControlFlow
import com.example.myapplication4.network.MyDatabase
import com.example.myapplication4.network.ResponseFuncBind
import com.example.myapplication4.network.SocketResponses
import com.example.myapplication4.network.UserAccount
import com.example.myapplication4.network.get_profile_pic_path
import com.example.myapplication4.network.set_socket_event
import com.example.myapplication4.network.socky
import kotlinx.coroutines.runBlocking

private const val ARG_PARAM1 = "param1"
private const val ARG_PARAM2 = "param2"

//class UsersMenuHandler(var contextt: Context) : AbstractViewClass(){
    //override var view = UsersMenu(contextt)
   // init{
       // view = UsersMenu(contextt)
      //  view.offline_fill_views()
  //  }

//}
class UsersMenu(var contextt: Context,  var inflater: LayoutInflater) : Fragment() {
    // TODO: Rename and change types of parameters
    private var param1: String? = null
    private var param2: String? = null
    var response = false
    var views : MutableList<View> = mutableListOf()
   // var offline_sync_done = false
    var binding :UsersBinding? = null
    init{
        val json =  "{\"T\":\"gau\",\"data\":\"\"}"

//        binding = UsersBinding.inflate(layoutInflater)
        var func : (Any?) -> Unit = { a->
            fill_views(contextt, a as Array<UserAccount>)
            response = true
        }
        var bind = ResponseFuncBind(SocketResponses.get_all_users, func)
        socky.expcted_functions_promises.add(bind)
        socky.socketClient?.webSocket?.send(json)
        offline_fill_views(inflater)
    }

    fun offline_fill_views(inflater: LayoutInflater){
        binding = UsersBinding.inflate(inflater)
        response = true


    }
    fun fill_views(context: Context, user_accounts: Array<UserAccount>){
      //  var db = MyDatabase(requireContext())
      //  var read = db.readableDatabase
      //  val SQL = "SELECT * FROM ${db.TABLE_NAME}"
       // var cursor = read.rawQuery(SQL, null)
        //while (cursor.moveToNext()){
        //    cursor.

        //}
        var db = MyDatabase(context)
        for (user in user_accounts){
           var  template = MenuProfilePlaceholderBinding.inflate(inflater)
            db.register_to_database(user.id, user.username, user.name, user.image, null)
            template.menuProfileName.text = user.username
            template.menuProfileStatus.text = user.status
            var image = get_profile_pic_path(context, user.id)
            when (image){
                is ControlFlow.Ok -> {template.profileImageView.setImageURI(image.value as Uri)}
                is ControlFlow.DefaultBehavior ->{}
                is ControlFlow.RequestServer ->{
                    val json =  "{\"T\":\"gi\",\"data\":\"${user.image}\"}"
                    var a : (Any?)-> Unit = { image_path->
                        template.profileImageView.setImageURI(image_path as Uri)


                    }
                    socky.socketClient?.webSocket?.send(json)
                }
                else ->{}
            }
            views.add(template.root)
          //  var binding = UsersBinding.inflate(layoutInflater)


        }
        binding = UsersBinding.inflate(inflater)
        for (view in views ){
            binding!!.chatLayout.addView(view)
        }
        views.clear()
        requireView().invalidate()

    }

    override fun onInflate(context: Context, attrs: AttributeSet, savedInstanceState: Bundle?) {
        super.onInflate(context, attrs, savedInstanceState)
        print("aaa")
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        //socky.expcted_functions_promises.add(bind)
       // socky?.socketClient?.webSocket?.send(json)
    }

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
       // runBlocking {
  //      while(response == false){
    //        Thread.sleep(100)
    //    }
      //  }
       return  binding!!.root
       // Log.d("sock", "response is $response")
        // Inflate the layout for this fragment
        //return inflater.inflate(R.layout.users, container, false)
    }
}