import { describe, it } from "mocha";
import { expect, browser, $ } from "@wdio/globals";

describe("Basic test", () => {
  beforeEach(async () => {
    await browser.url("/");
  });

  it("should have the default_theme light", async () => {
    expect(await browser.execute(() => window.default_theme)).toBe("light");
  });

  it("should show theme button", async () => {
    await expect($("#theme-toggle")).toBeDisplayed();
  });

  it("should show search button", async () => {
    await expect($("#search-toggle")).toBeDisplayed();
  });
});
