(function redBoxButton() {
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
  var hideShowButton = document.createElement("button");
  hideShowButton.innerHTML = '<i class="fa fa-square-o"></i>';
  hideShowButton.className = "icon-button";
  hideShowButton.type = "button";
  hideShowButton.title =
    "Outline the area that fits on one screen while teaching the course.";
  hideShowButton.id = "Dev";
  var navbarButtons = document.getElementsByClassName("left-buttons");
  navbarButtons[0].insertBefore(hideShowButton, navbarButtons.firstChild);
  //Default hiding the redbox
  document.getElementById("aspect-ratio-helper").style.display = "none";
  //Add Event listener to button to perform on click action.
  hideShowButton.addEventListener("click", function () {
    if (
      document.getElementById("aspect-ratio-helper").style.display === "none"
    ) {
      document.getElementById("aspect-ratio-helper").style.display = "block";
      hideShowButton.innerHTML = '<i class="fa fa-square"></i>';
    } else {
      document.getElementById("aspect-ratio-helper").style.display = "none";
      hideShowButton.innerHTML = '<i class="fa fa-square-o"></i>';
    }
  });
})();

function setCodeToPlayground() {
  var codes = JSON.parse(localStorage.getItem(window.location.href));
  if(codes){
    var i = 0;
    Array.from(document.querySelectorAll(".playground")).forEach(function (pre_block) {
    let code_block = pre_block.querySelector("code");
    let editor = window.ace.edit(code_block);
    editor.setValue(codes[i]);
    editor.clearSelection();
    i += 1
    })
  }
}
setTimeout(setCodeToPlayground, 100);
function getCodeFromPlayground() {
  var codes = []
  Array.from(document.querySelectorAll(".playground")).forEach(function (pre_block) {
    let code_block = pre_block.querySelector("code");
    let editor = window.ace.edit(code_block);
    let code = editor.getValue();
    codes.push(code)
  })
  localStorage.setItem(window.location.href,JSON.stringify(codes));
};
setInterval(getCodeFromPlayground,900)
