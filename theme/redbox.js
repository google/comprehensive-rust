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
  const code = JSON.parse(localStorage.getItem(window.location.href));
  console.log(code);
  if (code) {
    const playground = document.getElementsByClassName("ace_text-layer")[0];
    while (playground.firstElementChild) {
      console.log(playground.firstElementChild.innerText.replace(/\s+/g, " "));
      playground.removeChild(playground.firstElementChild);
    }
    console.log(playground, "after removal");
    for (let i = 0; i < code.length; i++) {
      let parentDiv = code[i][0];
      let spanChild = code[i][1];
      let div = document.createElement(parentDiv.tag);
      div.style.height = "17.5938px";
      div.style.top = `${17.5938 * i}px`;
      for (let cls in parentDiv.classes) {
        div.classList.add(parentDiv.classes[cls]);
      }
      for (let j = 0; j < spanChild.length; j++) {
        //console.log(spanChild[j].styles,typeof(spanChild[j].styles))
        let span = document.createElement(spanChild[j].tag);
        //span.classList = spanChild[j].classes
        for (let cls in spanChild[j].classes) {
          span.classList.add(spanChild[j].classes[cls]);
        }
        span.innerText = spanChild[j].text;
        div.insertBefore(span, div.lastChild);
      }
      playground.insertBefore(div, playground.lastChild);
    }
  }
  localStorage.removeItem(window.location.href);
}

window.onunload = setTimeout(setCodeToPlayground, 5000);
function getCodeFromPlayground() {
  console.log("getCodeFromPlayground");
  const playground =
    document.getElementsByClassName("ace_text-layer")[0].children;
  var code = [];
  for (let i = 0; i < playground.length; i++) {
    let parentCodeList = {
      tag: playground[i].tagName,
      classes: playground[i].classList,
      styles: playground[i].style,
    };
    var line = [];
    for (let j = 0; j < playground[i].children.length; j++) {
      console.log(playground[i].innerHTML);
      let codeList = {
        tag: playground[i].children[j].tagName,
        text: playground[i].children[j].innerText,
        classes: playground[i].children[j].classList,
        styles: playground[i].children[j].style,
      };
      line.push(codeList);
    }
    code.push([parentCodeList, line]);
  }
  console.log(code);
  //localStorage.removeItem(window.location.href)
  localStorage.setItem(window.location.href, JSON.stringify(code));
}
addEventListener("beforeunload", getCodeFromPlayground());
