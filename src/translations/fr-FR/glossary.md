<!-- i18n:comment Please keep { #glossary } untranslated. -->

# Glossaire { #glossary }

Ce qui suit est un glossaire visant à donner une brève définition de nombreux termes de Rust.
Pour les traductions, cela permet également de relier chaque terme à son 
équivalent français.

<style>
h1#glossary ~ ul {
    list-style: none;
    padding-inline-start: 0;
}

h1#glossary ~ ul > li {
    /* Simplify with "text-indent: 2em hanging" when supported:
       https://caniuse.com/mdn-css_properties_text-indent_hanging */
    padding-left: 2em;
    text-indent: -2em;
}

h1#glossary ~ ul > li:first-line {
    font-weight: bold;
}
</style>

<!-- i18n:comment Please add the English term in italic after your -->
<!-- i18n:comment translated term. Also, please keep the hard line -->
<!-- i18n:comment breaks to ensure a nice formatting. -->

- allouer :\
  Allocation dynamique de mémoire sur [le tas](memory-management/review.md). *allocate*
- argument :\
  Information transmise à une fonction ou méthode. *argument*
- type associé :\
  Type associé à un trait spécifique. Utile pour définir une relation entre types. *associated type*
- Rust bare-metal :\
  Développement bas niveau en Rust, souvent destiné à des systèmes sans système d’exploitation. Voir [Rust bare-metal](bare-metal.md). *Bare-metal Rust*
- bloc :\
  Voir [Blocs](control-flow-basics/blocks-and-scopes.md) et _portée_. *block*
- emprunt :\
  Voir [Emprunts](borrowing/shared.md). *borrow*
- vérificateur d’emprunt :\
  Partie du compilateur Rust qui vérifie la validité de tous les emprunts. *borrow checker*
- accolade :\
  `{` et `}`. Aussi appelées _accolades courbes_, elles délimitent les _blocs_. *brace*
- construction :\
  Processus de transformation du code source en un programme exécutable ou utilisable. *build*
- appel :\
  Invocation ou exécution d’une fonction ou méthode. *call*
- canal :\
  Utilisé pour transmettre des messages en toute sécurité [entre threads](concurrency/channels.md). *channel*
- Comprehensive Rust 🦀 :\
  Les cours ici sont collectivement appelés Comprehensive Rust 🦀. *Comprehensive Rust*
- concurrence :\
  Exécution simultanée de plusieurs tâches ou processus. *concurrency*
- Concurrence en Rust :\
  Voir [Concurrence en Rust](concurrency/welcome.md). *Concurrency in Rust*
- constante :\
  Valeur qui ne change pas pendant l’exécution du programme. *constant*
- contrôle de flux :\
  Ordre dans lequel les instructions d’un programme sont exécutées. *control flow*
- plantage :\
  Échec ou arrêt inattendu et non géré d’un programme. *crash*
- énumération :\
  Type de données pouvant contenir une des constantes nommées, parfois avec un tuple ou une structure associée. *enumeration*
- erreur :\
  Condition ou résultat inattendu qui diffère du comportement prévu. *error*
- gestion des erreurs :\
  Processus de traitement et de réponse aux erreurs pendant l’exécution d’un programme. *error handling*
- exercice :\
  Tâche ou problème destiné à pratiquer et tester des compétences en programmation. *exercise*
- fonction :\
  Bloc de code réutilisable accomplissant une tâche spécifique. *function*
- ramasse-miettes :\
  Mécanisme libérant automatiquement la mémoire occupée par des objets inutilisés. *garbage collector*
- génériques :\
  Fonctionnalité permettant d’écrire du code avec des types paramétrés, pour réutiliser avec différents types. *generics*
- immuable :\
  Ne peut pas être modifié après création. *immutable*
- test d’intégration :\
  Test vérifiant les interactions entre différentes parties d’un système. *integration test*
- mot-clé :\
  Mot réservé du langage avec une signification spécifique, non utilisable comme identifiant. *keyword*
- bibliothèque :\
  Ensemble de routines ou de code précompilé pouvant être utilisé par d’autres programmes. *library*
- macro :\
  Les macros en Rust se reconnaissent par un `!`. Utilisées quand les fonctions normales ne suffisent pas. Exemple : `format!`, qui accepte un nombre variable d’arguments, ce que Rust ne permet pas dans les fonctions classiques. *macro*
- fonction `main` :\
  L’exécution d’un programme Rust commence par la fonction `main`. *main function*
- match :\
  Structure de contrôle permettant le filtrage par motifs sur une valeur. *match*
- fuite mémoire :\
  Situation où la mémoire non utilisée n’est pas libérée, entraînant une croissance continue de l’usage mémoire. *memory leak*
- méthode :\
  Fonction associée à un objet ou un type en Rust. *method*
- module :\
  Espace de nom contenant définitions (fonctions, types, traits…) pour organiser le code Rust. *module*
- déplacement :\
  Transfert de propriété d’une valeur d’une variable vers une autre. *move*
- mutable :\
  Propriété permettant de modifier une variable après déclaration. *mutable*
- propriété :\
  Concept définissant quelle partie du code gère la mémoire d’une valeur. *ownership*
- panique :\
  Erreur irrécupérable entraînant l’arrêt du programme. *panic*
- paramètre :\
  Valeur transmise à une fonction ou méthode lors de son appel. *parameter*
- motif :\
  Combinaison de valeurs, littéraux ou structures pouvant être comparée à une expression. *pattern*
- charge utile :\
  Données transportées par un message, un événement ou une structure. *payload*
- programme :\
  Ensemble d’instructions qu’un ordinateur peut exécuter pour accomplir une tâche. *program*
- langage de programmation :\
  Système formel permettant de communiquer des instructions à un ordinateur, comme Rust. *programming language*
- récepteur :\
  Premier paramètre d’une méthode Rust représentant l’instance appelée. *receiver*
- comptage de références :\
  Technique de gestion mémoire où un objet est libéré quand plus aucune référence ne l’utilise. *reference counting*
- retour :\
  Mot-clé Rust pour indiquer la valeur à retourner depuis une fonction. *return*
- Rust :\
  Langage de programmation système centré sur la sécurité, les performances et la concurrence. *Rust*
- Fondamentaux Rust :\
  Les jours 1 à 4 de ce cours. *Rust Fundamentals*
- Rust sur Android :\
  Voir [Rust sur Android](android.md). *Rust in Android*
- Rust dans Chromium :\
  Voir [Rust dans Chromium](chromium.md). *Rust in Chromium*
- sûr :\
  Code respectant les règles de propriété et d’emprunt de Rust, évitant les erreurs mémoire. *safe*
- portée :\
  Zone du programme où une variable est valide. *scope*
- bibliothèque standard :\
  Ensemble de modules fournissant les fonctionnalités de base en Rust. *standard library*
- statique :\
  Mot-clé définissant des éléments ayant une durée de vie `'static`. *static*
- chaîne de caractères :\
  Type de données textuelles. Voir [Chaînes de caractères](references/strings.html). *string*
- structure :\
  Type composé regroupant des variables de différents types sous un même nom. *struct*
- test :\
  Module Rust contenant des fonctions testant la validité d’autres fonctions. *test*
- thread :\
  Séquence d’exécution parallèle dans un programme. *thread*
- sûreté des threads :\
  Propriété garantissant un comportement correct dans un environnement multi-thread. *thread safety*
- trait :\
  Ensemble de méthodes définies pour un type inconnu, permettant le polymorphisme. *trait*
- contrainte de trait :\
  Abstraction imposant à un type d’implémenter certains traits. *trait bound*
- tuple :\
  Type composé contenant des variables de types différents. Les champs sont anonymes et accédés par indice. *tuple*
- type :\
  Catégorie définissant les opérations applicables à une valeur. *type*
- inférence de type :\
  Capacité du compilateur Rust à déduire le type d’une variable ou d’une expression. *type inference*
- comportement indéfini :\
  Actions sans résultat défini, pouvant entraîner un comportement imprévisible. *undefined behavior*
- union :\
  Type pouvant contenir différentes valeurs, mais une seule à la fois. *union*
- test unitaire :\
  Rust prend en charge les tests unitaires et d’intégration. Voir [Tests unitaires](testing/unit-tests.html). *unit test*
- type unité :\
  Type ne contenant aucune donnée, représenté par un tuple vide. *unit type*
- unsafe :\
  Sous-ensemble de Rust permettant d’outrepasser les règles de sécurité. Voir [Rust unsafe](unsafe-rust/unsafe.md). *unsafe*
- variable :\
  Emplacement mémoire contenant une donnée. Les variables sont valides dans une _portée_. *variable*