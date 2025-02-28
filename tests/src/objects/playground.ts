import { $, browser } from "@wdio/globals";

export default class Playground {
  /**
   * stores a playground mock
   */
  mock: WebdriverIO.Mock;

  get area() {
    return $(".ace_content");
  }
  get start_button() {
    return $("button.play-button");
  }
  get stderr() {
    return $("code.result.stderr");
  }
  get stdout() {
    return $("code.result.stdout");
  }

  /**
   * activate the playground by clicking into it
   */
  async activate() {
    // clicking into the content is necessary for the button to be displayed
    await this.area.waitForClickable();
    await this.area.click();
  }

  /**
   *  run the code in the playground editor
   */
  async run() {
    // clicking the button triggers action
    await this.start_button.waitForClickable();
    // await browser.debug();
    await this.start_button.click();
  }

  /**
   * generate a new mock for the playground that overrides playground requests
   */
  async mock_new() {
    return browser.mock("https://play.rust-lang.org/execute", {
      method: "post",
    });
  }

  /**
   * reset the mock object
   */
  async mock_reset() {
    this.mock.clear();
  }

  /**
   * mock a successful playground run
   * @param stdout a string that is expected to be in stdout
   */
  async mock_success(stdout: string) {
    this.mock = await this.mock_new();

    // be aware that there is a still a preflight OPTIONS request
    this.mock.respond(
      {
        success: true,
        exitDetail: "Exited with status 0",
        stdout: stdout,
        stderr:
          "   Compiling playground v0.0.1 (/playground)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.62s\n     Running `target/debug/playground`\n",
      },
      {
        headers: {
          "Access-Control-Allow-Origin": "*",
        },
        fetchResponse: false,
      },
    );
  }

  /**
   * mock a failed run that would be caused if a `fn fail()` is added without any function body
   */
  async mock_fail() {
    // mock the response so this test does not rely on the playground backend to be working
    this.mock = await this.mock_new();

    // be aware that there is a still a preflight OPTIONS request
    this.mock.respond(
      {
        success: false,
        exitDetail: "Exited with status 101",
        stdout: "",
        stderr:
          '   Compiling playground v0.0.1 (/playground)\nerror: expected one of `->`, `where`, or `{`, found `}`\n --> src/main.rs:5:1\n  |\n4 |     fn fail()\n  |        ----  - expected one of `->`, `where`, or `{`\n  |        |\n  |        while parsing this `fn`\n5 | }\n  | ^ unexpected token\n\nerror: free function without a body\n --> src/main.rs:4:5\n  |\n4 |     fn fail()\n  |     ^^^^^^^^^- help: provide a definition for the function: `{ <body> }`\n\nerror: could not compile `playground` (bin "playground") due to 2 previous errors\n',
      },
      {
        headers: {
          "Access-Control-Allow-Origin": "*",
        },
        fetchResponse: false,
      },
    );
  }
}
