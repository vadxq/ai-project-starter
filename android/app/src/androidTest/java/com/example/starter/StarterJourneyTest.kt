package com.example.starter

import androidx.compose.ui.test.assertCountEquals
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsOn
import androidx.compose.ui.test.ComposeTimeoutException
import androidx.compose.ui.test.hasAnyAncestor
import androidx.compose.ui.test.hasAnyDescendant
import androidx.compose.ui.test.hasContentDescription
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.v2.createAndroidComposeRule
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.onRoot
import androidx.compose.ui.test.printToLog
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextReplacement
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.uiautomator.UiDevice
import java.util.UUID
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

/** 真实账号密码 / JWT / API 流程，运行前按 README 启动服务并 adb reverse。 */
@RunWith(AndroidJUnit4::class)
class StarterJourneyTest {
    @get:Rule val rule = createAndroidComposeRule<MainActivity>()
    private val device: UiDevice = UiDevice.getInstance(InstrumentationRegistry.getInstrumentation())
    private val timeout: Long = 20_000

    private fun closeKeyboard(): Unit = rule.runOnIdle {
        WindowInsetsControllerCompat(rule.activity.window, rule.activity.window.decorView)
            .hide(WindowInsetsCompat.Type.ime())
    }

    private fun waitForText(text: String): Unit {
        try {
            rule.waitUntil(timeout) { rule.onAllNodesWithText(text).fetchSemanticsNodes().isNotEmpty() }
        } catch (error: ComposeTimeoutException) {
            rule.onRoot(useUnmergedTree = true).printToLog("StarterFailure")
            throw error
        }
    }

    private fun login(): Unit {
        rule.onNodeWithTag("preferences").performClick()
        rule.onNodeWithText("English").performClick()
        rule.onNodeWithTag("username").performTextReplacement("alice")
        rule.onNodeWithTag("password").performTextReplacement("wrong-password")
        closeKeyboard()
        rule.onNodeWithTag("sign-in").performClick()
        waitForText("Incorrect username or password.")
        rule.onNodeWithTag("password").performTextReplacement("starter-password")
        closeKeyboard()
        rule.onNodeWithTag("sign-in").performClick()
        rule.waitUntil(timeout) { rule.onAllNodes(hasTestTag("new-title")).fetchSemanticsNodes().isNotEmpty() }
    }

    @Test fun realLoginCrudPreferencesAndRecreation(): Unit {
        login()
        val title: String = "Android " + UUID.randomUUID().toString().take(8)
        rule.onNodeWithTag("new-title").performTextReplacement(title)
        closeKeyboard()
        rule.onNodeWithTag("add-item").performClick()
        waitForText(title)
        rule.activityRule.scenario.recreate()
        waitForText(title)
        rule.onAllNodesWithText(title).assertCountEquals(1)
        val row = hasTestTag("item-row") and hasAnyDescendant(hasText(title))
        rule.onNode(hasContentDescription("Mark complete") and hasAnyAncestor(row)).performClick()
        rule.waitUntil(timeout) { rule.onAllNodes(hasContentDescription("Mark incomplete") and hasAnyAncestor(row)).fetchSemanticsNodes().isNotEmpty() }
        rule.onNode(hasContentDescription("Mark incomplete") and hasAnyAncestor(row)).assertIsOn()
        rule.onNode(hasText("Edit") and hasAnyAncestor(row)).performClick()
        rule.onNode(hasTestTag("edit-title")).performTextReplacement(title + " updated")
        closeKeyboard()
        rule.onNodeWithText("Save").performClick()
        waitForText(title + " updated")
        rule.onNodeWithTag("preferences").performClick()
        rule.onNodeWithText("Dark").performClick()
        rule.onNodeWithTag("preferences").performClick()
        rule.onNodeWithText("简体中文").performClick()
        waitForText("添加事项")
        device.setOrientationLandscape()
        rule.onNodeWithText("添加事项").assertIsDisplayed()
        device.setOrientationNatural()
        rule.onNodeWithTag("preferences").performClick()
        rule.onNodeWithText("English").performClick()
        val updated = hasTestTag("item-row") and hasAnyDescendant(hasText(title + " updated"))
        rule.onNode(hasText("Delete") and hasAnyAncestor(updated)).performClick()
        rule.onNode(hasText("Delete") and !hasAnyAncestor(hasTestTag("item-row"))).performClick()
        rule.waitUntil(timeout) { rule.onAllNodesWithText(title + " updated").fetchSemanticsNodes().isEmpty() }
        rule.onNodeWithTag("preferences").performClick()
        rule.onNodeWithText("Sign out").performClick()
        rule.waitUntil(timeout) { rule.onAllNodes(hasTestTag("sign-in")).fetchSemanticsNodes().isNotEmpty() }
    }
}
