# Comprehensive Rust 🦀 Style Guide

The course has been expanded and improved by tons of volunteers like you! Thank
you for that! To help ensure a consistent style throughout the course, we have
written down some guidelines for you to follow.

## Course Philosophy and Design

To contribute effectively, it's helpful to understand the core design principles
of Comprehensive Rust. This is not a self-study book; it is a set of slides and
notes for an **instructor-led course**.

### Target Audience

The course is designed for an audience of experienced software engineers who are
new to Rust. We assume they have 2-3 years of experience in an imperative
language like C, C++11+, Java 7+, Python, or Go.

We **do not** assume familiarity with functional programming concepts or
features from more modern languages like Swift or Kotlin. Course material should
build upon the concepts that are likely to be familiar to this audience.

### Goals

The goal of the course is to provide a solid foundation in Rust within a bounded
time frame. This prepares students to continue learning effectively as they
begin to apply their new skills on the job.

### Pedagogical Principles

We follow a few key principles to make the material effective for learning:

- **Build on a Foundation:** New Rust concepts should be connected to what a
  learner already knows, either from their prior language experience or from
  earlier parts of this course.
- **Provide a Working Mental Model (The "No Magic" Rule):** As much as possible,
  avoid telling students to accept syntax or behavior that will be explained
  later. For everything that appears on the slides or in exercises, we must
  provide a working mental model that allows the student to understand and use
  the concept.
- **Use a [Spiral Approach](https://en.wikipedia.org/wiki/Spiral_approach):** To
  avoid overwhelming the learner, it is highly encouraged to introduce a concept
  by first providing basic facts and a simplified mental model. The topic can
  then be revisited later to provide more detail. For example, very early in the
  course we explain the basics of `println!`, mention that it is a macro so the
  usage syntax is a bit unusual, but we don't go into details of format strings
  or macros. We explain details of format strings later, once we have covered
  traits and can mention the `Debug` and `Display` traits.
- **Live, Interactive Instruction:** The instructor is expected to run and
  modify the code on the slides, and use compiler errors as a teaching tool. The
  audience is expected to frequently interrupt with questions, and the
  instructor would often experiment with the code on the slide to illustrate the
  answer.

### Pacing and Structure

The course is designed for approximately 5 hours of teaching per day, typically
split into a 3-hour morning session and a 2-hour afternoon session.

This pacing is important context for contributors. Material should be structured
to fit this rhythm, with clear sections that can be taught in roughly 45-50
minute blocks to accommodate short breaks and Q&A.

Each slide must include a `minutes` field in its frontmatter, which specifies
the estimated teaching time for that slide. This helps ensure the overall pacing
of the course remains consistent.

### Course Structure

The course starts with a core **Rust Fundamentals** curriculum, followed by a
collection of specialized **deep dives**. All students take the Fundamentals
course and can then opt into any deep dives that are relevant to them.

#### The Rust Fundamentals Course

The **Rust Fundamentals** course provides a solid foundation in a strictly
bounded, four-day time frame. This duration is firm, and its scope is carefully
managed to focus on the most essential concepts for new Rust programmers.

The overall progression of the course starts with the parts of the Rust language
that should be conceptually familiar to most students from other languages. Then
we move on to more difficult parts (for example, enums with payloads and
generics), and parts that are unique to Rust (lifetimes and the borrow checker).

Contributors should keep this structure in mind. The four-day schedule for the
Fundamentals course is completely full, leaving no time slack for new topics.
Proposals to add material to the Rust Fundamentals course must also include a
plan to shorten or remove existing content. Refinements to existing topics are
always welcome. Topics that are not essential for all new Rust programmers
should be proposed as new deep dives.

#### Deep Dives

Specialized material can be added to _deep dives_. These cover things like Rust
in Android, Bare-Metal Rust, etc., which are not necessarily something every
Rust developer should know. More deep dives can be added in the future.

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

### One Core Idea Per Slide

Ideally, each slide should focus on a single, clear takeaway. If a slide
introduces a core concept and then explores an important but distinct tangent
(e.g., a limitation or an advanced use case), that tangent should be moved to
its own slide. This keeps the presentation focused and easier to follow.

Consider the instructor's workflow. If the speaker notes require a long or
complex series of live edits, it can be difficult for the instructor to execute
well every time. It may be better to add a new slide that presents the desired
state of the code.

### Pedagogical Flow

When introducing a new concept, start with a simple, relatable, and concrete
example. A good opening example grounds the concept for the learner and provides
motivation for the more detailed explanation that will follow.

### Use Meaningful Examples

Code samples on the slides should be short and do something meaningful. Avoid
using generic placeholders like `Foo`, `Bar`, and `Baz`. Using descriptive names
from a real-world, even if simplified, domain makes the code easier to
understand and relate to.

### Plan Interactive Code Snippets

All Rust code blocks in the course are not static text but are live, editable
playgrounds. An important teaching method is for the instructor to edit these
snippets live to demonstrate concepts, introduce and fix errors, and explore
variations based on student questions.

Contributors should design their slides with this interactivity in mind. The
initial state of the code should be a good starting point for a live
demonstration.

### `mdbook` and `mdbook-course` Conventions

The project uses `mdbook` features in specific ways, as well as a custom
preprocessor, `mdbook-course`. The following conventions are mandatory:

- **YAML Frontmatter:** Every slide file **must** include YAML frontmatter at
  the top. At a minimum, this must include the `minutes` field to specify the
  estimated teaching time.
- **Outline Helpers:** Pages that serve as an index for a session or segment
  **must** use the `{{%session outline%}}` or `{{%segment outline%}}` helpers.
- **File Includes:** Code for exercises and their solutions **must** be included
  from external files using the standard `mdbook` `{{#include ...}}` helper.
- **Translation Directives:** To prevent an element (such as a paragraph, code
  block, or list item) from being translated, place a
  `<!-- mdbook-xgettext: skip -->` comment on a line by itself, followed by a
  blank line, immediately before the element.

For a complete explanation of the custom helpers and all available frontmatter
fields, please refer to the [`mdbook-course` README](mdbook-course/README.md).

### Language and Tone

The courses are written in American English, so write "initialize", not
"initialise".

Use an informal, friendly, and concise tone. Remember that the courses are meant
to be taught by an experienced programmer to other experienced programmers. When
possible, prefer terminology used in
[the official Rust Book](https://doc.rust-lang.org/book/). If a less common but
necessary term is used, provide a brief definition.

#### Glossary

The `src/glossary.md` file contains definitions for key Rust terms used
throughout the course. When editing course content, use the glossary to anchor
concepts and ensure consistency in terminology. Terms should be defined and used
consistently with their glossary entries.

## Exercises

At the end of some sections, learners will actively engage with the material by
completing a small exercise. The goal of an exercise is to provide hands-on
practice with the concepts just taught.

Please keep the following principles in mind when creating or updating
exercises:

- **Focused Scope:** An exercise should focus on the topic of the preceding
  section. It should not require knowledge of concepts that have not yet been
  taught.
- **Short Duration:** An exercise should be solvable by the target audience in
  approximately 10-15 minutes. The goal is a quick, successful application of
  knowledge, not a complex project.
- **Clear Instructions:** The problem description should be clear and
  unambiguous.

## Speaker Notes

We have extended `mdbook` with support for speaker notes: content added between
`<details> ... </details>` tags is rendered in a special box that can be
collapsed or removed entirely from the slide.

- Speaker notes suggest a narrative structure for the instructor.

- The speaker notes should expand on the topic of the slide. Use them to provide
  interesting background information for both the instructor and for students
  who look at the material outside of a class. Remember that many more people
  will read the course by themselves, so make the notes complete and useful even
  when there is no Rust expert around.

- For slides with evolving code examples, the notes provide a clear,
  step-by-step flow for how the code is modified and presented. This is a
  suggested flow for the instructor's live-coding session within the slide's
  interactive playground. This includes:

  - The order in which to introduce concepts, how to motivate them.

  - Framing of the code example: the problem it tries to solve, if not obvious.

  - How to demonstrate variations of the code example (e.g., code that does not
    compile or illustrates a bug).

  - How to change the code on the slide to illustrate the concepts being taught.

  - Where to pause and engage the class with questions.

- Speaker notes should serve as a quick reference for instructors, not a
  verbatim script. Because instructors have limited time to glance at notes, the
  content should be concise and easy to scan.

  **Avoid** long, narrative paragraphs meant to be read aloud:
  > **Bad:** _"In this example, we define a trait named `StrExt`. This trait has
  > a single method, `is_palindrome`, which takes a `&self` receiver and returns
  > a boolean value indicating if the string is the same forwards and
  > backwards..."_

  **Instead, prefer** bullet points with background information or actionable
  **teaching prompts**:
  > **Good:**
  >
  > - Note: The `Ext` suffix is a common convention.
  > - Ask: What happens if the `use` statement is removed?
  > - Demo: Comment out the `use` statement to show the compiler error.

- Nevertheless, include all of the necessary teaching prompts for the instructor
  in the speaker notes. Unlike the main content, the speaker notes don't have to
  fit on a single slide.

### More to Explore

Use the "More to Explore" section for valuable topics that are outside the main
scope of the class. The content should be placed within the `<details>` block as
shown below:

```markdown
<details>

...

## More to Explore

...

</details>
```

This section can contain a deeper explanation of a concept or provide specific
pointers to external resources. A link should be accompanied by a brief
explanation of what the resource contains and why it is relevant. A vague
reference is not helpful, but a specific one can be a great tool for
self-learners.

## Code Blocks Mechanics

Code blocks are a critical part of the course. To ensure they are consistent and
behave as expected, please follow these conventions.

### Language Identifiers

Use the following language identifiers for fenced code blocks:

- **`rust`**: For Rust code examples.
- **`shell`**: For shell commands. Use a `$` prompt for consistency. Omit the
  prompt for multi-line commands or when the output is shown.
- **`bob`**: For ASCII art diagrams generated by `mdbook-bob`.
- **`ignore`**: For code snippets that are not complete, self-contained programs
  or are for illustrative purposes only and should not be compiled.

### mdbook Annotations

You can add annotations to Rust code blocks to control how they are tested and
displayed:

- **`editable`**: Makes the code block an interactive playground where users can
  edit and run the code. This should be used for most Rust examples.
- **`compile_fail`**: Indicates that the code is expected to fail compilation.
  This is used to demonstrate specific compiler errors.
- **`should_panic`**: Indicates that the code is expected to panic when run.
- **`warnunused`**: Re-enables `unused` lints for a code block. By default, the
  course's test runner disables lints for unused variables, imports, etc., to
  avoid distracting warnings. Use this annotation only when a warning is part of
  the lesson.

### Rust Code Formatting

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
