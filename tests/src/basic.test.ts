import { describe, it } from "mocha";
import { expect, browser } from "@wdio/globals";

describe("Basic test", () => {
  it("should have the default_theme light", async () => {
    await browser.url("/");
    expect(await browser.execute(() => window.default_theme)).toBe("light");
  });
});
