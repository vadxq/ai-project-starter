package com.example.starter

import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.Checkbox
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.example.starter.generated.model.Item
import com.example.starter.generated.model.UpdateItem
import java.time.Instant
import java.time.ZoneId
import java.time.format.DateTimeFormatter
import java.time.format.FormatStyle

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun StarterScreen(model: ItemsViewModel, signOut: () -> Unit): Unit {
    val state: ScreenState by model.state.collectAsStateWithLifecycle()
    val dark: Boolean = state.theme == "dark" || (state.theme == "system" && isSystemInDarkTheme())
    MaterialTheme(colorScheme = if (dark) darkColorScheme(primary = Color(0xFFA3D4AE)) else lightColorScheme(primary = Color(0xFF285F43))) {
        Scaffold(topBar = { TopAppBar(title = { Text(stringResource(R.string.app)) }, actions = { PreferencesMenu(model, state, signOut) }) }) { padding ->
            Column(Modifier.fillMaxSize().padding(padding).padding(horizontal = 20.dp), verticalArrangement = Arrangement.spacedBy(16.dp)) {
                state.error?.let { ErrorMessage(it, model::refresh) }
                if (state.signedIn) ItemList(model, state)
                else LoginForm(model, state)
            }
        }
    }
}

@Composable
private fun LoginForm(model: ItemsViewModel, state: ScreenState): Unit {
    var username: String by rememberSaveable { mutableStateOf("") }
    // 密码不进入 savedInstanceState，提交后清除输入。
    var password: String by remember { mutableStateOf("") }
    Column(Modifier.fillMaxWidth().verticalScroll(rememberScrollState()),
        verticalArrangement = Arrangement.spacedBy(16.dp)) {
        Text(stringResource(R.string.welcome), style = MaterialTheme.typography.headlineMedium, modifier = Modifier.padding(top = 32.dp))
        OutlinedTextField(value = username, onValueChange = { username = it },
            label = { Text(stringResource(R.string.username)) }, singleLine = true, enabled = !state.loading,
            modifier = Modifier.fillMaxWidth().testTag("username"))
        OutlinedTextField(value = password, onValueChange = { password = it },
            label = { Text(stringResource(R.string.password)) }, singleLine = true, enabled = !state.loading,
            visualTransformation = PasswordVisualTransformation(), keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Password),
            modifier = Modifier.fillMaxWidth().testTag("password"))
        Button(onClick = { model.signIn(username, password); password = "" },
            enabled = !state.loading && username.isNotEmpty() && password.isNotEmpty(), modifier = Modifier.testTag("sign-in")) {
            Text(stringResource(if (state.loading) R.string.loading else R.string.sign_in))
        }
    }
}

@Composable
private fun PreferencesMenu(model: ItemsViewModel, state: ScreenState, signOut: () -> Unit): Unit {
    var open: Boolean by rememberSaveable { mutableStateOf(false) }
    TextButton(onClick = { open = true }, modifier = Modifier.testTag("preferences")) { Text(stringResource(R.string.preferences)) }
    DropdownMenu(expanded = open, onDismissRequest = { open = false }) {
        DropdownMenuItem(text = { Text("English") }, onClick = { model.language("en"); open = false })
        DropdownMenuItem(text = { Text("简体中文") }, onClick = { model.language("zh-CN"); open = false })
        for ((theme, label) in listOf("system" to R.string.system, "light" to R.string.light, "dark" to R.string.dark)) {
            DropdownMenuItem(text = { Text(stringResource(label)) }, onClick = { model.theme(theme); open = false })
        }
        if (state.signedIn) DropdownMenuItem(text = { Text(stringResource(R.string.sign_out)) }, onClick = { open = false; signOut() })
    }
}

@Composable
private fun ItemList(model: ItemsViewModel, state: ScreenState): Unit {
    var title: String by rememberSaveable { mutableStateOf("") }
    val listState = rememberLazyListState()
    // 新增或换页后展示当前页顶部，避免按旧 Item key 保留滚动位置而隐藏新记录。
    LaunchedEffect(state.offset, state.items.firstOrNull()?.id) { listState.scrollToItem(0) }
    OutlinedTextField(value = title, onValueChange = { title = it }, label = { Text(stringResource(R.string.title)) },
        modifier = Modifier.fillMaxWidth().testTag("new-title"), enabled = !state.saving, singleLine = true)
    Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.End) {
        Button(onClick = { model.create(title) { title = "" } }, enabled = !state.saving, modifier = Modifier.testTag("add-item")) { Text(stringResource(R.string.add)) }
    }
    Text(pluralStringResource(R.plurals.item_count, state.total.toInt(), state.total), style = MaterialTheme.typography.labelMedium)
    if (state.loading) CircularProgressIndicator()
    if (!state.loading && state.items.isEmpty() && state.error == null) {
        Text(stringResource(R.string.empty), style = MaterialTheme.typography.titleMedium)
        Text(stringResource(R.string.empty_body), color = MaterialTheme.colorScheme.onSurfaceVariant)
    }
    LazyColumn(state = listState, modifier = Modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(8.dp)) {
        items(state.items, key = { it.id }) { item -> ItemRow(item, model, state.saving); HorizontalDivider() }
        if (state.total > 20 || state.offset > 0) item {
            Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                TextButton(onClick = { model.page(false) }, enabled = state.offset > 0 && !state.loading) { Text(stringResource(R.string.previous)) }
                TextButton(onClick = { model.page(true) }, enabled = state.offset + 20 < state.total && !state.loading) { Text(stringResource(R.string.next)) }
            }
        }
    }
}

@Composable
private fun ItemRow(item: Item, model: ItemsViewModel, saving: Boolean): Unit {
    var editing: Boolean by rememberSaveable(item.id) { mutableStateOf(false) }
    var deleting: Boolean by rememberSaveable(item.id) { mutableStateOf(false) }
    var title: String by rememberSaveable(item.id) { mutableStateOf(item.title) }
    val description: String = stringResource(if (item.completed) R.string.reopen else R.string.complete)
    Column(Modifier.fillMaxWidth().padding(vertical = 8.dp).testTag("item-row")) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Checkbox(checked = item.completed, onCheckedChange = { model.update(item, UpdateItem(version = item.version, completed = it)) {} },
                enabled = !saving, modifier = Modifier.semantics { contentDescription = description })
            Spacer(Modifier.width(8.dp))
            Column {
                Text(item.title, textDecoration = if (item.completed) TextDecoration.LineThrough else TextDecoration.None)
                Text(DateTimeFormatter.ofLocalizedDate(FormatStyle.MEDIUM).withZone(ZoneId.systemDefault()).format(Instant.parse(item.updatedAt)),
                    style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
            }
        }
        Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.End) {
            TextButton(onClick = { title = item.title; editing = true }, enabled = !saving) { Text(stringResource(R.string.edit)) }
            TextButton(onClick = { deleting = true }, enabled = !saving) { Text(stringResource(R.string.delete)) }
        }
    }
    if (editing) AlertDialog(onDismissRequest = { editing = false }, title = { Text(stringResource(R.string.edit)) },
        text = { OutlinedTextField(value = title, onValueChange = { title = it }, label = { Text(stringResource(R.string.title)) }, modifier = Modifier.testTag("edit-title")) },
        confirmButton = { TextButton(onClick = { model.update(item, UpdateItem(version = item.version, title = title)) { editing = false } }, enabled = !saving) { Text(stringResource(R.string.save)) } },
        dismissButton = { TextButton(onClick = { editing = false }) { Text(stringResource(R.string.cancel)) } })
    if (deleting) AlertDialog(onDismissRequest = { deleting = false }, title = { Text(stringResource(R.string.confirm_delete)) },
        confirmButton = { TextButton(onClick = { model.delete(item); deleting = false }, enabled = !saving) { Text(stringResource(R.string.delete)) } },
        dismissButton = { TextButton(onClick = { deleting = false }) { Text(stringResource(R.string.cancel)) } })
}

@Composable
private fun ErrorMessage(code: String, retry: () -> Unit): Unit {
    val resource: Int = when (code) {
        "invalidTitle" -> R.string.invalid_title
        "validation_error" -> R.string.validation_error
        "unauthorized" -> R.string.unauthorized
        "invalid_credentials" -> R.string.invalid_credentials
        "forbidden" -> R.string.forbidden
        "not_found" -> R.string.not_found
        "version_conflict" -> R.string.version_conflict
        "payload_too_large" -> R.string.payload_too_large
        "service_unavailable" -> R.string.service_unavailable
        "network_error" -> R.string.network_error
        "auth_error" -> R.string.auth_error
        else -> R.string.unknown_error
    }
    Row(verticalAlignment = Alignment.CenterVertically) {
        Text(stringResource(resource), color = MaterialTheme.colorScheme.error, modifier = Modifier.weight(1f))
        TextButton(onClick = retry) { Text(stringResource(R.string.retry)) }
    }
}
