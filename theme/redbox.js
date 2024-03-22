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
  //Default hiding the redbox
  document.getElementById("aspect-ratio-helper").style.display = "none";
})();

//Create a function to button to perform on click action.
function redboxButtonClicked() {
  var hideShowButton = document.getElementById("redbox");
  if (document.getElementById("aspect-ratio-helper").style.display === "none") {
    document.getElementById("aspect-ratio-helper").style.display = "block";
    hideShowButton.innerHTML = "aspect-ratio box";
  } else {
    document.getElementById("aspect-ratio-helper").style.display = "none";
    hideShowButton.innerHTML = "aspect-ratio box";
  }
}
window.redboxButtonClicked = redboxButtonClicked;
