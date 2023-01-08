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

use once_cell::sync::Lazy;
use regex::Regex;

static PARAGRAPH_SEPARATOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n\n+").unwrap());

/// Extract paragraphs from text.
///
/// Paragraphs are separated by at least two newlines. Returns an
/// iterator over line numbers (starting from 1) and paragraphs.
pub fn extract_paragraphs(text: &str) -> impl Iterator<Item = (usize, &str)> {
    // TODO: This could be made more sophisticated by parsing the
    // Markdown and stripping off the markup characters.
    //
    // As an example, a header like "## My heading" could become just
    // "My heading" in the `.pot` file. Similarly, paragraphs could be
    // unfolded and list items could be translated one-by-one.

    // Skip over leading empty lines.
    let trimmed = text.trim_start_matches('\n');
    let mut matches = PARAGRAPH_SEPARATOR.find_iter(trimmed);
    let mut lineno = 1 + text.len() - trimmed.len();
    let mut last = 0;

    std::iter::from_fn(move || match matches.next() {
        Some(m) => {
            let result = (lineno, &trimmed[last..m.start()]);
            lineno += trimmed[last..m.end()].lines().count();
            last = m.end();
            Some(result)
        }
        None => {
            if last < trimmed.len() {
                let result = (lineno, trimmed[last..].trim_end_matches('\n'));
                last = trimmed.len();
                Some(result)
            } else {
                None
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_iter_eq {
        ($left_iter:expr, $right:expr) => {
            assert_eq!($left_iter.collect::<Vec<_>>(), $right)
        };
    }

    #[test]
    fn test_extract_paragraphs_empty() {
        assert_iter_eq!(extract_paragraphs(""), vec![]);
    }

    #[test]
    fn test_extract_paragraphs_single_line() {
        assert_iter_eq!(
            extract_paragraphs("This is a paragraph."),
            vec![(1, "This is a paragraph.")]
        );
    }

    #[test]
    fn test_extract_paragraphs_simple() {
        assert_iter_eq!(
            extract_paragraphs(
                "This is\n\
                 the first\n\
                 paragraph.\n\
                 \n\
                 Second paragraph."
            ),
            vec![
                (1, "This is\nthe first\nparagraph."),
                (5, "Second paragraph.")
            ]
        );
    }

    #[test]
    fn test_extract_paragraphs_leading_newlines() {
        assert_iter_eq!(
            extract_paragraphs(
                "\n\
                 \n\
                 \n\
                 This is the\n\
                 first paragraph."
            ),
            vec![(4, "This is the\nfirst paragraph.")]
        );
    }

    #[test]
    fn test_extract_paragraphs_trailing_newlines() {
        assert_iter_eq!(
            extract_paragraphs(
                "This is\n\
                 a paragraph.\n\
                 \n\
                 \n"
            ),
            vec![(1, "This is\na paragraph.")]
        );
    }
}
