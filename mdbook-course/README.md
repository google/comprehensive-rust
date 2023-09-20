# mdbook-course

This is an mdBook preprocessor to handle some specific details of Comprehensive
Rust.

## Frontmatter

The preprocessor parses "frontmatter" -- YAML between `---` at the beginning of
a Markdown file -- and removes it from the rendered result. At the moment, to
aid review of the new course, it places this content in a `<pre>` block.

## Future Work

- Parse the `minutes` property from frontmatter and
  - Generate a course timeline
  - Include timing information in the speaker notes
- Generate per-segment tables of contents.
