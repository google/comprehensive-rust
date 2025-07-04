# Translations of Comprehensive Rust 🦀

We would love to have your help with translating the course into other
languages! Please see the [translations page] for the existing translations.

[translations page]: https://google.github.io/comprehensive-rust/running-the-course/translations.html

We use the [Gettext] system for translations. This means that you don't modify
the Markdown files directly: instead you modify `.po` files in a `po/`
directory. The `.po` files are small text-based translation databases.

> **Tip:** You should not edit the `.po` files by hand. Instead use a PO editor,
> such as [Poedit](https://poedit.net/). There are also several online editors
> available. This will ensure that the file is encoded correctly.

> **Important:** You need to run `dprint fmt` after editing the PO file. This
> ensures consistent formatting of the file. You need to install the Gettext
> tools for this, see the Preparation section below.

There is a `.po` file for each language. They are named after the [ISO 639]
language codes: Danish would go into `po/da.po`, Korean would go into
`po/ko.po`, etc. The `.po` files contain all the English text plus the
translations. They are initialized from a `messages.pot` file (a PO template)
which contains only the English text.

We will show how to update and manipulate the `.po` and `.pot` files using the
GNU Gettext utilities below.

[Gettext]: https://www.gnu.org/software/gettext/manual/html_node/index.html
[ISO 639]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes

## Preparation

Please make sure you can [build the course](README.md#building). You will also
need the `msgmerge` and `msgcat` Gettext tool installed. Please see our
[contribution guide](CONTRIBUTING.md#formatting) for details.

## Creating and Updating Translations

First, you need to know how to update the `.pot` and `.po` files.

You should never touch the auto-generated `book/xgettext/messages.pot` file. You
should also never edit the `msgid` entries in a `po/xx.po` file. If you find
mistakes, you need to update the original English text instead. The fixes to the
English text will flow into the `.po` files the next time the translators update
them.

> **Tip:** See our [style guide](STYLE.md) for some things to keep in mind when
> writing the translation.

### Generating the PO Template

To extract the original English text and generate a `messages.pot` file, you
build the book. This will automatically invoke the `mdbook-xgettext` renderer:

```shell
mdbook build
```

You will find the generated POT file as `book/xgettext/messages.pot`.

### Initialize a New Translation

To start a new translation, first generate the `book/xgettext/messages.pot`
file. Then use `msginit` to create a `xx.po` file for the fictional `xx`
language:

```shell
msginit -i book/xgettext/messages.pot -l xx -o po/xx.po
```

You can also simply copy `book/xgettext/messages.pot` to `po/xx.po`. Then update
the file header (the first entry with `msgid ""`) to the correct language.

> **Tip:** You can use the
> [`cloud-translate`](https://github.com/mgeisler/cloud-translate) tool to
> quickly machine-translate a new translation. Install it with
>
> ```shell
> cargo install cloud-translate
> ```
>
> Untranslated entries will be sent through GCP Cloud Translate. Some of the
> translations will be wrong after this, so you must inspect them by hand
> afterwards.

Next, please update the file `.github/labeler.yml` to include the new language:

```diff
+"translation/xx":
+  - changed-files:
+      - any-glob-to-any-file: po/xx.po
```

### Refreshing an Existing Translation

As the English text changes, translations gradually become outdated. The
translations contain a POT-Creation-Date header which tells you when they were
last updated with new English messages.

To update the `po/xx.po` file with new messages, first extract the English text
into a `book/xgettext/messages.pot` template file. Then run

```shell
msgmerge --update po/xx.po book/xgettext/messages.pot
```

Notice that the POT-Creation-Date field is updated to the current time and date.
This becomes the new baseline for the translation: new English text added
afterwards will not show up in your translation, including completely new pages.

When running `msgmerge`, unchanged messages stay intact, deleted messages are
marked as old, and updated messages are marked "fuzzy". A fuzzy entry is not
used when we publish a translation! You have to go over the fuzzy entries by
hand and verify that the translation is correct the fuzzy marker.

> **Note:** Your PRs should either be the result of running `msgmerge` or the
> result of new translation work on the PO file for your language. Avoid mixing
> the two since it often creates a very large diff, which is hard or impossible
> to review.

### Editing a Translation

You should install a PO editor to edit the `.po` file for your language. The
files are simple text files, but it helps to use a dedicated editor since it
will take care of escaping things like `"` correctly.

There are many PO editors available. [Poedit](https://poedit.net/) is a popular
cross-platform choice, but you can also find several online editors.

### Formatting a Translation

If the file is not formatted correct, you will get an error on the PR. Make sure
to follow the [steps](#preparation) to install [Gettext] and
[`dprint`](https://dprint.dev/) and then run:

```shell
dprint fmt po/xx.po
```

This will automatically format the `.po` file for you. Commit the formatting fix
and push to your branch. Your PR should now be error free.

## Using Translations

This will show you how to use the translations to generate localized HTML
output.

> **Note:** `mdbook` will use original untranslated entries for all entries
> marked as "fuzzy" (visible as "Needs work" in Poedit). This is especially
> important when using
> [`cloud-translate`](https://github.com/mgeisler/cloud-translate) for initial
> translation as all entries will be marked as "fuzzy".

### Building a Translation

Make sure you have gone through the [build setup](./README.md#building) at least
once.

To use the `po/xx.po` file for your output, run the following command:

```shell
MDBOOK_BOOK__LANGUAGE=xx mdbook build -d book/xx
```

This will tell the `mdbook-gettext` preprocessor to translate the book using the
`po/xx.po` file. The HTML output can be found in `book/xx/html/`.

### Serving a Translation

Like normal, you can use `mdbook serve` to view your translation as you work on
it. You use the same command as with `mdbook build` above:

```shell
MDBOOK_BOOK__LANGUAGE=xx mdbook serve -d book/xx
```

When you update the `po/xx.po` file, the translated book will automatically
reload.

## Reviewing Translations

When a new translation is started, we look for people who can help review it.
These reviewers are often Googlers, but they don't have to be. To automatically
get an email when new PRs are created for your language, please add yourself to
the [CODEOWNERS] file.

When reviewing a translation, please keep in mind that translations are a labour
of love. Someone spends their free time translating the course because they want
to bring Rust to users who speak their language.

Nothing is published right away after a PR lands for a new in-progress language.
It is therefore safe to merge the PR as long as the translation is reasonable.
This is often better than leaving 50+ comments since this can be overwhelming
for the contributor. Instead, please work with the contributor to improve things
in follow-up PRs.

### GitHub Suggestions

When reviewing a translation PR, please use the
[GitHub suggestion feature](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/reviewing-changes-in-pull-requests/commenting-on-a-pull-request).
This feature allows you to directly write how you think a line or paragraph
should be phrased. Use the left-most button in the toolbar to create a
suggestion.

The PR author can
[apply the changes with a single click](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/reviewing-changes-in-pull-requests/incorporating-feedback-in-your-pull-request)
afterwards, drastically reducing the number of round-trips needed in a review.

### Incomplete Translations

When the first 1-2 days of the course have been translated, we can publish the
translation and link it from the [translations page]. The idea is to celebrate
the hard work, even if it is incomplete.

[CODEOWNERS]: https://github.com/google/comprehensive-rust/blob/main/.github/CODEOWNERS

## Publication Workflow

> This section is for the developers of Comprehensive Rust, but it might give
> you valuable background information on how the translations are published.

When a change is made to the `main` branch, the [`publish.yml`] GitHub CI
workflow starts.

The `publish` job in this workflow will:

- Install dependencies as described in [`CONTRIBUTING`](CONTRIBUTING.md).

- Build every translation of the course, including the original English, using
  [`build.sh`]. The English HTML ends up in `book/html/`, the HTML for the `xx`
  language ends up in `book/xx/html/`.

- Publish the entire `book/html/` directory to
  https://google.github.io/comprehensive-rust/.

[`build.sh`]: https://github.com/google/comprehensive-rust/blob/main/.github/workflows/build.sh

### `build.sh`

The `build.sh` script is used both when testing code from a PR (with
[`build.yml`]) and when publishing the finished book (with [`publish.yml`]).

[`build.yml`]: https://github.com/google/comprehensive-rust/blob/main/.github/workflows/build.yml
[`publish.yml`]: https://github.com/google/comprehensive-rust/blob/main/.github/workflows/publish.yml

The job of the script is to call `mdbook build`, but with a few extra steps:

- It will enable the PDF output using `mdbook-pandoc`. This is disabled by
  default to make it easier for people to run `mdbook build` without having to
  configure LaTeX.

#### Restoring Translations

When building a translation (languages other than English), `build.sh` will
restore all Markdown files to how they looked at the time recorded in the
POT-Creation-Date header.

This means that:

- A translation does not degrade when the English text is changed.
- A translation will not received the latest fixes to the English text.

The script restores the Markdown with a simple

```sh
$ git restore --source $LAST_COMMIT src/ third_party/
```

command, where `$LAST_COMMIT` is the commit at the time of the POT-Creation-Date
header.

A consequence of this is that we use the latest theme, CSS, JavaScript, etc for
each translation.

After `build.sh` was run, the working copy is left in this dirty state. Beware
of this if you want to build the English version next, as you will have to clean
up manually.

## Status reports

Two translation status reports are automatically generated:

- [Translation status as checked in][translation-report]
- [Translation status after syncing to the latest version of the source with msgmerge][synced-translation-report]

You can also generate this report locally to see the effect of your local
changes:

```shell
i18n-report translation-report.html po/*.po
```

[translation-report]: https://google.github.io/comprehensive-rust/translation-report.html
[synced-translation-report]: https://google.github.io/comprehensive-rust/synced-translation-report.html
