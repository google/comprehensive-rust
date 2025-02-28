// Copyright 2023 Google LLC
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
  // Valid speaker notes states
  const NotesState = {
    Popup: "popup",
    Inline: "inline-open",
    Closed: "inline-closed",
  };

  // The mode/function of this window
  const WindowMode = {
    Regular: "regular",
    RegularWithSpeakerNotes: "regular-speaker-notes",
    SpeakerNotes: "speaker-notes",
    SpeakerNotesDefunct: "speaker-notes-defunct",
    PrintPage: "print-page",
  };

  // detect the current window mode based on window location properties
  function detectWindowMode() {
    if (window.location.hash == "#speaker-notes-open") {
      return WindowMode.SpeakerNotes;
    } else if (window.location.hash == "#speaker-notes") {
      return WindowMode.RegularWithSpeakerNotes;
    } else if (window.location.hash == "#speaker-notes-defunct") {
      return WindowMode.SpeakerNotesDefunct;
    } else if (window.location.pathname.endsWith("/print.html")) {
      return WindowMode.PrintPage;
    } else {
      return WindowMode.Regular;
    }
  }

  let notes = document.querySelector("details");
  // Create an unattached DOM node for the code below.
  if (!notes) {
    notes = document.createElement("details");
  }
  let popIn = document.createElement("button");

  // Mark the speaker note window defunct. This means that it will no longer
  // show the notes.
  function markDefunct() {
    const main = document.querySelector("main");
    const h4 = document.createElement("h4");
    h4.append("(You can close this window now.)");
    main.replaceChildren(h4);
    window.location.hash = "#speaker-notes-defunct";
  }

  // Update the window. This shows/hides controls as necessary for regular and
  // speaker note pages.
  function applyState() {
    if (detectWindowMode() == WindowMode.SpeakerNotes) {
      if (getState() != NotesState.Popup) {
        markDefunct();
      }
      return;
    }

    switch (getState()) {
      case NotesState.Popup:
        popIn.classList.remove("hidden");
        notes.classList.add("hidden");
        break;
      case NotesState.Inline:
        popIn.classList.add("hidden");
        notes.open = true;
        notes.classList.remove("hidden");
        break;
      case NotesState.Closed:
        popIn.classList.add("hidden");
        notes.open = false;
        notes.classList.remove("hidden");
        break;
    }
  }

  // Get the state of the speaker note window.
  function getState() {
    return window.localStorage["speakerNotes"] || NotesState.Closed;
  }

  // Set the state of the speaker note window. Call applyState as needed
  // afterwards.
  function setState(state) {
    window.localStorage["speakerNotes"] = state;
  }

  // Create controls for a regular page.
  function setupRegularPage() {
    // Create pop-in button.
    popIn.setAttribute("id", "speaker-notes-toggle");
    popIn.setAttribute("type", "button");
    popIn.setAttribute("title", "Close speaker notes");
    popIn.setAttribute("aria-label", "Close speaker notes");
    popIn.classList.add("icon-button");
    let popInIcon = document.createElement("i");
    popInIcon.classList.add("fa", "fa-window-close-o");
    popIn.append(popInIcon);
    popIn.addEventListener("click", (event) => {
      setState(NotesState.Inline);
      applyState();
    });
    document.querySelector(".left-buttons").append(popIn);

    // Create speaker notes.
    notes.addEventListener("toggle", (event) => {
      setState(notes.open ? NotesState.Inline : NotesState.Closed);
    });

    let summary = document.createElement("summary");
    notes.insertBefore(summary, notes.firstChild);

    let h4 = document.createElement("h4");
    h4.setAttribute("id", "speaker-notes");
    h4.append("Speaker Notes");
    h4.addEventListener("click", (event) => {
      // Update fragment as if we had clicked a link. A regular a element would
      // result in double-fire of the event.
      window.location.hash = "#speaker-notes";
    });
    summary.append(h4);

    // Create pop-out button.
    let popOutLocation = new URL(window.location.href);
    popOutLocation.hash = "#speaker-notes-open";
    let popOut = document.createElement("button");
    popOut.classList.add("icon-button", "pop-out");
    popOut.addEventListener("click", (event) => {
      let popup = window.open(
        popOutLocation.href,
        "speakerNotes",
        NotesState.Popup,
      );
      if (popup) {
        setState(NotesState.Popup);
        applyState();
        // bind the popup to reset the speaker note state on close of the popup
        popup.onload = () => {
          popup.onbeforeunload = () => {
            setState(NotesState.Inline);
            applyState();
          };
        };
      } else {
        window.alert(
          "Could not open popup, please check your popup blocker settings."
        );
      }
    });
    let popOutIcon = document.createElement("i");
    popOutIcon.classList.add("fa", "fa-external-link");
    popOut.append(popOutIcon);
    summary.append(popOut);
  }

  // Create headers on the print page.
  function setupPrintPage() {
    for (const notes of document.querySelectorAll("details")) {
      notes.open = true;
      let summary = document.createElement("summary");
      notes.insertBefore(summary, notes.firstChild);
      let h4 = document.createElement("h4");
      h4.append("Speaker Notes");
      summary.append(h4);
    }
  }

  // Create controls for a speaker note window.
  function setupSpeakerNotes() {
    // Hide sidebar and buttons.
    document.querySelector("html").classList.remove("sidebar-visible");
    document.querySelector("html").classList.add("sidebar-hidden");
    document.querySelector(".left-buttons").classList.add("hidden");
    document.querySelector(".right-buttons").classList.add("hidden");

    // Hide content except for the speaker notes and h1 elements.
    const main = document.querySelector("main");
    let children = main.childNodes;
    let i = 0;
    while (i < children.length) {
      const node = children[i];
      switch (node.tagName) {
        case "DETAILS":
          // We found the speaker notes: extract their content.
          let div = document.createElement("div");
          div.replaceChildren(...node.childNodes);
          node.replaceWith(div);
          i += 1;
          break;
        case "H1":
          // We found a header: turn it into a smaller header for the speaker
          // note window.
          let h4 = document.createElement("h4");
          let pageLocation = new URL(window.location.href);
          pageLocation.hash = "";
          let a = document.createElement("a");
          a.setAttribute("href", pageLocation.href);
          a.append(node.innerText);
          h4.append("Speaker Notes for ", a);
          node.replaceWith(h4);
          i += 1;
          break;
        default:
          // We found something else: remove it.
          main.removeChild(node);
      }
    }

    // Update prev/next buttons to keep speaker note state.
    document
      .querySelectorAll('a[rel~="prev"], a[rel~="next"]')
      .forEach((elem) => {
        elem.href += "#speaker-notes-open";
      });
  }

  let timeout = null;
  // This will fire on _other_ open windows when we change window.localStorage.
  window.addEventListener("storage", (event) => {
    switch (event.key) {
      case "currentPage":
        if (getState() == NotesState.Popup) {
          // We link all windows when we are showing speaker notes.
          window.location.pathname = event.newValue;
        }
        break;
      case "speakerNotes":
        // When navigating to another page, we see two state changes in rapid
        // succession:
        //
        // - "popup" -> "inline-open"
        // - "inline-open" -> "popup"
        //
        // When the page is closed, we only see:
        //
        // - "popup" -> "inline-open"
        //
        // We can use a timeout to detect the difference. The effect is that
        // showing the speaker notes is delayed by 500 ms when closing the
        // speaker notes window.
        if (timeout) {
          clearTimeout(timeout);
        }
        timeout = setTimeout(applyState, 500);
        break;
    }
  });
  window.localStorage["currentPage"] = window.location.pathname;

  // apply the correct state for the window
  switch (detectWindowMode()) {
    case WindowMode.SpeakerNotesDefunct:
      // mark the window defunct then fall-through into speaker notes
      markDefunct();
    case WindowMode.SpeakerNotes:
      setupSpeakerNotes();
      break;
    case WindowMode.PrintPage:
      setupPrintPage();
      break;
    case WindowMode.RegularWithSpeakerNotes:
      // Regular page with inline speaker notes, set state then fall-through
      setState(NotesState.Inline);
    case WindowMode.Regular:
      applyState();
      setupRegularPage();
      break;
  }
})();
