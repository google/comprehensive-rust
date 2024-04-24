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

use std::fmt;
use std::path::Path;

/// Given a source_path for the markdown file being rendered and a source_path
/// for the target, generate a relative link.
pub fn relative_link(
    doc_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
) -> String {
    let doc_path = doc_path.as_ref();
    let target_path = target_path.as_ref();

    let mut dotdot = -1;
    for parent in doc_path.ancestors() {
        if target_path.starts_with(parent) {
            break;
        }
        dotdot += 1;
    }
    if dotdot > 0 {
        format!("{}{}", "../".repeat(dotdot as usize), target_path.display())
    } else {
        format!("./{}", target_path.display())
    }
}

/// Represent the given duration in a human-readable way.
///
/// This will round times longer than 5 minutes to the next 5-minute interval.
pub fn duration(mut minutes: u64) -> String {
    if minutes > 5 {
        minutes += 4;
        minutes -= minutes % 5;
    }

    let (hours, minutes) = (minutes / 60, minutes % 60);
    match (hours, minutes) {
        (0, 1) => "1 minute".into(),
        (0, m) => format!("{m} minutes"),
        (1, 0) => "1 hour".into(),
        (1, m) => format!("1 hour and {m} minutes"),
        (h, 0) => format!("{h} hours"),
        (h, m) => format!("{h} hours and {m} minutes"),
    }
}

/// Table implements Display to format a two-dimensional table as markdown,
/// following https://github.github.com/gfm/#tables-extension-.
pub struct Table<const N: usize> {
    header: [String; N],
    rows: Vec<[String; N]>,
}

impl<const N: usize> Table<N> {
    pub fn new(header: [String; N]) -> Self {
        Self { header, rows: Vec::new() }
    }

    pub fn add_row(&mut self, row: [String; N]) {
        self.rows.push(row);
    }

    fn write_row<'a, I: Iterator<Item = &'a str>>(
        &self,
        f: &mut fmt::Formatter<'_>,
        iter: I,
    ) -> fmt::Result {
        write!(f, "|")?;
        for cell in iter {
            write!(f, " {} |", cell)?;
        }
        write!(f, "\n")
    }
}

impl<const N: usize> fmt::Display for Table<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_row(f, self.header.iter().map(|s| s.as_str()))?;
        self.write_row(f, self.header.iter().map(|_| "-"))?;
        for row in &self.rows {
            self.write_row(f, row.iter().map(|s| s.as_str()))?
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn relative_link_same_dir() {
        assert_eq!(
            relative_link(Path::new("welcome.md"), Path::new("hello-world.md")),
            "./hello-world.md".to_string()
        );
    }

    #[test]
    fn relative_link_subdir() {
        assert_eq!(
            relative_link(
                Path::new("hello-world.md"),
                Path::new("hello-world/foo.md")
            ),
            "./hello-world/foo.md".to_string()
        );
    }

    #[test]
    fn relative_link_parent_dir() {
        assert_eq!(
            relative_link(
                Path::new("references/foo.md"),
                Path::new("hello-world.md")
            ),
            "../hello-world.md".to_string()
        );
    }

    #[test]
    fn relative_link_deep_parent_dir() {
        assert_eq!(
            relative_link(
                Path::new("references/foo/bar.md"),
                Path::new("hello-world.md")
            ),
            "../../hello-world.md".to_string()
        );
    }

    #[test]
    fn relative_link_peer_dir() {
        assert_eq!(
            relative_link(
                Path::new("references/foo.md"),
                Path::new("hello-world/foo.md")
            ),
            "../hello-world/foo.md".to_string()
        );
    }

    #[test]
    fn duration_no_time() {
        assert_eq!(duration(0), "0 minutes")
    }

    #[test]
    fn duration_single_minute() {
        assert_eq!(duration(1), "1 minute")
    }

    #[test]
    fn duration_two_minutes() {
        assert_eq!(duration(2), "2 minutes")
    }

    #[test]
    fn duration_seven_minutes() {
        assert_eq!(duration(7), "10 minutes")
    }

    #[test]
    fn duration_hour() {
        assert_eq!(duration(60), "1 hour")
    }

    #[test]
    fn duration_hour_mins() {
        assert_eq!(duration(61), "1 hour and 5 minutes")
    }

    #[test]
    fn duration_hours() {
        assert_eq!(duration(120), "2 hours")
    }

    #[test]
    fn duration_hours_mins() {
        assert_eq!(duration(130), "2 hours and 10 minutes")
    }

    #[test]
    fn table() {
        let mut table = Table::new(["a".into(), "b".into()]);
        table.add_row(["a1".into(), "b1".into()]);
        table.add_row(["a2".into(), "b2".into()]);
        assert_eq!(
            format!("{}", table),
            "| a | b |\n| - | - |\n| a1 | b1 |\n| a2 | b2 |\n"
        );
    }
}
