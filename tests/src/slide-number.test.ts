import { $, browser, expect } from "@wdio/globals";
import { describe, it } from "mocha";

describe("Slide Number Pill", () => {
  const pill = () => $("#slide-number-pill");
  const sidebarToggle = () => $("#mdbook-sidebar-toggle");

  beforeEach(async () => {
    await browser.url("/hello-world.html");
    await browser.execute(() => localStorage.removeItem("mdbook-sidebar"));
    await browser.refresh();
    await $("#mdbook-sidebar a.active").waitForExist();
  });

  it("should be hidden when the sidebar is visible", async () => {
    await expect($("html")).toHaveElementClass("sidebar-visible");
    await expect(pill()).toExist();
    await expect(pill()).not.toBeDisplayed();
  });

  it("should display the active chapter number when the sidebar is hidden", async () => {
    const activeNumber = await $("#mdbook-sidebar a.active strong");
    const expected = (await activeNumber.getText()).trim().replace(/\.$/, "");

    await sidebarToggle().click();

    await expect($("html")).not.toHaveElementClass("sidebar-visible");
    await expect(pill()).toBeDisplayed();
    await expect(pill()).toHaveText(expected);
  });

  it("should open the sidebar and hide the pill when clicked", async () => {
    await sidebarToggle().click();
    await expect(pill()).toBeDisplayed();

    await pill().click();

    await expect($("html")).toHaveElementClass("sidebar-visible");
    await expect(pill()).not.toBeDisplayed();
  });

  it("should not exist on unnumbered pages", async () => {
    await browser.url("/");
    await browser.refresh();
    await $("#mdbook-sidebar a.active").waitForExist();

    await sidebarToggle().click();

    await expect($("html")).not.toHaveElementClass("sidebar-visible");
    await expect(pill()).not.toExist();
  });
});
