# Bienvenue dans Comprehensive Rust 🦀

[![Build workflow](https://img.shields.io/github/actions/workflow/status/google/comprehensive-rust/build.yml?style=flat-square)](https://github.com/google/comprehensive-rust/actions/workflows/build.yml?query=branch%3Amain)
[![GitHub contributors](https://img.shields.io/github/contributors/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/graphs/contributors)
[![GitHub stars](https://img.shields.io/github/stars/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/stargazers)

Ceci est un cours gratuit sur Rust développé par l’équipe Android de Google. Le cours
couvre l’ensemble de Rust, de la syntaxe de base aux sujets avancés comme
les génériques et la gestion des erreurs.

> La version la plus récente du cours se trouve à l’adresse :
> <https://google.github.io/comprehensive-rust/>. Si vous le lisez ailleurs,
> vérifiez cette page pour obtenir les dernières mises à jour.
>
> Le cours est disponible dans d’autres langues. Sélectionnez votre langue préférée
> en haut à droite de la page ou consultez la page
> [Traductions](running-the-course/translations.md) pour la liste complète des traductions disponibles.
>
> Le cours est également disponible [au format PDF](comprehensive-rust.pdf).

L’objectif de ce cours est de vous apprendre Rust. Nous partons du principe que vous
ne connaissez rien à Rust, et nous espérons :

- Vous donner une compréhension complète de la syntaxe et du langage Rust.
- Vous permettre de modifier des programmes existants et d’en écrire de nouveaux en Rust.
- Vous présenter les idiomes courants en Rust.

Nous appelons les quatre premiers jours du cours les Fondamentaux Rust.

En complément, vous êtes invité à explorer un ou plusieurs sujets spécialisés :

- [Android](android.md) : un cours d’une demi-journée sur l’utilisation de Rust pour le développement
  de la plateforme Android (AOSP). Cela inclut l’interopérabilité avec C, C++ et Java.
- [Chromium](chromium.md) : un cours d’une demi-journée sur l’utilisation de Rust dans les navigateurs
  basés sur Chromium. Cela inclut l’interopérabilité avec C++ et l’intégration de
  *crates* tierces dans Chromium.
- [Bare-metal](bare-metal.md) : un cours d’une journée entière sur l’utilisation de Rust pour
  le développement *bare-metal* (embarqué). Les microcontrôleurs et processeurs applicatifs
  y sont abordés.
- [Concurrence](concurrency/welcome.md) : un cours d’une journée entière sur la concurrence en Rust.
  Nous couvrons la concurrence classique (planification préemptive avec threads et mutex)
  ainsi que la concurrence async/await (coopérative avec *futures*).

## Objectifs non couverts

Rust est un langage vaste, et nous ne pourrons pas tout aborder en quelques jours.
Ce que nous ne couvrons pas dans ce cours :

- Apprendre à développer des macros : veuillez consulter
  [le chapitre 19.5 du Rust Book](https://doc.rust-lang.org/book/ch19-06-macros.html)
  et [Rust by Example](https://doc.rust-lang.org/rust-by-example/macros.html) à la place.

## Hypothèses

Le cours part du principe que vous savez déjà programmer. Rust est un
langage typé statiquement, et nous ferons parfois des comparaisons avec C ou C++
pour mieux expliquer ou contraster l’approche de Rust.

Si vous savez programmer dans un langage typé dynamiquement comme Python ou
JavaScript, vous pourrez suivre sans problème.

<details>

Ceci est un exemple de _note de présentation_. Nous les utiliserons pour ajouter
des informations complémentaires aux diapositives. Il peut s’agir de points clés
à aborder par l’instructeur ou de réponses aux questions fréquemment posées en classe.

</details>