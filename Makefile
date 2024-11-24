install:
	cargo install mdbook
	cargo install --locked mdbook-svgbob
	cargo install --locked mdbook-i18n-helpers
	cargo install --locked i18n-report
	cargo install --locked mdbook-linkcheck
	cargo install --locked --path mdbook-exerciser
	cargo install --locked --path mdbook-course

test: 
	mdbook test

serve:
	mdbook serve
