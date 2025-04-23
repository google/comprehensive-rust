# Contenu du cours

Les fichiers de ce répertoire constituent le contenu du cours.  
Ils peuvent également inclure du contenu tiers provenant de `../third_party/`.

Lorsque nous publions une traduction du cours, nous utilisons `git restore` sur les répertoires
`src/` et `third_party/` à la racine du dépôt, en les restaurant à la date indiquée dans l’en-tête
POT-Creation-Date de la traduction. **Il est essentiel que tout le contenu traduisible se trouve
dans ces deux répertoires.**  
Les autres fichiers (tels que `book.toml` et `theme/`) ne sont pas restaurés et nous utilisons
toujours leur dernière version.