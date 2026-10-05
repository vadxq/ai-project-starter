package com.example.starter.auth

import android.content.Context
import com.example.starter.BuildConfig
import com.example.starter.generated.api.AuthApi
import com.example.starter.generated.model.LoginRequest
import com.example.starter.generated.model.RefreshRequest
import com.example.starter.generated.model.TokenResponse
import com.example.starter.network.apiResult
import com.example.starter.network.ApiFailure
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import retrofit2.Retrofit
import retrofit2.converter.kotlinx.serialization.asConverterFactory

class AuthenticationRequired : Exception("Authentication required")

/** Rust 签发 JWT；刷新串行执行，Keystore 只保存 refresh token。 */
class AuthController(context: Context) {
    private val store = SecureAuthStore(context)
    @Volatile private var refresh: String? = store.read()
    @Volatile private var access: String? = null
    private var expiresAt: Long = 0
    private val mutex = Mutex()
    private val json = Json { ignoreUnknownKeys = true; explicitNulls = false }
    private val http: OkHttpClient = OkHttpClient.Builder().retryOnConnectionFailure(false)
        .followRedirects(false).callTimeout(10, TimeUnit.SECONDS).build()
    private val api: AuthApi = Retrofit.Builder().baseUrl(BuildConfig.API_BASE_URL).client(http)
        .addConverterFactory(json.asConverterFactory("application/json".toMediaType())).build().create(AuthApi::class.java)

    val signedIn: Boolean get() = refresh != null

    @Synchronized private fun accept(session: TokenResponse): Unit {
        store.save(session.refreshToken)
        refresh = session.refreshToken
        access = session.accessToken
        expiresAt = System.currentTimeMillis() + session.expiresIn * 1000
    }

    suspend fun signIn(username: String, password: String): Unit = mutex.withLock {
        accept(apiResult(api.login(LoginRequest(username = username, password = password))))
    }

    suspend fun accessToken(): String = mutex.withLock {
        val token: String = refresh ?: throw AuthenticationRequired()
        if (access == null || System.currentTimeMillis() >= expiresAt - 30_000) {
            try { accept(apiResult(api.refreshSession(RefreshRequest(token)))) }
            catch (error: ApiFailure) { if (error.status == 401) clear(); throw error }
        }
        access ?: throw AuthenticationRequired()
    }

    suspend fun signOut(): Unit = mutex.withLock {
        val token: String? = refresh
        clear()
        if (token != null) {
            val response = api.logout(RefreshRequest(token))
            if (!response.isSuccessful) apiResult(response)
        }
    }

    @Synchronized fun clear(): Unit {
        store.clear()
        refresh = null
        access = null
        expiresAt = 0
    }

    @Synchronized fun reject(token: String): Unit { if (access == token) clear() }
}
