// Copyright 2026 Google LLC
// SPDX-License-Identifier: Apache-2.0
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

(function () {
  if (
    window.location.hash === "#speaker-notes-open" ||
    window.location.pathname.endsWith("/print.html")
  ) {
    return;
  }

  function renderPill() {
    if (document.getElementById("slide-number-pill")) {
      return true;
    }

    const activeLink = document.querySelector("#mdbook-sidebar a.active");
    if (!activeLink) {
      return false;
    }

    const strong = activeLink.querySelector("strong");
    if (!strong) {
      // Unnumbered page (such as the root welcome page).
      return true;
    }

    const slideNumber = strong.textContent.trim().replace(/\.$/, "");
    if (!slideNumber) {
      return true;
    }

    const pill = document.createElement("button");
    pill.id = "slide-number-pill";
    pill.type = "button";
    pill.textContent = slideNumber;
    pill.setAttribute("aria-label", `Slide ${slideNumber}`);
    pill.setAttribute("title", activeLink.textContent.trim());
    pill.addEventListener("click", () => {
      const sidebarToggle = document.getElementById("mdbook-sidebar-toggle");
      if (sidebarToggle) {
        sidebarToggle.click();
      }
    });

    document.body.appendChild(pill);
    return true;
  }

  if (renderPill()) {
    return;
  }

  const sidebar = document.getElementById("mdbook-sidebar");
  if (!sidebar) {
    return;
  }

  const observer = new MutationObserver(() => {
    if (renderPill()) {
      observer.disconnect();
    }
  });
  observer.observe(sidebar, { childList: true, subtree: true });
})();
