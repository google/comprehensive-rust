# Testing the comprehensive rust course book

The course material contains JS code that can break and needs to be checked for
functionality. Examples are `theme/speaker-notes.js` or `theme/book.js`.

Comprehensive Rust is using [webdriverIO](https://webdriver.io/) and the
[webdriverIO Expect API](https://webdriver.io/docs/api/expect-webdriverio/) in
combination with [Mocha] (https://mochajs.org/). WebdriverIO is taking care of
accessing the webpage with a real browser and can access the state of the page
so behavior can be asserted.

The [Static Server Service](https://webdriver.io/docs/static-server-service/) is
used mainly in the [CI](../github/workflows/build.yml) to serve the book on port
localhost:8080 such that the test runner can access it. This mode is used when
`npm start` or `npm test` is executed.

For local testing and quick iterations it is possible to use `mdbook serve`
which creates a small HTTP server on port 3000 by default. There is a special
config that is invoked with `npm run test-mdbook` that uses
`http://localhost:3000`
