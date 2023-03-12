// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use mdbook::utils::new_cmark_parser;
use pulldown_cmark::{Event, Tag};
use std::ops::Range;

/// A translatable message.
#[derive(PartialEq, Debug)]
pub struct Message {
    /// Line number where this message begins.
    line: usize,

    /// Span of the input text containing this message.
    span: Range<usize>,
}

impl Message {
    fn new(line: usize, span: Range<usize>) -> Self {
        Self { line, span }
    }

    /// Get the text of this message, as a slice of the document from which it was generated.
    pub fn text<'doc>(&self, document: &'doc str) -> &'doc str {
        &document[self.span.clone()]
    }

    /// Get the line number at which this message begins.
    pub fn line_number(&self) -> usize {
        self.line
    }

    /// Get the span of the source document from which this message is drawn.
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    /// Extend this message to the given offset.
    fn extend(&mut self, to_end: usize) {
        self.span.end = to_end;
    }

    /// Trim trailing newlines from this message.
    fn trim_right(&mut self, document: &str) {
        let trimmed_len = document[self.span.clone()].trim_end_matches('\n').len();
        self.span.end = self.span.start + trimmed_len;
    }
}

/// Accumulator for translatable messages based on input from the Markdown parser.
struct MsgAccumulator<'a> {
    /// The input document.
    document: &'a str,

    /// Offsets of each newline in the input, used to calculate line numbers from byte offsets.
    offsets: Vec<usize>,

    /// The resulting messages, as ranges of the input document.
    msgs: Vec<Message>,

    /// Current nesting depth of Start/End events.
    depth: usize,

    /// If set, skip until the nesting depth returns to this level.
    skip_until_depth: Option<usize>,

    /// True if the last message can still be appended to. If this is true then
    /// msgs has at least one element.
    message_open: bool,
}

impl<'a> MsgAccumulator<'a> {
    fn new(document: &'a str) -> Self {
        Self {
            document: document,
            offsets: document
                .match_indices("\n")
                .map(|(offset, _)| offset)
                .collect(),
            msgs: vec![],
            depth: 0,
            skip_until_depth: None,
            message_open: false,
        }
    }

    /// Mark the current message as finished.
    fn finish_message(&mut self) {
        self.message_open = false;
    }

    /// Add a new text message, or extend an existing one.
    fn push_message(&mut self, span: Range<usize>) {
        // try to combine with an existing message.
        if self.message_open {
            if let Some(last) = self.msgs.last_mut() {
                last.extend(span.end);
                return;
            }
        }

        self.msgs
            .push(Message::new(self.line_number(span.start), span));
        self.message_open = true;
    }

    /// Calculate the line number for the given offset.
    fn line_number(&self, offset: usize) -> usize {
        self.offsets.partition_point(|&o| o < offset) + 1
    }

    /// Push a new Markdown event into the accumulator.
    fn push_event(&mut self, evt: Event<'a>, span: Range<usize>) {
        #[cfg(test)]
        println!("{evt:?} -- {:?}", &self.document[span.start..span.end]);

        // Track the nesting depth.
        match evt {
            Event::Start(_) => self.depth += 1,
            Event::End(_) => self.depth -= 1,
            _ => {}
        }

        // Handle skip_until_depth, including skipping the End event that
        // returned to the desired level.
        if let Some(depth) = self.skip_until_depth {
            if self.depth <= depth {
                self.skip_until_depth = None;
            }
            return;
        }

        match evt {
            // Consider "inline" tags to be just part of the text.
            Event::Start(
                Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link(..),
            ) => self.push_message(span),
            Event::End(
                Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link(..),
            ) => self.push_message(span),

            // We want to translate everything: text, code (from backticks, `..`), or HTML.
            Event::Text(_) | Event::Code(_) | Event::Html(_) => self.push_message(span),

            // For many event types we just take the entire text from Start to End, which is
            // already encompassed in the event span.
            Event::Start(
                Tag::CodeBlock(_)
                | Tag::Heading(..)
                | Tag::List(..)
                | Tag::BlockQuote
                | Tag::Table(..),
            ) => {
                self.finish_message();
                self.push_message(span);
                self.finish_message();
                // Skip until we get to a nesting depth outside of this Start event.
                self.skip_until_depth = Some(self.depth - 1);
            }

            // For any other Start or End events, finish the current message but do
            // not begin a new one.
            Event::Start(_) | Event::End(_) => self.finish_message(),

            _ => {}
        }
    }

    /// Get the resulting list of messages.
    fn into_msgs(mut self) -> Vec<Message> {
        let parser = new_cmark_parser(self.document, false);
        for (evt, span) in parser.into_offset_iter() {
            self.push_event(evt, span);
        }
        for msg in &mut self.msgs {
            msg.trim_right(self.document);
        }
        self.msgs
    }
}

/// Extract translatable messages from the markdown text.
///
/// Returns a vector of (line number, text), where line numbers begin at 1.
pub fn extract_msgs(document: &str) -> Vec<Message> {
    MsgAccumulator::new(document).into_msgs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_to_line_empty() {
        assert_eq!(MsgAccumulator::new("").line_number(0), 1);
    }

    #[test]
    fn offset_to_line_multiline() {
        let input = "abc\ndef\nghi";
        let acc = MsgAccumulator::new(input);
        let line_nums: Vec<_> = input
            .chars()
            .enumerate()
            .map(|(idx, ch)| (acc.line_number(idx), ch))
            .collect();

        assert_eq!(
            line_nums,
            vec![
                (1, 'a'),
                (1, 'b'),
                (1, 'c'),
                (1, '\n'),
                (2, 'd'),
                (2, 'e'),
                (2, 'f'),
                (2, '\n'),
                (3, 'g'),
                (3, 'h'),
                (3, 'i'),
            ]
        );
    }

    fn msg_to_tuple<'doc>(msg: &Message, document: &'doc str) -> (usize, &'doc str) {
        (msg.line_number(), msg.text(document))
    }

    macro_rules! assert_extract_msgs {
        ($document:expr, $exp:expr) => {{
            let document = $document;
            assert_eq!(
                extract_msgs(document)
                    .iter()
                    .map(|m| msg_to_tuple(m, document))
                    .collect::<Vec<_>>(),
                $exp
            )
        }};
    }

    #[test]
    fn extract_msgs_empty() {
        assert_extract_msgs!("", vec![]);
    }

    #[test]
    fn extract_msgs_single_line() {
        assert_extract_msgs!("This is a paragraph.", vec![(1, "This is a paragraph.")]);
    }

    #[test]
    fn extract_msgs_simple() {
        assert_extract_msgs!(
            "This is\n\
                 the first\n\
                 paragraph.🦀\n\
                 \n\
                 Second paragraph.",
            vec![
                (1, "This is\nthe first\nparagraph.🦀"),
                (5, "Second paragraph.")
            ]
        );
    }

    #[test]
    fn extract_msgs_leading_newlines() {
        assert_extract_msgs!(
            "\n\
                 \n\
                 \n\
                 This is the\n\
                 first paragraph.",
            vec![(4, "This is the\nfirst paragraph.")]
        );
    }

    #[test]
    fn extract_msgs_trailing_newlines() {
        assert_extract_msgs!(
            "This is\n\
                 a paragraph.\n\
                 \n\
                 \n",
            vec![(1, "This is\na paragraph.")]
        );
    }

    #[test]
    fn extract_msgs_styled_text() {
        assert_extract_msgs!(
            "**This** ~~message~~ _has_ `code` *style*\n",
            vec![(1, "**This** ~~message~~ _has_ `code` *style*")]
        );
    }

    #[test]
    fn extract_msgs_inline_html() {
        assert_extract_msgs!(
            "Hi <script>alert('there');</script>",
            vec![(1, "Hi <script>alert('there');</script>")]
        );
    }

    #[test]
    fn extract_msgs_links() {
        assert_extract_msgs!(
            "See [this page](https://example.com) for more info.",
            vec![(1, "See [this page](https://example.com) for more info.")]
        );
    }

    #[test]
    fn extract_msgs_links_footer() {
        assert_extract_msgs!(
            r#"
* [Brazilian Portuguese][pt-BR] and
* [Korean][ko]

[pt-BR]: https://google.github.io/comprehensive-rust/pt-BR/
[ko]: https://google.github.io/comprehensive-rust/ko/
"#,
            // The parser does not include the referenced links in the events it produces. This is
            // probably OK: links would not have been translated, anyway.
            vec![(2, "* [Brazilian Portuguese][pt-BR] and\n* [Korean][ko]"),]
        );
    }

    #[test]
    fn extract_msgs_block_quote() {
        assert_extract_msgs!(
            r#"One of my favorite quotes is:

> Don't believe everything you read on the Internet.
>
> I didn't say this second part, but I needed a paragraph for testing.

--Abraham Lincoln
"#,
            vec![
                (1, "One of my favorite quotes is:"),
                (3, "> Don't believe everything you read on the Internet.\n>\n> I didn't say this second part, but I needed a paragraph for testing."),
                (7, "--Abraham Lincoln"),
            ]
        );
    }

    #[test]
    fn extract_msgs_table() {
        let table = r#"| Module Type       | Description
|-------------------|------------------------------------------------------------------------
| `rust_binary`     | Produces a Rust binary.
| `rust_library`    | Produces a Rust library, and provides both `rlib` and `dylib` variants."#;
        let input = format!("Hey, a table\n\n{table}\n\nFooter.\n");
        // tables are included as part of the text.
        assert_extract_msgs!(
            &input,
            vec![(1, "Hey, a table"), (3, table), (8, "Footer."),]
        );
    }

    #[test]
    fn extract_msgs_code_block() {
        assert_extract_msgs!("Preamble\n```rust\nfn hello() {\n  some_code()\n\n  todo!()\n}\n```\nPostamble",
            vec![
                (1, "Preamble"),
                (2, "```rust\nfn hello() {\n  some_code()\n\n  todo!()\n}\n```"),
                (9, "Postamble")
            ]
        );
    }

    #[test]
    fn extract_msgs_details() {
        // This isn't great, because the parser treats any data following a tag as also HTML,
        // but works well enough when `<details>` has blank lines before and after.
        assert_extract_msgs!(
            "Preamble\n<details>\nSome Details\n</details>\n\nPostamble",
            vec![
                (1, "Preamble"),
                (2, "<details>\nSome Details\n</details>"),
                (6, "Postamble")
            ]
        );
        assert_extract_msgs!(
            "Preamble\n\n<details>\n\nSome Details\n\n</details>\n\nPostamble",
            vec![
                (1, "Preamble"),
                (3, "<details>"),
                (5, "Some Details"),
                (7, "</details>"),
                (9, "Postamble")
            ]
        );
    }

    #[test]
    fn extract_msgs_list() {
        assert_extract_msgs!(
            "Some text\n * List item 1🦀\n * List item 2\n\nMore text",
            vec![
                (1, "Some text"),
                (2, " * List item 1🦀\n * List item 2"),
                (5, "More text")
            ]
        );
    }

    #[test]
    fn extract_msgs_multilevel_list() {
        assert_extract_msgs!("Some text\n * List item 1\n * List item 2\n    * Sublist 1\n    * Sublist 2\n\nMore text",
            vec![
                (1, "Some text"),
                (2, " * List item 1\n * List item 2\n    * Sublist 1\n    * Sublist 2"),
                (7, "More text")
            ]
        );
    }

    #[test]
    fn extract_msgs_list_with_paras() {
        assert_extract_msgs!(
            r#"* Item 1.
* Item 2,
  two lines.

  * Sub 1.
  * Sub 2.

  More paragraph.

Top level.
"#,
            vec![
                (1, "* Item 1.\n* Item 2,\n  two lines.\n\n  * Sub 1.\n  * Sub 2.\n\n  More paragraph."),
                (10, "Top level."),
            ]
        );
    }

    #[test]
    fn extract_msgs_headings() {
        assert_extract_msgs!(
            r#"Some text
# Headline News🦀

* A
* List

## Subheading
"#,
            vec![
                (1, "Some text"),
                (2, "# Headline News🦀"),
                (4, "* A\n* List"),
                (7, "## Subheading")
            ]
        );
    }

    #[test]
    fn extract_msgs_code_followed_by_details() {
        // This is a regression test for an error that would incorrectly combine
        // CodeBlock and HTML.
        assert_extract_msgs!(
            r#"```bob
BOB
```

<details>

* Blah blah

</details>
"#,
            vec![
                (1, "```bob\nBOB\n```"),
                (5, "<details>"),
                (7, "* Blah blah"),
                (9, "</details>"),
            ]
        );
    }
}
