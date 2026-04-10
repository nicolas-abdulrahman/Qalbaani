package com.example.myapplication4.network

import android.content.Context
import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper
import android.util.Log
import androidx.core.net.toUri
import java.io.File
import java.io.FileOutputStream


val DATABASE_NAME = "CoolDB"
val DATABASE_VERSION = 1
class MyDatabase(context: Context) : SQLiteOpenHelper(context, DATABASE_NAME, null, DATABASE_VERSION) {
    val TABLE_NAME = "my_table"
    val IMG_TABLE_NAME = "images"
    val CHAT_ROOM_TABLE_NAME = "chat_room"
    val CHAT_MESSAGES_TABLE_NAME = "messages"
    val ROOMS_TABLE_NAME = "rooms"
    private var TAG = "DB"


    private val CREATE_TABLE_QUERY = """
        CREATE TABLE IF NOT EXISTS $TABLE_NAME (
            'external_id' INT PRIMARY KEY,
            'username' TEXT,
            'name' TEXT,
            'image_id' TEXT DEFAULT NULL
           
        );
    """.trimIndent()
    private val CREATE_CHAT_ROOM_TABLE_QUERY = """
        CREATE TABLE IF NOT EXISTS $CHAT_ROOM_TABLE_NAME (
            'external_id' INT PRIMARY KEY,
            'room_type' TEXT CHECK (room_type IN ("Private", "GroupChat")),
            'other_user_id' BIGINT DEFAULT NULL,
            'image_id' TEXT DEFAULT NULL,
            'last_room_id' INT DEFAULT 0,
            'last_message_id' INT DEFAULT NULL
        );
    """.trimIndent()
    private val CREATE_MESSAGES_TABLE_QUERY = """
        CREATE TABLE IF NOT EXISTS $CHAT_MESSAGES_TABLE_NAME(
        'chat_room_id' INT NOT NULL,
        'room_id' INT NOT NULL,
        'message_id' INT NOT NULL,
        'owner_id' INT  NOT NULL,
        'data' TEXT NOT NULL,
        'timestap' TEXT DEFAULT NULL
        ) WITHOUT ROWID;
    """.trimMargin()

    private val CREATE_ROOM_TABLE_QUERY = """
          CREATE TABLE IF NOT EXISTS $CHAT_MESSAGES_TABLE_NAME(
        'chat_room_id' INT NOT NULL,
        'chat_id' INT NOT NULL,
        'messages_count' INT DEFAULT NULL,
        'description' TEXT DEFAULT NULL
        
        )WITHOUT ROWID;
    """.trimIndent()

    private val CREATE_IMAGE_TABLE_QUERY = """
        CREATE TABLE IF NOT EXISTS $IMG_TABLE_NAME (
            'external_id' TEXT PRIMARY KEY,
            'name' TEXT DEFAULT NULL,
            'local_image_path' TEXT DEFAULT NULL
           
        );
    """.trimIndent()

    override fun onCreate(db: SQLiteDatabase?) {
        db?.execSQL(CREATE_TABLE_QUERY)
        db?.execSQL(CREATE_IMAGE_TABLE_QUERY)
        db?.execSQL(CREATE_CHAT_ROOM_TABLE_QUERY)
        db?.execSQL(CREATE_MESSAGES_TABLE_QUERY)
        db?.execSQL(CREATE_ROOM_TABLE_QUERY)
    }

    override fun onUpgrade(db: SQLiteDatabase, oldVersion: Int, newVersion: Int) {
        // Implement upgrade logic here if needed
        db.execSQL("DROP TABLE IF EXISTS $TABLE_NAME");
        onCreate(db)
    }

    fun register_to_database(id: UInt, username: String, name: String, image_path: String?, local_image_path: String?){
        try{
            var write = this.writableDatabase
            var image_id = ""
            var image_name = image_path
            if (image_path == "default_profile.png") {
                image_id = "0"
            }
            else{
                var id_name = parse_image(image_path!!)
                 image_name = id_name.image_name
                 image_id = id_name.image_id
            }
            val SQL = """
                    INSERT INTO ${TABLE_NAME}(external_id, username, name, image_id)
                     values($id, "$username", "$name", "$image_id") 
                """.trimIndent()

            write.execSQL(SQL)

            if(image_id=="0"){return}
            val SQL2 = """
                    INSERT INTO ${IMG_TABLE_NAME}(external_id, name, local_image_path)
                     values("$image_id", "$image_name", "$local_image_path") 
                """.trimIndent()
            write.execSQL(SQL2)
        }
        catch (e:Exception){
            Log.e(TAG, e.toString())
        }

    }

    fun register_image(context: Context, image_data: ImageData): Result<String>{
        try {
            Log.d(TAG, "gonna write image to file")
            if (image_data.name =="default_profile.png"){
                return Result.success("")
            }
            var id_name = parse_image(image_data.name)
            var img = parse_image(image_data.name)
            var b = File(context.filesDir, image_data.name)
            b.writeBytes(image_data.data)
            Log.d(TAG, "wrote image to file ${b.toURI()} ")
            var write = this.writableDatabase
            val uri =  b.toPath().toString()

            val SQL = """
             INSERT INTO ${IMG_TABLE_NAME}(external_id, name, local_image_path)
               values("${id_name.image_id}", "${id_name.image_name}"   "$uri") 
        """.trimIndent()

            write.execSQL(SQL)
            return  Result.success(uri)
        }catch(e: Exception){
            Log.e(TAG, e.toString())
            return Result.failure(e)
        }
    }

   /// fun get_all_users(): Array<MyTable>{
     //   val SQL = "SELECT * FROM $TABLE_NAME"
      //  var query =
   // }

     fun get_user(user_id: String): MyTable?{
        try {
            Log.d(TAG, "getting user $user_id")


            var read = this.readableDatabase
            val SQL = """
            SELECT * FROM my_table WHERE external_id = $user_id;
        """.trimIndent()
            // read.execSQL(SQL)
           var cursor = read.rawQuery(SQL, null)
         //   var cursor = read.query(
          //      TABLE_NAME,
           //     arrayOf("username", "name"),
            //    "id",
          //      arrayOf(user_id),
            //    null,
           ////     null,
            //    null,
          //  )
            if (cursor == null){
                Log.e(TAG, "null cursor")
                return null
            }
            cursor.moveToFirst()
            val index = cursor.getColumnIndex("username")
            val username = cursor.getString(index)
            val index2 = cursor.getColumnIndex("name")
            val name = cursor.getString(index2)
            var table = MyTable(username, name, null, null)
            return table


        }
        catch (e: Exception){
            Log.e(TAG, e.toString())
            return null
        }

    }
}

data class MyTable(
    val username: String, val name : String, val image_path: String?, val local_image_path: String?
)