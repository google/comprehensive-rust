import { describe, it } from "mocha";
import { $, expect, browser } from "@wdio/globals";

describe("speaker-notes", () => {
  beforeEach(async () => {
    await browser.url("/");
  });

  it("contains summary with heading and button", async () => {
    const summary$ = await $("details summary");
    await expect(summary$).toExist();
    await expect(summary$.$("#speaker-notes")).toHaveText("Speaker Notes");
    await expect(summary$.$(".pop-out")).toExist();
  });

  it("opens a new window on button click and hide details on main page", async () => {
    const details$ = await $("details");
    const button$ = await $("details summary .pop-out");
    await expect(details$).toBeDisplayed();
    button$.scrollIntoView();
    await button$.click();
    await expect(details$).not.toBeDisplayed();

    // a new window should have opened, it should be the second one
    const handles = await browser.getWindowHandles();
    await browser.switchToWindow(handles[1]);
    await expect(browser).toHaveUrl(
      expect.stringContaining("#speaker-notes-open")
    );
  });
});
