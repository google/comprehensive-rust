(function () {
  if (window.location.hash === "#speaker-notes-open") {
    return;
  }

  const redBox = document.createElement("div");
  redBox.id = "aspect-ratio-helper";
  redBox.style.display = "none"; // Initially hidden

  const nestedDiv = document.createElement("div");
  redBox.appendChild(nestedDiv);

  const turnOffButton = document.createElement("button");
  turnOffButton.id = "turn-off-red-box";
  turnOffButton.textContent = "Hide Red Box";
  nestedDiv.appendChild(turnOffButton);

  document.body.prepend(redBox);

  const storageKey = "showRedBox";

  const turnOff = () => {
    redBox.style.display = "none";
    sessionStorage.removeItem(storageKey);
  };

  const turnOn = () => {
    sessionStorage.setItem(storageKey, "true");
    redBox.style.display = "block";
  };

  const toggleRedBox = () => {
    if (redBox.style.display === "none") {
      turnOn();
    } else {
      turnOff();
    }
  };

  document.addEventListener("keydown", (event) => {
    // Toggle the red box with Ctrl+Alt+B
    if (event.ctrlKey && event.altKey && event.key === "b") {
      event.preventDefault();
      toggleRedBox();
    }
  });

  turnOffButton.addEventListener("click", turnOff);

  // Check initial state from URL parameter or session storage
  const searchParams = new URLSearchParams(window.location.search);
  const showRedBoxParam = searchParams.get("show-red-box");

  if (showRedBoxParam === "true") {
    turnOn();
  } else if (showRedBoxParam === "false") {
    turnOff();
  } else if (sessionStorage.getItem(storageKey) === "true") {
    turnOn();
  }
})();
