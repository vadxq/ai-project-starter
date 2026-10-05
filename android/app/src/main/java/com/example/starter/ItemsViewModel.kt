package com.example.starter

import android.app.Application
import android.app.LocaleManager
import android.os.LocaleList
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.example.starter.auth.AuthController
import com.example.starter.auth.AuthenticationRequired
import com.example.starter.generated.model.Item
import com.example.starter.generated.model.UpdateItem
import com.example.starter.network.ApiFailure
import com.example.starter.network.ItemService
import java.io.IOException
import java.util.Locale
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class ScreenState(val signedIn: Boolean, val loading: Boolean, val saving: Boolean,
    val items: List<Item>, val total: Long, val offset: Long, val error: String?, val theme: String)

class ItemsViewModel(application: Application) : AndroidViewModel(application) {
    private val auth = AuthController(application)
    private val preferences = application.getSharedPreferences("preferences", Application.MODE_PRIVATE)
    private val mutable = MutableStateFlow(ScreenState(auth.signedIn, false, false, emptyList(), 0, 0, null,
        preferences.getString("theme", "system") ?: "system"))
    val state: StateFlow<ScreenState> = mutable.asStateFlow()
    private val service = ItemService(auth) { if (Locale.getDefault().language == "zh") "zh-CN" else "en" }
    private var generation: Long = 0
    private var loadSequence: Long = 0

    init { if (auth.signedIn) refresh() }

    private fun report(error: Exception): Unit {
        if (error is CancellationException) throw error
        val code: String = when (error) {
            is ApiFailure -> error.code
            is AuthenticationRequired -> "unauthorized"
            is IOException -> "network_error"
            else -> "unknown_error"
        }
        mutable.update { it.copy(error = code, loading = false, saving = false, signedIn = auth.signedIn,
            items = if (auth.signedIn) it.items else emptyList(), total = if (auth.signedIn) it.total else 0) }
    }

    fun signIn(username: String, password: String): Unit {
        if (mutable.value.loading) return
        val started: Long = generation
        mutable.update { it.copy(loading = true, error = null) }
        viewModelScope.launch {
            try {
                auth.signIn(username, password)
                if (generation != started) return@launch
                mutable.update { it.copy(signedIn = true) }
                load()
            } catch (error: Exception) { if (generation == started) report(error) }
        }
    }

    fun signOut(): Unit {
        generation += 1
        loadSequence += 1
        mutable.update { it.copy(signedIn = false, items = emptyList(), total = 0, offset = 0, error = null, loading = false, saving = false) }
        viewModelScope.launch {
            try { auth.signOut() } catch (error: Exception) { report(error) }
        }
    }

    private suspend fun load(): Unit {
        val started: Long = generation
        loadSequence += 1
        val sequence: Long = loadSequence
        mutable.update { it.copy(loading = true, error = null) }
        service.me()
        val page = service.list(mutable.value.offset)
        if (generation != started || loadSequence != sequence) return
        mutable.update { it.copy(items = page.items, total = page.total, loading = false) }
    }

    fun refresh(): Unit {
        val started: Long = generation
        viewModelScope.launch { try { load() } catch (error: Exception) { if (generation == started) report(error) } }
    }

    private fun write(action: suspend () -> Unit, didSave: () -> Unit): Unit {
        if (mutable.value.saving) return
        val started: Long = generation
        mutable.update { it.copy(saving = true, error = null) }
        viewModelScope.launch {
            try {
                action()
                if (generation != started) return@launch
                didSave()
                load()
            } catch (error: Exception) { if (generation == started) report(error) }
            finally { if (generation == started) mutable.update { it.copy(saving = false) } }
        }
    }

    fun create(title: String, done: () -> Unit): Unit {
        if (!validTitle(title)) { mutable.update { it.copy(error = "invalidTitle") }; return }
        write(action = { service.create(title) }, didSave = { mutable.update { it.copy(offset = 0) }; done() })
    }

    fun update(item: Item, patch: UpdateItem, done: () -> Unit): Unit {
        if (patch.title != null && !validTitle(patch.title)) { mutable.update { it.copy(error = "invalidTitle") }; return }
        write(action = { service.update(item, patch) }, didSave = done)
    }

    fun delete(item: Item): Unit {
        write(action = { service.delete(item) }, didSave = {
            if (state.value.items.size == 1) mutable.update { it.copy(offset = maxOf(0, it.offset - 20)) }
        })
    }
    fun page(forward: Boolean): Unit { mutable.update { it.copy(offset = maxOf(0, it.offset + if (forward) 20 else -20)) }; refresh() }
    fun theme(value: String): Unit { preferences.edit().putString("theme", value).apply(); mutable.update { it.copy(theme = value) } }
    fun language(value: String): Unit { getApplication<Application>().getSystemService(LocaleManager::class.java).applicationLocales = LocaleList.forLanguageTags(value) }

}

fun validTitle(title: String): Boolean = title.trim(' ', '\t', '\r', '\n').toByteArray(Charsets.UTF_8).size in 1..240
