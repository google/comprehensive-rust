import { describe, it } from "mocha";
import { $, expect, browser } from "@wdio/globals";
import { Key } from "webdriverio";

describe("Playground", () => {
  it("executes the hello world code and prints the hello message", async () => {
    await browser.url("/hello-world/hello-world.html");
    const playground_area = $(".ace_content");
    const playground_start_button = $("button.play-button");
    const playground_stderr = $("code.result.stderr");
    const playground_stdout = $("code.result.stdout");

    // ensure a playground exists and pre-state is as expected
    await expect(playground_area).toExist();
    await expect(playground_start_button).toExist();
    await expect(playground_start_button).not.toBeDisplayed();
    await expect(playground_stderr).not.toExist();
    await expect(playground_stdout).not.toExist();

    // clicking into the content is necessary for the button to be displayed
    await playground_area.waitForClickable();
    await playground_area.click();
    await expect(playground_start_button).toBeDisplayed();

    // clicking the button triggers action
    await playground_start_button.waitForClickable();
    await playground_start_button.click();
    await expect(playground_stdout).toBeDisplayed();
    await expect(playground_stderr).not.toBeDisplayed();
    await expect(playground_stdout).toHaveText(expect.stringContaining("🌍"));
  });

  it("shows error messages in stderr if the code is broken", async () => {
    await browser.url("/hello-world/hello-world.html");
    const playground_area = $(".ace_content");
    const playground_start_button = $("button.play-button");
    const playground_stderr = $("code.result.stderr");
    const playground_stdout = $("code.result.stdout");

    // clicking into the content is necessary for the button to be displayed
    await playground_area.waitForClickable();
    await playground_area.click();

    // append some failing code to the editor that is now in focus
    await browser.keys([Key.Enter, "fn expect_failure()"]);

    // clicking the button triggers action
    await playground_start_button.waitForClickable();
    await playground_start_button.click();
    await expect(playground_stdout).toBeDisplayed();
    await expect(playground_stderr).toBeDisplayed();

    // check for error message in stderr
    await expect(playground_stderr).toHaveText(
      expect.stringContaining("error: could not compile")
    );
    await expect(playground_stdout).toHaveText("No output");
  });
});
