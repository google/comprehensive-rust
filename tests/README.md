# Testing Comprehensive Rust

The course material contains JS code that can break and needs to be checked for
functionality. Examples are `theme/speaker-notes.js` or `theme/book.js`.

Comprehensive Rust is using [webdriverIO](https://webdriver.io/) and the
[webdriverIO Expect API](https://webdriver.io/docs/api/expect-webdriverio/) in
combination with [Mocha](https://mochajs.org/). WebdriverIO is taking care of
accessing the webpage with a real browser and can access the state of the page
so behavior can be asserted.

The [Static Server Service](https://webdriver.io/docs/static-server-service/) is
used mainly in the [CI](../.github/workflows/build.yml) to serve the book on
port `localhost:8080` such that the test runner can access it. This mode is used
when `npm start` or `npm test` is executed.

> **Tip:** Use `cargo xtask web-tests` to run the tests in this directory from
> anywhere in the repository.

For local testing and quick iterations it is possible to use `cargo xtask serve`
which creates a small HTTP server on port 3000 by default. There is a special
config that is invoked with `npm run test-mdbook` that uses
`http://localhost:3000`

## Deal with failing tests

When you see tests failing they should already indicate what checks broke for
specific pages.

### Legitimate warnings

You might e.g. need to reduce the length of an overlong page (or get an
exemption) or updating some mdbook infrastructure had a breaking change that
breaks functionality. These issues need to be fixed before this change is
merged.

### Broken test environment

Sometimes tests can also fail in the CI environment with errors like this.

```
ERROR webdriver: WebDriverError: tab crashed
```

If you see messages like these, it may indicate an issue with the web-tests that
is not caused by your changes. Please file a bug to report this. As a temporary
workaround, if all other checks pass and you are confident your changes are
correct, you may override the web-test requirement to merge your pull request.
