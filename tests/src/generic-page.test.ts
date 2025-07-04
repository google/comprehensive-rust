import { describe, it } from "mocha";
import { expect, browser, $ } from "@wdio/globals";

describe("Generic page", () => {
  beforeEach(async () => {
    await browser.url("/");
  });

  it("should have the default_theme light", async () => {
    expect(await browser.execute(() => window.default_theme)).toBe("light");
  });

  it("should have theme button and show theme list on click", async () => {
    await expect($("#theme-toggle")).toBeDisplayed();
    await $("#theme-toggle").click();
    await expect($("#theme-list")).toBeDisplayed();
  });

  it("should have search button and successfully provide search results on search", async () => {
    await expect($("#search-toggle")).toBeDisplayed();
    await $("#search-toggle").click();
    await browser.keys(["Welcome"]);
    // any of the <a> links in the searchresults is containing "Welcome"
    await expect($("#searchresults").$("*=Welcome")).toBeDisplayed();
  });

  it("should have language button and show language list on click", async () => {
    await expect($("#language-toggle")).toBeDisplayed();
    await $("#language-toggle").click();
    await expect($("#language-list")).toBeDisplayed();
  });
});
