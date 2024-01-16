# mdbook-course

This is an mdBook preprocessor to handle some specific details of Comprehensive
Rust.

## Frontmatter

The preprocessor parses "frontmatter" -- YAML between `---` at the beginning of
a Markdown file -- and removes it from the rendered result.

Frontmatter is optional, and can contain any of the following fields, defined
below:

```yaml
minutes: NNN
target_minutes: NNN
course: COURSE NAME
session: SESSION NAME
```

## Course Structure

A book can contain multiple _courses_. Each course is made up of _sessions_,
which are blocks of instructional time (and include breaks). Typically two
sessions are taught per day, morning and afternoon.

Each session is comprised of _segments_, which are slides on a related theme.
Breaks are scheduled between segments.

Each segment is comprised of _slides_. A slide can be made up of one or more
mdBook chapters.

The course structure is derived from the mdBook structure. Each top-level mdBook
"section" is treated as a segment, and may optionally begin a new session or
course. Within each section, the first chapter and subsequent second-level
chapters are each treated as a slide. Any further-nested chapters are treated as
parts of the parent slide. For example:

```ignore
- [Frobnication](frobnication.md)
  - [Integer Frobnication](frobnication/integers.md)
  - [Frob Expansion](frobnication/expansion.md)
    - [Structs](frobnication/expansion-structs.md)
    - [Enums](frobnication/expansion-structs.md)
  - [Exercise](frobnication/exercise.md)
    - [Solution](frobnication/Solution.md)
```

In this segment, there are four slides: "Frobnication", "Integer Frobnication",
"Frob Expansion", and "Exercise". The last two slides are made up of multiple
chapters.

The first chapter of a segment can use the `course` and `session` fields in its
frontmatter to indicate that it is the first segment in a session or course.

## Timing

Each chapter should specify an estimate of the instructional time it will
require in the `minutes` field. This information is summed, with breaks
automatically added between segments, to give time estimates for segments,
sessions, and courses.

Each session should list a `target_minutes` that is the target duration of the
session.

## Directives

Within the course material, the following directives can be used:

```
{{%segment outline}}
{{%session outline}}
{{%course outline}}
{{%course outline COURSENAME}}
```

These will be replaced with a markdown outline of the current segment, session,
or course. The last directive can refer to another course by name and is used in
the "Running the Course" section.

# Course-Schedule Comments

The `course-schedule` binary generates Markdown output that is included in a
GitHub pull request comment, based on the information provided in the above
format.
