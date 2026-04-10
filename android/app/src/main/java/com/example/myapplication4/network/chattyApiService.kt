package com.example.myapplication4.network


import com.squareup.moshi.Moshi
import com.squareup.moshi.kotlin.reflect.KotlinJsonAdapterFactory
import retrofit2.Retrofit
import retrofit2.converter.moshi.MoshiConverterFactory
import retrofit2.converter.scalars.ScalarsConverterFactory
import retrofit2.http.Body
import retrofit2.http.GET
import retrofit2.http.POST

public const val BASE_URL =
    "http://192.168.10.13"


private val moshi = Moshi.Builder()
    .add(KotlinJsonAdapterFactory())
    .build()
private val retrofit = Retrofit.Builder()
    .addConverterFactory(MoshiConverterFactory.create(moshi))
    .baseUrl(BASE_URL)
    .build()

object chattyApi{
    val retrofitService : chattyApiService by lazy {
        retrofit.create(chattyApiService::class.java)
    }
}

data class userData(
    val name: String, val name_color: String, val image: String
)

data class ImageData(
    val name: String,
    val data : ByteArray,
)
data class RegisterBody(
    val username: String, val password: String, val name: String?, val image: ImageData?,
)

data class idToken(
    val id: UInt, val token: String,
)
data class ImageResponse(
    val id: String, val name: String
)
data class RegisterResponse(
    val id_token: idToken, val username: String, val name: String, val img_path: String
)

data class LoginBody(
    val username: String,
    val password: String,
)
data class UserAccount(
    val id : UInt,
    val username: String,
    val name: String,
    val cs: String,
    val status : String,
    val image : String
)
interface chattyApiService {
    @GET("user/sunshine")
    suspend fun getUser() : userData

    @GET()
    suspend fun getRoot() : String

    @POST("sign_in")
    suspend fun signIn(@Body body: RegisterBody ) : RegisterResponse

    @GET("log_in")
    suspend fun logIn(@Body body: LoginBody ) : idToken

}