(function handleInstructor() {
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

    instructorMenu.title = "Utilities for course instructors";
    instructorMenu.innerHTML =
      '<i class="fa fa-ellipsis-v" aria-hidden="true"></i>';
    redBoxButton.innerHTML = "aspect-ratio box";
    redBoxButton.title =
      "Outline the area that fits on one screen while teaching the course.";
    playgroundStateButton.innerHTML = "reset all playgrounds";
    playgroundStateButton.title =
      "Reset code in all playgrounds to its original value.";

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
    instructorMenuList.id = "instructor-menu-list";

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

    document.addEventListener("click", (e) => {
      if (
        !instructorMenu.contains(e.target) &&
        !instructorMenuList.contains(e.target)
      ) {
        instructorMenuList.style.display = "none";
      }
    });
  }
  handleInstructorMenu();
  window.redBoxButton();
})();
