# Translations of Comprehensive Rust 🦀

We would love to have your help with translating the course into other
languages! We use the [Gettext] system for translations. This means that you
don't modify the Markdown files directly: instead you modify `.po` files in a
`po/` directory. The `.po` files are small text-based translation databases.

> **Tip:** You should not edit the `.po` files by hand. Instead use a PO
> editor, such as [Poedit](https://poedit.net/). There are also several online
> editors available. This will ensure that the file is encoded correctly.

There is a `.po` file for each language. They are named after the [ISO 639]
language codes: Danish would go into `po/da.po`, Korean would go into
`po/ko.po`, etc. The `.po` files contain all the English text plus the
translations. They are initialized from a `messages.pot` file (a PO template)
which contains only the English text.

We will show how to update and manipulate the `.po` and `.pot` files using the
GNU Gettext utilities below.

[Gettext]: https://www.gnu.org/software/gettext/manual/html_node/index.html
[ISO 639]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes

## I18n Helpers

We use two helpers for the translations:

* `mdbook-xgettext`: This program extracts the English text. It is an mdbook
  renderer.
* `mdbook-gettext`: This program translates the book into a target language. It
  is an mdbook preprocessor.

Install both helpers with the following command from the root of the course:

```shell
$ cargo install --path i18n-helpers
```

## Creating and Updating Translations

First, you need to know how to update the `.pot` and `.po` files.

As a general rule, you should never touch the auto-generated `po/messages.pot`
file. You should also not edit the `msgid` entries in a `po/xx.po` file. If you
find mistakes, you need to update the original English text instead. The fixes
to the English text will flow into the `.po` files the next time the translators
update them.

### Generating the PO Template

To extract the original English text and generate a `messages.pot` file, you run
`mdbook` with a special renderer:

```shell
$ MDBOOK_OUTPUT='{"xgettext": {"pot-file": "messages.pot"}}' \
  mdbook build -d po
```

You will find the generated POT file as `po/messages.pot`.

### Initialize a New Translation

To start a new translation, first generate the `po/messages.pot` file. Then use
`msginit` to create a `xx.po` file for the fictional `xx` language:

```shell
$ msginit -i po/messages.pot -l xx -o po/xx.po
```

You can also simply copy `po/messages.pot` to `po/xx.po`. Then update the file
header (the first entry with `msgid ""`) to the correct language.

### Updating an Existing Translation

As the English text changes, translations gradually become outdated. To update
the `po/xx.po` file with new messages, first extract the English text into a
`po/messages.pot` template file. Then run

```shell
$ msgmerge --update po/xx.po po/messages.pot
```

Unchanged messages will stay intact, deleted messages are marked as old, and
updated messages are marked "fuzzy". A fuzzy entry will reuse the previous
translation: you should then go over it and update it as necessary before you
remove the fuzzy marker.

## Using Translations

This will show you how to use the translations to generate localized HTML
output.

## Building a Translation

To use the `po/xx.po` file for your output, run the following command:

```shell
$ MDBOOK_BOOK__LANGUAGE=xx mdbook build -d book/xx
```

This will update the book's language to `xx`, it will make the `mdbook-gettext`
preprocessor become active and tell it to use the `po/xx.po` file, and finally
it will redirect the output to `book/xx`.

## Serving a Translation

Like normal, you can use `mdbook serve` to view your translation as you work on
it. You use the same command as with `mdbook build` above:

```shell
$ MDBOOK_BOOK__LANGUAGE=xx mdbook serve -d book/xx
```
