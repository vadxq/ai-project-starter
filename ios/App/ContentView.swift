import SwiftUI

struct ContentView: View {
    @Bindable var auth: AuthSession
    @Bindable var preferences: Preferences
    @Bindable var model: ItemsModel
    @State private var title: String = ""
    @State private var username: String = ""
    @State private var password: String = ""
    @State private var editing: Item?
    @State private var deleting: Item?

    var body: some View {
        NavigationStack {
            Group {
                if auth.signedIn { itemList }
                else { welcome }
            }
            .navigationTitle(preferences.text("app"))
            .toolbar { ToolbarItem(placement: .topBarTrailing) { preferencesMenu } }
            .task(id: auth.signedIn) {
                if auth.signedIn { await model.load() }
                else { model.clear() }
            }
            .sheet(isPresented: Binding(get: { editing != nil }, set: { if !$0 { editing = nil } })) {
                if let editing { ItemEditor(item: editing, model: model, preferences: preferences) }
            }
            .confirmationDialog(preferences.text("confirmDelete"), isPresented: Binding(get: { deleting != nil }, set: { if !$0 { deleting = nil } }), titleVisibility: .visible) {
                Button(preferences.text("delete"), role: .destructive) {
                    guard let item = deleting else { return }
                    Task { await model.delete(item: item); deleting = nil }
                }
                Button(preferences.text("cancel"), role: .cancel) { deleting = nil }
            }
        }
    }

    private var welcome: some View {
        ScrollView { VStack(alignment: .leading, spacing: 20) {
            Text(preferences.text("welcome")).font(.largeTitle.weight(.medium))
            TextField(preferences.text("username"), text: $username)
                .textContentType(.username).textInputAutocapitalization(.never).autocorrectionDisabled()
                .textFieldStyle(.roundedBorder).accessibilityIdentifier("username")
            SecureField(preferences.text("password"), text: $password)
                .textContentType(.password).textFieldStyle(.roundedBorder).accessibilityIdentifier("password")
            if let error = auth.errorCode { Text(preferences.text(error)).foregroundStyle(.red).accessibilityIdentifier("auth-error") }
            Button(preferences.text("signIn")) {
                let submitted: String = password
                password = ""
                Task { await auth.signIn(username: username, password: submitted) }
            }
                .buttonStyle(.borderedProminent).disabled(auth.busy || username.isEmpty || password.isEmpty).accessibilityIdentifier("sign-in")
            if auth.busy { ProgressView() }
            Spacer()
        }.padding(24).padding(.top, 36) }
    }

    private var preferencesMenu: some View {
        Menu {
            Section(preferences.text("language")) {
                Button("English") { preferences.language = "en" }.accessibilityIdentifier("language-en")
                Button("简体中文") { preferences.language = "zh-Hans" }.accessibilityIdentifier("language-zh")
            }
            Section(preferences.text("theme")) {
                ForEach(["system", "light", "dark"], id: \.self) { theme in
                    Button(preferences.text(theme)) { preferences.theme = theme }
                }
            }
            if auth.signedIn {
                Button(preferences.text("signOut"), role: .destructive) { Task { await auth.signOut() } }
                    .disabled(auth.busy).accessibilityIdentifier("sign-out")
            }
        } label: { Image(systemName: "ellipsis.circle").accessibilityLabel(preferences.text("preferences")) }
            .accessibilityIdentifier("preferences")
    }

    private var itemList: some View {
        List {
            Section {
                TextField(preferences.text("placeholder"), text: $title)
                    .accessibilityLabel(preferences.text("title")).accessibilityIdentifier("new-title")
                Button(preferences.text("add")) {
                    Task { if await model.create(title: title) { title = "" } }
                }.disabled(model.saving).accessibilityIdentifier("add-item")
            }
            if let error = model.errorCode {
                Section {
                    Text(preferences.text(error)).foregroundStyle(.red).accessibilityIdentifier("api-error")
                    Button(preferences.text("retry")) { Task { await model.load() } }
                }
            }
            Section {
                if model.loading { ProgressView(preferences.text("loading")) }
                if model.items.isEmpty && !model.loading && model.errorCode == nil {
                    ContentUnavailableView(preferences.text("empty"), systemImage: "checklist", description: Text(preferences.text("emptyBody")))
                }
                ForEach(model.items, id: \.id) { item in itemRow(item: item) }
            } header: { Text(String.localizedStringWithFormat(preferences.text("item_count"), model.total)) }
            if model.total > 20 || model.offset > 0 {
                HStack {
                    Button(preferences.text("previous")) { Task { await model.page(forward: false) } }.disabled(model.offset == 0 || model.loading)
                    Spacer()
                    Button(preferences.text("next")) { Task { await model.page(forward: true) } }.disabled(model.offset + 20 >= model.total || model.loading)
                }
            }
        }.listStyle(.insetGrouped).refreshable { await model.load() }
    }

    private func itemRow(item: Item) -> some View {
        HStack(spacing: 12) {
            Button { Task { _ = await model.update(item: item, title: nil, completed: !item.completed) } } label: {
                Image(systemName: item.completed ? "checkmark.circle.fill" : "circle").font(.title2)
            }.buttonStyle(.borderless).disabled(model.saving)
                .accessibilityLabel(preferences.text(item.completed ? "reopen" : "complete"))
            VStack(alignment: .leading, spacing: 4) {
                Text(item.title).strikethrough(item.completed).foregroundStyle(item.completed ? .secondary : .primary)
                Text(item.updatedAt, format: .dateTime.year().month().day()).font(.caption).foregroundStyle(.secondary)
            }
            Spacer()
            Menu {
                Button(preferences.text("edit")) { editing = item }
                Button(preferences.text("delete"), role: .destructive) { deleting = item }
            } label: { Image(systemName: "ellipsis").frame(minWidth: 44, minHeight: 44) }
                .disabled(model.saving).accessibilityLabel(preferences.text("actions"))
        }.accessibilityIdentifier("item-\(item.id)")
    }
}
