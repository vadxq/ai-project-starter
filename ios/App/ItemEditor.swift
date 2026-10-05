import SwiftUI

/// 保留编辑输入直到服务端保存成功，验证或版本冲突时允许用户继续修改。
struct ItemEditor: View {
    let item: Item
    @Bindable var model: ItemsModel
    @Bindable var preferences: Preferences
    @Environment(\.dismiss) private var dismiss
    @State private var title: String

    init(item: Item, model: ItemsModel, preferences: Preferences) {
        self.item = item
        self.model = model
        self.preferences = preferences
        _title = State(initialValue: item.title)
    }

    var body: some View {
        NavigationStack {
            Form {
                TextField(preferences.text("title"), text: $title).accessibilityIdentifier("edit-title")
                if let error = model.errorCode { Text(preferences.text(error)).foregroundStyle(.red) }
            }
            .navigationTitle(preferences.text("edit"))
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button(preferences.text("cancel")) { dismiss() }.disabled(model.saving)
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button(preferences.text("save")) {
                        Task { if await model.update(item: item, title: title, completed: nil) { dismiss() } }
                    }.disabled(model.saving).accessibilityIdentifier("save-item")
                }
            }
        }.interactiveDismissDisabled(model.saving)
    }
}
