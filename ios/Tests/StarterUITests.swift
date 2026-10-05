import XCTest

final class StarterUITests: XCTestCase {
    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    @MainActor
    func testLaunchAndPreferences() throws {
        let app = XCUIApplication()
        app.launch()
        XCTAssertTrue(app.buttons["preferences"].waitForExistence(timeout: 10))
        app.buttons["preferences"].tap()
        XCTAssertTrue(app.buttons["English"].waitForExistence(timeout: 5))
        app.buttons["English"].tap()
        XCTAssertTrue(app.buttons["sign-in"].exists || app.textFields["new-title"].exists)
        app.buttons["preferences"].tap()
        app.buttons["Dark"].tap()
        app.buttons["preferences"].tap()
        app.buttons["简体中文"].tap()
        XCTAssertTrue(app.navigationBars["事项"].waitForExistence(timeout: 5))
        let screenshot = XCTAttachment(screenshot: app.screenshot())
        screenshot.lifetime = .keepAlways
        add(screenshot)
        app.buttons["preferences"].tap()
        app.buttons["English"].tap()
        app.buttons["preferences"].tap()
        app.buttons["System"].tap()
    }

    @MainActor
    func testRealLoginAndCRUD() throws {
        let app = XCUIApplication()
        app.launch()
        app.buttons["preferences"].tap()
        app.buttons["English"].tap()
        // 失败重跑时仍使用正常注销流程，覆盖 Keychain 恢复后的会话。
        if app.textFields["new-title"].exists {
            app.buttons["preferences"].tap()
            app.buttons["Sign out"].tap()
        }
        XCTAssertTrue(app.buttons["sign-in"].waitForExistence(timeout: 10))
        let username = app.textFields["username"]
        XCTAssertTrue(username.waitForExistence(timeout: 10))
        username.tap()
        username.typeText("alice")
        let password = app.secureTextFields["password"]
        password.tap()
        password.typeText("wrong-password")
        app.buttons["sign-in"].tap()
        XCTAssertTrue(app.staticTexts["auth-error"].waitForExistence(timeout: 10))
        XCTAssertTrue(username.exists)
        password.tap()
        password.typeText("starter-password")
        app.buttons["sign-in"].tap()
        XCTAssertTrue(app.textFields["new-title"].waitForExistence(timeout: 20))
        app.terminate()
        app.launch()
        XCTAssertTrue(app.textFields["new-title"].waitForExistence(timeout: 20))
        let title = "iOS " + UUID().uuidString.prefix(8)
        app.textFields["new-title"].tap()
        app.textFields["new-title"].typeText(title)
        app.buttons["add-item"].tap()
        XCTAssertTrue(app.staticTexts[title].waitForExistence(timeout: 10))
        let row = app.cells.containing(.staticText, identifier: title).firstMatch
        row.buttons["Mark complete"].tap()
        XCTAssertTrue(row.buttons["Mark incomplete"].waitForExistence(timeout: 10))
        row.buttons["Item actions"].tap()
        app.buttons["Edit"].tap()
        let edit = app.textFields["edit-title"]
        edit.tap()
        edit.typeText(" updated")
        app.buttons["save-item"].tap()
        XCTAssertTrue(app.staticTexts[title + " updated"].waitForExistence(timeout: 10))
        let updated = app.cells.containing(.staticText, identifier: title + " updated").firstMatch
        updated.buttons["Item actions"].tap()
        app.buttons["Delete"].tap()
        app.buttons["Delete"].tap()
        XCTAssertTrue(app.staticTexts[title + " updated"].waitForNonExistence(timeout: 10))
        app.buttons["preferences"].tap()
        app.buttons["Sign out"].tap()
        XCTAssertTrue(app.buttons["sign-in"].waitForExistence(timeout: 20))
    }
}
