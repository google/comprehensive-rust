import { describe, it } from "mocha";
import { expect, browser } from "@wdio/globals";

describe("Red Box", () => {
  const redBox = () => $("#aspect-ratio-helper");
  const redBoxButton = () => $("#turn-off-red-box");

  beforeEach(async () => {
    await browser.url("/hello-world.html");
    await browser.execute(() => sessionStorage.clear());
    // Clear any lingering state (like inline styles) from previous
    // tests. Reading https://webdriver.io/docs/api/browser/url,
    // this should not be necessary, but tests fail without it.
    await browser.refresh();
  });

  it("should be hidden by default", async () => {
    await expect(redBox()).not.toBeDisplayed();
  });

  describe("Keyboard Shortcut", () => {
    it("should show the red box when toggled on", async () => {
      await browser.toggleRedBox();
      await expect(redBox()).toBeDisplayed();
      await expect(redBoxButton()).toBeDisplayed();
    });

    it("should hide the red box when toggled off", async () => {
      // Toggle on first
      await browser.toggleRedBox();
      await expect(redBox()).toBeDisplayed();

      // Then toggle off
      await browser.toggleRedBox();
      await expect(redBox()).not.toBeDisplayed();
    });
  });

  describe("URL Parameter", () => {
    it("should show red box", async () => {
      await browser.url("/hello-world.html?show-red-box=true");
      await expect(redBox()).toBeDisplayed();
    });

    it("should override session storage", async () => {
      // Set session storage first to ensure the URL parameter takes precedence.
      await browser.execute(() => sessionStorage.setItem("showRedBox", "true"));
      await browser.url("/hello-world.html?show-red-box=false");
      await expect(redBox()).not.toBeDisplayed();
    });
  });

  describe("Hide Button", () => {
    it("should hide the red box when clicked", async () => {
      await browser.toggleRedBox();
      await expect(redBox()).toBeDisplayed();

      await (await redBoxButton()).click();
      await expect(redBox()).not.toBeDisplayed();
    });
  });

  describe("Session Storage", () => {
    it("should persist being shown after a reload", async () => {
      await browser.toggleRedBox();
      await expect(redBox()).toBeDisplayed();

      await browser.refresh();

      await expect(redBox()).toBeDisplayed();
    });

    it("should persist being hidden after a reload", async () => {
      await browser.toggleRedBox(); // turn on
      await browser.toggleRedBox(); // turn off
      await expect(redBox()).not.toBeDisplayed();

      // Explicitly check that storage is cleared before reloading
      const storage = await browser.execute(() =>
        sessionStorage.getItem("showRedBox"),
      );
      expect(storage).toBeNull();

      await browser.refresh();
      await expect(redBox()).not.toBeDisplayed();
    });
  });

  describe("Interactions", () => {
    it("should be able to be hidden with the keyboard after being shown with the URL", async () => {
      await browser.url("/hello-world.html?show-red-box=true");
      await expect(redBox()).toBeDisplayed();

      await browser.toggleRedBox();
      await expect(redBox()).not.toBeDisplayed();
    });
  });
});
