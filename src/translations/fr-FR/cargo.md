# Utiliser Cargo

Lorsque vous commencerez à lire sur Rust, vous rencontrerez bientôt
[Cargo](https://doc.rust-lang.org/cargo/), l'outil standard utilisé dans
l'écosystème Rust pour construire et exécuter des applications. Nous souhaitons ici donner un bref
aperçu de ce qu'est Cargo et de la manière dont il s'intègre dans l'écosystème au sens large, ainsi que la manière dont
il s'intègre dans cette formation.

## Installation

> **Suivez s'il-vous-plaît les instructions sur <https://rustup.rs/>.**

Cela vous donnera l'outil de construction (`cargo`) et le compilateur Rust
(`rustc`). Vous obtiendrez également `rustup`, un utilitaire en ligne de commande que vous pouvez utiliser
pour installer différentes versions du compilateur.

Après avoir installé Rust, vous devriez configurer votre éditeur ou IDE pour travailler avec
Rust. La plupart des éditeurs le font en s'adressant à [rust-analyzer], qui fournit
des fonctionnalités d'auto-complétion et de saut à la définition pour [VS Code], [Emacs],
[Vim/Neovim], et bien d'autres. Il existe également un IDE différent appelé
[RustRover].

<details>

- Sur Debian/Ubuntu, vous pouvez également installer Cargo, la source Rust et le
  [Rust formatter] via `apt`. Cependant, cela vous donne une version de Rust périmée
  et peut conduire à des comporemenets inattendus. La commande est la suivante :

  ```shell
  sudo apt install cargo rust-src rustfmt
  ```

- Sur macOS, vous pouvez utiliser [Homebrew](https://brew.sh/) pour installer Rust, mais cela pourrait fournir une version périmée. Par conséquent, il est recommandé d'installer Rust depuis le site officiel.

</details>

[rust-analyzer]: https://rust-analyzer.github.io/
[VS Code]: https://code.visualstudio.com/
[Emacs]: https://rust-analyzer.github.io/manual.html#emacs
[Vim/Neovim]: https://rust-analyzer.github.io/manual.html#vimneovim
[RustRover]: https://www.jetbrains.com/rust/
[Formatteur Rust]: https://github.com/rust-lang/rustfmt
