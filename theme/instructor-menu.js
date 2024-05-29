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
    playgroundStateButton.id = "playground-state";

    instructorMenu.addEventListener("click", () => {
      if (instructorMenuList.style.display === "none") {
        instructorMenuList.style.display = "block";
      } else {
        instructorMenuList.style.display = "none";
      }
    });

    document.addEventListener("click", (e) => {
      if (!instructorMenu.contains(e.target)) {
        instructorMenuList.style.display = "none";
      }
    });
  }
  handleInstructorMenu();
  var redBoxButton = document.getElementById("redbox");
  var playgroundStateButton = document.getElementById("playground-state");
  redBoxButton.addEventListener("click", () => window.redboxButtonClicked());
  playgroundStateButton.addEventListener("click", () =>
    window.resetPlaygroundsClicked()
  );
})();
