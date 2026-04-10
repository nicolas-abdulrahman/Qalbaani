import android.content.Context
import android.icu.util.TimeUnit
import android.net.Uri
import android.os.AsyncTask
import android.util.Log
import android.view.LayoutInflater
import android.widget.Toast
import com.example.myapplication4.databinding.ChatsBinding
import com.example.myapplication4.databinding.MenuProfilePlaceholderBinding
import com.example.myapplication4.network.BASE_URL
import com.example.myapplication4.network.ChatRoom
import com.example.myapplication4.network.MyDatabase
import com.example.myapplication4.network.MyResult
import com.example.myapplication4.network.SocketResponses
import com.example.myapplication4.network.UserAccount
import com.example.myapplication4.network.download_image
import com.example.myapplication4.network.get_current_user_id
import com.example.myapplication4.network.get_user_account
import com.example.myapplication4.network.get_users_accounts
import com.example.myapplication4.network.socky
import com.google.gson.Gson
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.Response
import okhttp3.WebSocket
import okhttp3.WebSocketListener
import java.io.BufferedReader
import java.io.InputStreamReader
import java.io.PrintWriter
import java.net.Socket
import java.time.Duration
import java.time.temporal.ChronoUnit
import java.time.temporal.TemporalUnit
import kotlin.concurrent.thread
import kotlin.math.log

class SocketClient(val id: UInt, val token: String) : AsyncTask<Void, Void, String>() {
    private var socketListener : SocketEventListener? = null
    private var TAG = "socky"
    var webSocket : WebSocket? = null

    fun setSocketListener(list: SocketEventListener  ){
        this.socketListener = list
    }

    fun getSocketListener(): SocketEventListener?{
        return socketListener
    }

    override fun doInBackground(vararg params: Void?): String {
        try {
            val a = ChronoUnit.HOURS
            val b = TimeUnit.SECOND
            val client = OkHttpClient.Builder()
                .connectTimeout(Duration.of(10, a))
                .readTimeout(Duration.of(10, a))
                .writeTimeout(Duration.of(10, a))
                .build()

            val request = Request.Builder()
                .url("ws://192.168.10.13/ws?id=$id&token=$token")
                .build()

            webSocket = client.newWebSocket(request, object : WebSocketListener() {
                // WebSocket event handling methods
                // ...

                override fun onOpen(webSocket: WebSocket, response: Response) {
                    // Connection opened
                    // You can send a message here if required
                   // webSocket.send("Hello Server!")
                }

                override fun onMessage(webSocket: WebSocket, text: String) {
                    // Handle received message
                   // Log.d("sock","Received message: $text")
                    socketListener?.onEventTriggered(text)
                }

            })
            if (socketListener != null){
                socketListener?.onEventTriggered("all done here")
            }
           // webSocket?.send("hello from client dwb")

        } catch (e: Exception) {
            Log.e(TAG, e.toString())

        }
        return "bob"
    }
}

interface SocketEventListener{
    fun onEventTriggered(data: String)
}

fun handly_socket(context: Context, data: String){
    var gson = Gson()
    var r = download_image(context!!, data)
    if (r.isSuccess){
        evaluate_promises(SocketResponses.get_image,r.getOrThrow())
        return
    }
    else if (get_user_account(context!!, data) == MyResult.Ok){
        evaluate_promises(SocketResponses.get_user, null)
        return
    }
    var type = parse_type(data)
    if (type == null){
        return
    }
  //  var result = message_get_chat_rooms(data)
    if (type == SocketResponses.get_chat_rooms ){
        var result = message_get_chat_rooms(data)
        evaluate_promises(SocketResponses.get_chat_rooms, result.getOrThrow())
        return
    }
    if (type == SocketResponses.get_all_users ){
        var result = get_users_accounts(data)
        evaluate_promises(SocketResponses.get_all_users, result.getOrThrow())
        return
    }

}

fun parse_type(data: String): SocketResponses? {
    val start_index = 6
    var end_index = 0
    var slice =""

    for (i in start_index..start_index + 2) {
        if (data[i] == '"') {
            break
        }
        slice += data[i]
    }
    if (slice == ""){
        return null
    }


    if (slice == "gcr"){
        return SocketResponses.get_chat_rooms
    }
    if (slice == "gau"){
        return SocketResponses.get_all_users
    }
    return null
   // var slice = data[start_index,end_index]



}
//fun message_get_all_users(json_string: String): Result<Array<UserAccount>>{

//}
data class Banana( var T: String, var data: Array<ChatRoom>)
fun message_get_chat_rooms(json_string: String): Result<Array<ChatRoom>>{
    try {
        val gson = Gson()

        var chat_rooms: Banana =
            gson.fromJson(json_string, Banana::class.java)
        return (Result.success(chat_rooms.data))
    }catch(e:Exception){
        Log.e("sock", "At message_get_chat_rooms ${e.toString()}" )
        return (Result.failure(e))
    }

}
fun evaluate_promises(socket_response: SocketResponses, data: Any?){
    if (socky.expcted_functions_promises.size >0 ){
        var index = 0
        for (response in socky.expcted_functions_promises){
            if (response.socket_response == socket_response){
                response.func(data)
                break
                index+=0
            }
        }
        socky.expcted_functions_promises.removeAt(index)
    }
}