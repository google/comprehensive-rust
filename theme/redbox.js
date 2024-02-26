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
hideShowButton.innerHTML = "redbox"
hideShowButton.className = "icon-button"
hideShowButton.type = "button"
hideShowButton.title = "showing/hiding red box"
hideShowButton.id = "Dev"
var navbarButtons = document.getElementsByClassName('left-buttons')
navbarButtons[0].insertBefore(hideShowButton, navbarButtons.firstChild);
document.getElementById("aspect-ratio-helper").style.display = "none";
hideShowButton.addEventListener("click", function () {
    if (document.getElementById("aspect-ratio-helper").style.display === "none") {
        document.getElementById("aspect-ratio-helper").style.display = "block";
        hideShowButton.innerHTML = "User"
    } else {
        document.getElementById("aspect-ratio-helper").style.display = "none";
        hideShowButton.innerHTML = "Dev"
    }
})