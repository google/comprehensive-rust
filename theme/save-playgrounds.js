(function savePlaygrounds() {
  function setCodeToPlayground() {
    var codes = JSON.parse(
      localStorage.getItem(`${window.location.href}₹code`)
    );
    if (codes) {
      var i = 0;
      Array.from(document.querySelectorAll(".playground")).forEach(function (
        pre_block
      ) {
        let code_block = pre_block.querySelector("code");
        let editor = window.ace.edit(code_block);
        editor.setValue(codes[i]);
        editor.clearSelection();
        i += 1;
      });
    }
  }
  function getCodeFromPlayground() {
    var codes = [];
    Array.from(document.querySelectorAll(".playground")).forEach(function (
      pre_block
    ) {
      let code_block = pre_block.querySelector("code");
      let editor = window.ace.edit(code_block);
      let code = editor.getValue();
      codes.push(code);
    });
    localStorage.setItem(`${window.location.href}₹code`, JSON.stringify(codes));
  }
  setCodeToPlayground();
  addEventListener("pagehide", getCodeFromPlayground);
})();

function resetPlaygroundsClicked() {
  let keys = [];
  for (var i = 0, len = localStorage.length; i < len; i++) {
    if (localStorage.key(i).includes("₹code")) {
      keys.push(localStorage.key(i));
    }
  }
  for (let j = 0; j < keys.length; j++) {
    localStorage.removeItem(keys[j]);
  }
}
window.resetPlaygroundsClicked = resetPlaygroundsClicked;
