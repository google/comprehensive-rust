rm -rf rendered
mkdir -p rendered
for lang in ko da de; do
  MDBOOK_BOOK__LANGUAGE=$lang-new MDBOOK_OUTPUT='{"markdown": {}}' mdbook build -d rendered/$lang-new
  diff -u --color -r rendered/$lang-new expected/$lang-expected
done
