import { describe, it } from "mocha";
import { expect, browser, $ } from "@wdio/globals";

describe("Generic page", () => {
  beforeEach(async () => {
    await browser.url("/");
  });

  it("should have the default_theme light or navy", async () => {
    const html = await $("html");
    const className = await html.getAttribute("class");
    // Note: The default theme can be "light" or "navy" depending on the preferred
    // color scheme of the system running the tests.
    const hasDefaultTheme =
      className.includes("light") || className.includes("navy");
    expect(hasDefaultTheme).toBe(true);
  });

  it("should have theme button and show theme list on click", async () => {
    await expect($("#mdbook-theme-toggle")).toBeDisplayed();
    await $("#mdbook-theme-toggle").click();
    await expect($("#mdbook-theme-list")).toBeDisplayed();
  });

  it("should have search button and successfully provide search results on search", async () => {
    await expect($("#mdbook-search-toggle")).toBeDisplayed();
    await $("#mdbook-search-toggle").click();
    await browser.keys(["Welcome"]);
    // any of the <a> links in the searchresults is containing "Welcome"
    await expect($("#mdbook-searchresults").$("*=Welcome")).toBeDisplayed();
  });

  it("should have language button and show language list on click", async () => {
    await expect($("#language-toggle")).toBeDisplayed();
    await $("#language-toggle").click();
    await expect($("#language-list")).toBeDisplayed();
  });
});
