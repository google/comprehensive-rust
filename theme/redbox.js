(function handleInstructor(params) {
  function handleInstructorMenu() {
    let leftButtons = document.getElementsByClassName("left-buttons")[0];
    let instructorMenu = document.createElement("button");
    let instructorMenuList = document.createElement("ul");
    let redBoxItem = document.createElement("li");
    let redBoxButton = document.createElement("button");
    let playgroundStateItem = document.createElement("li");
    let playgroundStateButton = document.createElement("button");

    leftButtons.insertBefore(instructorMenu, leftButtons.lastChild);
    leftButtons.insertBefore(instructorMenuList, leftButtons.lastChild);
    instructorMenuList.insertBefore(redBoxItem, instructorMenuList.lastChild);
    instructorMenuList.insertBefore(
      playgroundStateItem,
      instructorMenuList.lastChild
    );
    redBoxItem.insertBefore(redBoxButton, redBoxItem.lastChild);
    playgroundStateItem.insertBefore(
      playgroundStateButton,
      playgroundStateItem.lastChild
    );

    instructorMenu.innerHTML =
      '<i class="fa fa-ellipsis-v" aria-hidden="true"></i>';
    redBoxButton.innerHTML = "redbox";
    playgroundStateButton.innerHTML = "reset code";

    instructorMenu.className = "icon-button";
    instructorMenuList.className = "theme-popup";
    redBoxButton.className = "theme";
    playgroundStateButton.className = "theme";
    instructorMenuList.style.display = "none";

    instructorMenuList.role = "menu";
    redBoxItem.role = "none";
    playgroundStateItem.role = "none";
    redBoxButton.role = "menuitem";
    playgroundStateButton.role = "menuitem";

    redBoxButton.id = "redbox";

    instructorMenuList.style.marginLeft = "55px";

    instructorMenu.addEventListener("click", () => {
      if (instructorMenuList.style.display === "none") {
        instructorMenuList.style.display = "block";
      } else {
        instructorMenuList.style.display = "none";
      }
    });

    playgroundStateButton.addEventListener("click", () => {
      let keys = [];
      for (var i = 0, len = localStorage.length; i < len; i++) {
        if (localStorage.key(i).includes("₹code")) {
          keys.push(localStorage.key(i));
        }
      }
      for (let j = 0; j < keys.length; j++) {
        localStorage.removeItem(keys[j]);
      }
    });
  }

  function redBoxButton() {
    // Create a new div element
    var newDiv = document.createElement("div");
    // Set the id attribute of the new div
    newDiv.id = "aspect-ratio-helper";
    // Create a nested div inside the new div
    var nestedDiv = document.createElement("div");
    // Append the nested div to the new div
    newDiv.appendChild(nestedDiv, newDiv.firstChild);
    // Get the parent element where you want to append the new div
    var parentElement = document.body; // Change this to your desired parent element
    // Append the new div to the parent element
    parentElement.insertBefore(newDiv, parentElement.firstChild);
    // Create the button element
    var hideShowButton = document.getElementById("redbox");
    hideShowButton.title =
      "Outline the area that fits on one screen while teaching the course.";
    //Default hiding the redbox
    document.getElementById("aspect-ratio-helper").style.display = "none";
    //Add Event listener to button to perform on click action.
    hideShowButton.addEventListener("click", function () {
      if (
        document.getElementById("aspect-ratio-helper").style.display === "none"
      ) {
        document.getElementById("aspect-ratio-helper").style.display = "block";
        hideShowButton.innerHTML = "redbox";
      } else {
        document.getElementById("aspect-ratio-helper").style.display = "none";
        hideShowButton.innerHTML = "redbox";
      }
    });
  }

  handleInstructorMenu();
  redBoxButton();
})();
