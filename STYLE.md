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

You can test the amount of space available using a simple tool. This tool can be
used by clicking a toggle button next to the search button on left side of the
navbar.

The rectangle has an aspect ratio similar to what you can see when you share
your screen on a 16:9 display or projector.

Use the rectangle as a rough guide for how much you can fit on a single slide.
If you find yourself adding too much detail, move the details to the speaker
notes (see below).

### Rust Code

When showing Rust code inline, please use the same spacing as `rustfmt`: `3 * x`
instead of `3*x`. However, feel free to remove newlines when it can make the
code more compact and easier to understand, e.g., you can define a struct on one
line if it is not the focus of your example:

<!-- dprint-ignore-start -->

```rust
struct Person { name: String }
```

<!-- dprint-ignore-end -->

Enclose the code block in `<!-- dprint-ignore-start -->` and
`<!-- dprint-ignore-end -->` to suppress the automatic formatting. Please use
this sparingly.

### Language and Tone

The courses are written in American English, so write "initialize", not
"initialise".

Use an informal, friendly, and concise tone. Remember that the courses are meant
to be taught by an experienced programmer to other experienced programmers. We
expect familiarity with programming, typically in a statically typed language
like Java or C++. We don't explain common concepts known from that family of
languages, but we cannot assume familiarity with things like functional
programming.

## Speaker Notes

We have extended `mdbook` with support for speaker notes: content added between
`<details> ... </details>` tags is rendered in a special box that can be
collapsed or removed entirely from the slide.

- Unlike the main content, the speaker notes don't have to fit on a single
  slide.

- The speaker notes should expand on the topic of the slide. Use them to provide
  interesting background information for both the instructor and for students
  who look at the material outside of a class. Remember that many more people
  will read the course by themselves, so make the notes complete and useful even
  when there is no Rust expert around.

- Speaker notes are not a script for the instructor. When teaching the course,
  instructors only have a short time to glance at the notes. Don't include full
  paragraphs for the instructor to read out loud.

### More to Explore

Move extended explanations in the notes to a "More to Explore" section:

```markdown
<details>

...

## More to Explore

...

</details>
```

The material there is outside the scope of the regular class.

## Translations

This section is about what you write in the translation. We describe
[how to create or update translations elsewhere](TRANSLATIONS.md).

When translating the course, please take the following into account:

- Do not translate:
  - The course name ("Comprehensive Rust 🦀"). If the name is not easily
    understood in your language, please add the translated version after the
    original name.
  - Variable names (you _should_ translate the comments, though.)

- If the Rust Book has been
  [translated into your language](https://doc.rust-lang.org/book/appendix-06-translation.html),
  please use the same vocabulary.

- The text you write is in Markdown format. Make sure to preserve the original
  formatting in the translation by marking text as `` `code` ``, `_emphasis_`
  and `**strong emphasis**` like in the original.

- If you find mistakes or things that sound awkward in the original English
  text, please submit PRs to fix them in the English text! Fixing typos in the
  translation is great, but we want everybody to benefit from the fixes and that
  is why we need the fix to be made in the English text too.
