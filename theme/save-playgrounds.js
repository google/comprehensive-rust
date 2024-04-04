(function savePlaygrounds() {
  function setCodeToPlayground() {
    Array.from(document.querySelectorAll(".playground")).forEach(function (
      pre_block,
      index
    ) {
      let code_block = pre_block.querySelector("code");
      let editor = window.ace.edit(code_block);
      code = JSON.parse(
        localStorage.getItem(`${window.location.href}₹${index}`)
      );
      if (code) {
        editor.setValue(code);
        editor.clearSelection();
      }
    });
  }
  function getCodeFromPlayground() {
    Array.from(document.querySelectorAll(".playground")).forEach(function (
      pre_block,
      index
    ) {
      let code_block = pre_block.querySelector("code");
      let editor = window.ace.edit(code_block);
      editor.session.on("change", function () {
        let code = editor.getValue();
        localStorage.setItem(
          `${window.location.href}₹${index}`,
          JSON.stringify(code)
        );
      });
    });
  }
  setCodeToPlayground();
  getCodeFromPlayground();
})();

function resetPlaygroundsClicked() {
    Array.from(document.querySelectorAll(".playground")).forEach(function (
    pre_block,
    index
  ) {
    let code_block = pre_block.querySelector("code");
    let editor = window.ace.edit(code_block);
    editor.setValue(editor.originalCode)
    editor.clearSelection();
  });
  
  let keys = [];
  for (var i = 0, len = localStorage.length; i < len; i++) {
    if (localStorage.key(i).includes("₹")) {
      keys.push(localStorage.key(i));
    }
  }
  for (let j = 0; j < keys.length; j++) {
    localStorage.removeItem(keys[j]);
  }
}
window.resetPlaygroundsClicked = resetPlaygroundsClicked;
