# Comprehensive Rust 🦀 Style Guide

The course has been expanded and improved by tons of volunteers like you! Thank
you for that! To help ensure a consistent style throughout the course, we have
written down some guidelines for you to follow.

## Course Slides

Please take the following into account when updating the course material.

### Vertical Space

What looks like pages in a browser, are actually slides in a presentation. It is
important to keep this in mind when adding content: we only have limited
vertical space. Scrolling up and down should be avoided since it is very jarring
for people who attend the class.

You can test the amount of space available using a simple tool. Uncomment these
lines in the `book.toml` file to have a red rectangle rendered on top of all
pages:

```toml
[preprocessor.aspect-ratio-helper]
command = "./aspect-ratio-helper.py"
```

The rectangle has an aspect ratio similar to what you can see when you share
your screen on a 16:9 display or projector.

Use the rectangle as a rough guide for how much you can fit on a single slide.
If you find yourself adding too much detail, move the details to the speaker
notes (see below).

### Rust Code

When showing Rust code, please use the same spacing as `rustfmt`: `3 * x`
instead of `3*x`. However, feel free to remove newlines when it can make the
code more compact and easier to understand, e.g., you can use

<!-- dprint-ignore-start -->

```rust
struct Person { name: String }
```

<!-- dprint-ignore-end -->

if the `Person` struct is not important for your example. Please use this
sparingly: enclose the code block in `<!-- dprint-ignore-start -->` and
`<!-- dprint-ignore-end -->` to suppress warnings about the formatting.

## Speaker Notes

We have extended `mdbook` with support for speaker notes: content added between
`<details> ... </details>` tags is rendered in a special box that can be
collapsed or removed entirely from the slide.

- The speaker notes should expand on the topic of the slide. Use them to provide
  interesting background information for both the instructor and for students
  who look at the material outside of a class. Remember that many more people
  will read the course by themselves, so make the notes complete and useful even
  when there is no Rust expert around.

- Avoid using speaker notes as a script for the instructor. When teaching the
  course, instructors will only have time to glance at the notes so it is not
  useful to include full paragraphs which the instructor should read out loud.

## Translations

This section is about what you write in the translation. We describe
[how to create or update translations elsewhere](TRANSLATIONS.md).

When translating the course, please take the following into account:

- Do not translate the course name ("Comprehensive Rust 🦀"). If the name is not
  easily understood in your language, please add the translated version after
  the original name.

- If the Rust Book has been
  [translated into your language](https://doc.rust-lang.org/book/appendix-06-translation.html),
  please use the same vocabulary.

- Be careful to preserve the Markdown syntax of the original text. Pay special
  attention to reference links in all their variations: `[foo][bar]`, `[foo][]`
  (which means `[foo][foo]`), and `[foo]` (which also means `[foo][foo]`).

  As an example, if you translate `[foo]`, to `[FOO]`, you must also update the
  corresponding link definition from `[foo]: https://example.net` to
  `[FOO]: https://example.net`. If you forget to do this, you end up with a
  broken link in the translation.

- If you find mistakes or things that sound awkward in the original English
  text, please submit PRs to fix them! Fixing typos in the translation is great,
  but we want everybody to benefit from the fixes and that is why we need the
  fix to be made in the English text too.
