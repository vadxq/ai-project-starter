package com.example.starter.network

import com.example.starter.BuildConfig
import com.example.starter.auth.AuthController
import com.example.starter.generated.api.RoutesApi
import com.example.starter.generated.model.CreateItem
import com.example.starter.generated.model.Identity
import com.example.starter.generated.model.Item
import com.example.starter.generated.model.ItemPage
import com.example.starter.generated.model.Problem
import com.example.starter.generated.model.UpdateItem
import kotlinx.serialization.json.Json
import okhttp3.OkHttpClient
import okhttp3.MediaType.Companion.toMediaType
import retrofit2.Response
import retrofit2.Retrofit
import retrofit2.converter.kotlinx.serialization.asConverterFactory

class ApiFailure(val code: String, val status: Int, val requestId: String?) : Exception(code)

private val problemJson: Json = Json { ignoreUnknownKeys = true }

fun <T> apiResult(response: Response<T>): T {
    if (!response.isSuccessful) {
        val body: String = response.errorBody()?.string() ?: throw ApiFailure("unknown_error", response.code(), null)
        val problem: Problem = problemJson.decodeFromString(body)
        throw ApiFailure(problem.code, response.code(), problem.requestId)
    }
    return response.body() ?: throw ApiFailure("unknown_error", response.code(), response.headers()["x-request-id"])
}


class ItemService(private val auth: AuthController, private val locale: () -> String) {
    private val json = Json { ignoreUnknownKeys = true; explicitNulls = false }
    private val transport: OkHttpClient = OkHttpClient.Builder().retryOnConnectionFailure(false).build()

    private suspend fun client(): RoutesApi {
        val token: String = auth.accessToken()
        // 每次请求捕获当时的 token；refresh 交给 AuthController，网络层不阻塞协程或自动重试写入。
        val http: OkHttpClient = transport.newBuilder().addInterceptor { chain ->
            chain.proceed(chain.request().newBuilder().header("Authorization", "Bearer $token")
                .header("Accept-Language", locale()).build()).also { if (it.code == 401) auth.reject(token) }
        }.build()
        return Retrofit.Builder().baseUrl(BuildConfig.API_BASE_URL).client(http)
            .addConverterFactory(json.asConverterFactory("application/json".toMediaType())).build().create(RoutesApi::class.java)
    }

    suspend fun me(): Identity = apiResult(client().getMe())
    suspend fun list(offset: Long): ItemPage = apiResult(client().listItems(20, offset))
    suspend fun create(title: String): Unit { apiResult(client().createItem(CreateItem(false, title))) }
    suspend fun update(item: Item, patch: UpdateItem): Unit { apiResult(client().updateItem(item.id, patch)) }
    suspend fun delete(item: Item): Unit {
        val response: Response<Unit> = client().deleteItem(item.id, item.version)
        if (!response.isSuccessful) apiResult(response)
    }
}
