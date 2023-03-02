rm -rf rendered
mkdir -p rendered
for lang in ko; do
  git show 60aa747aca13b5672084b03ddb50733a7fd01ee8:po/$lang.po > po/$lang-orig.po
  MDBOOK_PREPROCESSOR__GETTEXT__PO_FILE=po/$lang-orig.po MDBOOK_OUTPUT='{"markdown": {}}' mdbook build -d rendered/$lang-expected
  MDBOOK_PREPROCESSOR__GETTEXT__PO_FILE=po/$lang-new.po MDBOOK_OUTPUT='{"markdown": {}}' mdbook build -d rendered/$lang-new
  diff --color -r rendered/$lang-{expected,new}
done
