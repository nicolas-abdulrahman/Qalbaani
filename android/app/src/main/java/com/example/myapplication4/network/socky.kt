package com.example.myapplication4.network

import SocketClient
import SocketEventListener
import android.content.Context
import android.util.Log
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import androidx.fragment.app.Fragment
import com.google.gson.Gson
import handly_socket

val TAG = "sock"

enum class SocketResponses{
    get_user,
    get_image,
    get_chat_rooms,
    get_all_users,
}
class ResponseFuncBind(var socket_response: SocketResponses, var func: (Any?) -> Unit){}
object socky {
    public var socketClient: SocketClient?= null
    var chat_rooms : Array<ChatRoom>? = null
    var expcted_functions_promises: MutableList<ResponseFuncBind> = mutableListOf()


}

data class ChatRoom(
    var name:String, var description:String, var chat_type: String,
    var rooms: List<Int>, var users: List<String>
)

fun connect_to_ws(id_token: idToken){
    val id = id_token.id
    val token = id_token.token
    socky.socketClient = SocketClient(id, token)
    socky.socketClient?.execute()


}

fun get_current_user_id(context: Context): String{
    var shared_pref = context!!.getSharedPreferences("login_data",
        AppCompatActivity.MODE_PRIVATE)

    var id = shared_pref.getString("id", null )
    return (id!!)
}

enum class MyResult{
    Ok,
    Err,
}

fun get_user_account(context: Context, json_data: String): MyResult{
    try {
        val gson = Gson()
        var user_account: UserAccount = gson.fromJson(json_data, UserAccount::class.java)
        var db = MyDatabase(context)

        db.register_to_database(user_account.id, user_account.username, user_account.name, user_account.image,null)
        if (user_account.image != "default_profile.png") {
            val json = "{\"T\":\"gi\",\"data\":\"${user_account.image}\"}"
            socky?.socketClient?.webSocket?.send(json)
        }
        return  MyResult.Ok
    }catch(e:Exception){
        Log.e(TAG, e.toString())
        return MyResult.Err
    }

}

data class Potato(var T: String, var data: Array<UserAccount>)
fun get_users_accounts( json_data: String): Result<Array<UserAccount>>{
    try {
        val gson = Gson()
        var user_accounts: Potato =
            gson.fromJson(json_data,Potato::class.java)
        return Result.success(user_accounts.data)

    }
    catch (e:Exception){
        Log.e(TAG, e.toString())
        return Result.failure(e)
    }


}

fun download_image(context: Context, json_data: String): Result<String>{
    try{
        val gson = Gson()
        var image_data: ImageData = gson.fromJson(json_data, ImageData::class.java)
        var db = MyDatabase(context)
        val image_path = db.register_image(context!!, image_data)



        return image_path
    }
    catch(e:Exception){
        Log.e(TAG,e.toString())
        return Result.failure(e)
    }
}
data class ImageIdName (
        val image_id :String,
        val image_name: String
        )
fun parse_image(image : String): ImageIdName{
    var index = 0
    var image_id: String = ""
    for (char in image){
        if (char == '/'){
            index+=1
            break
        }
        image_id += char
    }
    var image_name = image.substring(index)
    return ImageIdName(image_id,image_name)

}
sealed class ControlFlow<T,D>{
    class Ok<T, D>(val value: T) : ControlFlow<T, D>(
    )
    class RequestServer<T, D>(val data: D) : ControlFlow<T, D>()
    class DefaultBehavior<T,D>() : ControlFlow<T, D>()
}
fun get_profile_pic_path(context: Context, from_id : String): ControlFlow<String, String?>{
    var db = MyDatabase(context)
    var read = db.readableDatabase
    var SQL = """
         SELECT image_id FROM my_table WHERE external_id = ${from_id};
    """.trimIndent()

   var cursor =  read.rawQuery(SQL, null)
    cursor.moveToFirst()
    val index = cursor.getColumnIndex("image_id")
    val image_id = cursor.getString(index)
    if (image_id== "0"){
      //  var a = R.drawable.sil7flop

        return  ControlFlow.DefaultBehavior()
    }
    var SQL2 = """
         SELECT image_name FROM ${db.IMG_TABLE_NAME} WHERE external_id = ${image_id};
    """.trimIndent()

    var cursor2 =  read.rawQuery(SQL2, null)
    cursor2.moveToFirst()
    val index2 = cursor.getColumnIndex("local_image_path")
    val local_path  = cursor.getString(index2)
    if (local_path!= null){
        return ControlFlow.Ok(local_path)
    }
    val index3 = cursor.getColumnIndex("image_name")
    val image_name = cursor.getString(index3)
    return ControlFlow.RequestServer("/$image_id/$image_name")

    //return "null"



}

fun set_socket_event(context: Context){
    socky.socketClient?.setSocketListener(
        object : SocketEventListener {
            override fun onEventTriggered(data: String) {
                handly_socket(context, data)
            }
        })
}

abstract class AbstractViewClass {
    abstract var view: Fragment

    // You can also have abstract methods or other members in the abstract class
}