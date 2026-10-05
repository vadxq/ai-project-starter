package com.example.starter.auth

import android.content.Context
import android.util.Base64
import com.google.crypto.tink.Aead
import com.google.crypto.tink.KeyTemplates
import com.google.crypto.tink.RegistryConfiguration
import com.google.crypto.tink.config.TinkConfig
import com.google.crypto.tink.integration.android.AndroidKeysetManager

/** Tink 管理加密格式与 Android Keystore 主密钥，应用只存加密后的 refresh token。 */
class SecureAuthStore(context: Context) {
    private val preferences = context.getSharedPreferences("jwt-auth", Context.MODE_PRIVATE)
    private val associatedData: ByteArray = context.packageName.toByteArray(Charsets.UTF_8)
    private val aead: Aead

    init {
        TinkConfig.register()
        aead = AndroidKeysetManager.Builder()
            .withSharedPref(context, "auth-keyset", "auth-keys")
            .withKeyTemplate(KeyTemplates.get("AES256_GCM"))
            .withMasterKeyUri("android-keystore://starter-auth-master")
            .build().keysetHandle.getPrimitive(RegistryConfiguration.get(), Aead::class.java)
    }

    fun read(): String? {
        val encoded: String = preferences.getString("state", null) ?: return null
        val ciphertext: ByteArray = Base64.decode(encoded, Base64.NO_WRAP)
        val plaintext: ByteArray = aead.decrypt(ciphertext, associatedData)
        return plaintext.toString(Charsets.UTF_8)
    }

    fun save(state: String): Unit {
        val encrypted: ByteArray = aead.encrypt(state.toByteArray(Charsets.UTF_8), associatedData)
        check(preferences.edit().putString("state", Base64.encodeToString(encrypted, Base64.NO_WRAP)).commit()) { "Auth state storage failed" }
    }

    fun clear(): Unit {
        check(preferences.edit().remove("state").commit()) { "Auth state removal failed" }
    }
}
