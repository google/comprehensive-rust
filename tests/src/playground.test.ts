import { describe, it } from "mocha";
import { expect, browser } from "@wdio/globals";
import { Key } from "webdriverio";
import Playground from "./objects/playground";

describe("Playground", () => {
  beforeEach(async () => {
    // this page contains a hello world playground
    await browser.url("/hello-world/hello-world.html");
  });

  it("exists and is shown but output fields do not yet exist", async () => {
    const playground = new Playground();
    // ensure a playground exists and pre-state is as expected
    await expect(playground.area).toExist();
    await expect(playground.start_button).toExist();
    await expect(playground.start_button).not.toBeDisplayed();
    await expect(playground.stderr).not.toExist();
    await expect(playground.stdout).not.toExist();
  });

  it("executes the hello world code and prints the hello message", async () => {
    const playground = new Playground();
    await playground.mock_success("Hello world!\n");
    await playground.activate();
    await playground.run();

    await expect(playground.stdout).toBeDisplayed();
    await expect(playground.stderr).not.toBeDisplayed();
    await expect(playground.stdout).toHaveText("Hello world!");
  });

  it("shows error messages in stderr if the code is broken", async () => {
    const playground = new Playground();
    await playground.mock_fail();
    await playground.activate();
    // append some failing code to the editor that is now in focus
    await browser.keys([Key.Enter, "fn expect_failure()"]);
    await playground.run();

    await expect(playground.stdout).toBeDisplayed();
    await expect(playground.stderr).toBeDisplayed();
    // check for error message in stderr
    await expect(playground.stderr).toHaveText(
      expect.stringContaining("error: could not compile"),
    );
    await expect(playground.stdout).toHaveText("No output");
  });
});
