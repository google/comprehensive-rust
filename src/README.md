# Course Content

The files in this directory make up the content of the course. The files here
can include third-party content from `../third_party/` as well.

When we publish a translation of the course, we `git restore` the `src/` and
`third_party/` directories at the repository root back to the date listed in the
POT-Creation-Date header of the translation. **It is crucial, that all
translatable content lives in those two directories.** The other files (such as
`book.toml` and `theme/`) are not restored and we always use the latest version
of them.
