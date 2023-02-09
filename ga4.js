// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Look through all Playgrounds on the page to determine if the code snippet
// matches one of the. If the code is different from all Playgrounds, we
// conclude that the user modified the Playground before submitting it.
function isPlaygroundCodeModified(code) {
  // It sounds expensive to look through every Playground, but there are
  // normally at most two Playground instances on a page.
  let playgrounds = Array.from(document.querySelectorAll(".playground"));
  return playgrounds.every(playground => {
    let code_block = playground.querySelector("code");
    if (window.ace && code_block.classList.contains("editable")) {
      let editor = window.ace.edit(code_block);
      return code != editor.originalCode;
    } else {
      return code != code_block.textContent;
    }
  });
}

// Monkey-patch the window.fetch function so we can track the Playground
// executions.
const playgroundUrl = 'https://play.rust-lang.org/evaluate.json';
const { fetch: originalFetch } = window;
window.fetch = async (...args) => {
  let [resource, config ] = args;
  if (resource != playgroundUrl) {
    return originalFetch(resource, config);
  }

  const startTime = window.performance.now();
  let endTime, errorMessage;
  try {
    // The fetch_with_timeout function defaults to a 6000 ms timeout. We use a
    // slightly shorter timeout so that we can catch and log the error.
    config.signal = AbortSignal.timeout(5500);
    let response = await originalFetch(resource, config);
    payload = await response.json();
    errorMessage = (payload.error == null) ? null : 'compilation_error';
    // Return object compatible with the unpackign done in book.js.
    return {'json': () => payload};
  } catch (error) {
    // fetch seems to always return AbortError, despite the example on
    // https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/timeout.
    if (error.name == 'AbortError' || error.name == 'TimeoutError') {
      error = new Error('timeout');
    }
    errorMessage = error.message;
    throw error;
  } finally {
    endTime = window.performance.now();
    let code = JSON.parse(config.body).code;
    gtag("event", "playground", {
      "modified": isPlaygroundCodeModified(code),
      "error": errorMessage,
      "latency": (endTime - startTime) / 1000,
    });
  }
};
