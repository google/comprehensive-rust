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

//! Representation of Comprehensive Rust as a hierarchy of types.
//!
//! ```ignore
//! Courses -- a collection of courses
//!   Course -- the level of content at which students enroll (fundamentals, android, etc.)
//!     Session -- a block of instructional time (typically morning or afternoon)
//!       Segment -- a collection of slides with a related theme
//!         Slide -- a single topic (may be represented by multiple mdBook chapters)
//! ```
//!
//! This structure is parsed from the format of the book using a combination of
//! the order in which chapters are listed in `SUMMARY.md` and annotations in
//! the frontmatter of each chapter.
//!
//! A book contains a sequence of BookItems, each of which can contain
//! sub-items. A top-level item can potentially introduce a new course, session,
//! segment, and slide all in the same item. If the item has a `course` property
//! in its frontmatter, then it introduces a new course. If it has a `session`
//! property, then it introduces a new session. A top-level item always
//! corresponds 1-to-1 with a segment (as long as it is a chapter), and that
//! item becomes the first slide in that segment. Any other sub-items of the
//! top-level item are treated as further slides in the same segment.

use crate::frontmatter::{split_frontmatter, Frontmatter};
use crate::markdown::{duration, Table};
use mdbook::book::{Book, BookItem, Chapter};
use std::fmt::Write;
use std::path::PathBuf;

/// Duration, in minutes, of breaks between segments in the course.
const BREAK_DURATION: u64 = 10;

/// Courses is simply a collection of Courses.
///
/// Non-instructional material (such as the introduction) has `course: none` and
/// is not included in this data structure.
#[derive(Default, Debug)]
pub struct Courses {
    pub courses: Vec<Course>,
}

/// A Course is the level of content at which students enroll.
///
/// Courses are identified by the `course` property in a session's frontmatter.
/// All sessions with the same value for `course` are grouped into a Course.
#[derive(Default, Debug)]
pub struct Course {
    pub name: String,
    pub sessions: Vec<Session>,
}

/// A Session is a block of instructional time, containing segments. Typically a
/// full day of instruction contains two sessions: morning and afternoon.
///
/// A session is identified by the `session` property in the session's
/// frontmatter. There can be only one session with a given name in a course.
#[derive(Default, Debug)]
pub struct Session {
    pub name: String,
    pub segments: Vec<Segment>,
    target_minutes: u64,
}

/// A Segment is a collection of slides with a related theme.
///
/// A segment is identified as a top-level chapter within a session.
#[derive(Default, Debug)]
pub struct Segment {
    pub name: String,
    pub slides: Vec<Slide>,
}

/// A Slide presents a single topic. It may contain multiple mdBook chapters.
///
/// A slide is identified as an sub-chapter of a segment. Any sub-items of
/// that chapter are also included in the slide.
#[derive(Default, Debug)]
pub struct Slide {
    pub name: String,
    /// Minutes this slide should take to teach.
    pub minutes: u64,
    /// Source paths (`.md` files) in this slide.
    pub source_paths: Vec<PathBuf>,
}

impl Courses {
    /// Extract the course structure from the book. As a side-effect, the
    /// frontmatter is stripped from each slide.
    pub fn extract_structure(mut book: Book) -> anyhow::Result<(Self, Book)> {
        let mut courses = Courses::default();
        let mut current_course_name = None;
        let mut current_session_name = None;

        for item in &mut book.sections {
            // We only want to process chapters, omitting part titles and separators.
            let BookItem::Chapter(chapter) = item else {
                continue;
            };

            let (frontmatter, content) = split_frontmatter(chapter)?;
            chapter.content = content;

            // If 'course' is given, use that course (if not 'none') and reset the
            // session.
            if let Some(course_name) = &frontmatter.course {
                current_session_name = None;
                if course_name == "none" {
                    current_course_name = None;
                } else {
                    current_course_name = Some(course_name.clone());
                }
            }

            // If 'session' is given, use that session.
            if let Some(session_name) = &frontmatter.session {
                current_session_name = Some(session_name.clone());
            }

            if current_course_name.is_some() && current_session_name.is_none() {
                anyhow::bail!(
                    "{:?}: 'session' must appear in frontmatter when 'course' appears",
                    chapter.path
                );
            }

            // If we have a course and session, then add this chapter to it as a
            // segment.
            if let (Some(course_name), Some(session_name)) =
                (&current_course_name, &current_session_name)
            {
                let course = courses.course_mut(course_name);
                let session = course.session_mut(session_name);
                session.target_minutes += frontmatter.target_minutes.unwrap_or(0);
                session.add_segment(frontmatter, chapter)?;
            }
        }
        Ok((courses, book))
    }

    /// Get a reference to a course, adding a new one if none by this name
    /// exists.
    fn course_mut(&mut self, name: impl AsRef<str>) -> &mut Course {
        let name = name.as_ref();
        if let Some(found_idx) =
            self.courses.iter().position(|course| &course.name == name)
        {
            return &mut self.courses[found_idx];
        }
        let course = Course::new(name);
        self.courses.push(course);
        self.courses.last_mut().unwrap()
    }

    /// Find a course by name.
    pub fn find_course(&self, name: impl AsRef<str>) -> Option<&Course> {
        let name = name.as_ref();
        self.courses.iter().find(|c| c.name == name)
    }

    /// Find the slide generated from the given Chapter within these courses,
    /// returning the "path" to that slide.
    pub fn find_slide(
        &self,
        chapter: &Chapter,
    ) -> Option<(&Course, &Session, &Segment, &Slide)> {
        let Some(ref source_path) = chapter.source_path else {
            return None;
        };

        for course in self {
            for session in course {
                for segment in session {
                    for slide in segment {
                        if slide.source_paths.contains(source_path) {
                            return Some((course, session, segment, slide));
                        }
                    }
                }
            }
        }

        return None;
    }
}

impl<'a> IntoIterator for &'a Courses {
    type Item = &'a Course;
    type IntoIter = std::slice::Iter<'a, Course>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.courses).into_iter()
    }
}

impl Course {
    fn new(name: impl Into<String>) -> Self {
        Course { name: name.into(), ..Default::default() }
    }

    /// Get a reference to a session, adding a new one if none by this name
    /// exists.
    fn session_mut(&mut self, name: impl AsRef<str>) -> &mut Session {
        let name = name.as_ref();
        if let Some(found_idx) =
            self.sessions.iter().position(|session| &session.name == name)
        {
            return &mut self.sessions[found_idx];
        }
        let session = Session::new(name);
        self.sessions.push(session);
        self.sessions.last_mut().unwrap()
    }

    /// Return the total duration of this course, as the sum of all segment
    /// durations.
    ///
    /// This includes breaks between segments, but does not count time between
    /// between sessions.
    pub fn minutes(&self) -> u64 {
        self.into_iter().map(|s| s.minutes()).sum()
    }

    /// Return the target duration of this course, as the sum of all segment
    /// target durations.
    ///
    /// This includes breaks between segments, but does not count time between
    /// sessions.
    pub fn target_minutes(&self) -> u64 {
        self.into_iter().map(|s| s.target_minutes()).sum()
    }

    /// Generate a Markdown schedule for this course, for placement at the given
    /// path.
    pub fn schedule(&self) -> String {
        let mut outline = String::from("Course schedule:\n");
        for session in self {
            writeln!(
                &mut outline,
                " * {} ({}, including breaks)\n",
                session.name,
                duration(session.minutes())
            )
            .unwrap();
            let mut segments = Table::new(["Segment".into(), "Duration".into()]);
            for segment in session {
                // Skip short segments (welcomes, wrap-up, etc.)
                if segment.minutes() == 0 {
                    continue;
                }
                segments
                    .add_row([segment.name.clone(), duration(segment.minutes())]);
            }
            writeln!(&mut outline, "{}\n", segments).unwrap();
        }
        outline
    }
}

impl<'a> IntoIterator for &'a Course {
    type Item = &'a Session;
    type IntoIter = std::slice::Iter<'a, Session>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.sessions).into_iter()
    }
}

impl Session {
    fn new(name: impl Into<String>) -> Self {
        Session { name: name.into(), ..Default::default() }
    }

    /// Add a new segment to the session, representing sub-items as slides.
    fn add_segment(
        &mut self,
        frontmatter: Frontmatter,
        chapter: &mut Chapter,
    ) -> anyhow::Result<()> {
        let mut segment = Segment::new(&chapter.name);
        segment.add_slide(frontmatter, chapter, false)?;
        for sub_chapter in &mut chapter.sub_items {
            let BookItem::Chapter(sub_chapter) = sub_chapter else {
                continue;
            };
            let (frontmatter, content) = split_frontmatter(sub_chapter)?;
            sub_chapter.content = content;

            segment.add_slide(frontmatter, sub_chapter, true)?;
        }
        self.segments.push(segment);
        Ok(())
    }

    /// Generate a Markdown outline for this session, for placement at the given
    /// path.
    pub fn outline(&self) -> String {
        let mut segments = Table::new(["Segment".into(), "Duration".into()]);
        for segment in self {
            // Skip short segments (welcomes, wrap-up, etc.)
            if segment.minutes() == 0 {
                continue;
            }
            segments.add_row([segment.name.clone(), duration(segment.minutes())]);
        }
        format!(
            "Including {BREAK_DURATION} minute breaks, this session should take about {}. It contains:\n\n{}",
            duration(self.minutes()), segments)
    }

    /// Return the total duration of this session.
    pub fn minutes(&self) -> u64 {
        let instructional_time: u64 = self.into_iter().map(|s| s.minutes()).sum();
        if instructional_time == 0 {
            return instructional_time;
        }
        let breaks = (self.into_iter().filter(|s| s.minutes() > 0).count() - 1)
            as u64
            * BREAK_DURATION;
        instructional_time + breaks
    }

    /// Return the target duration of this session.
    ///
    /// This includes breaks between segments.
    pub fn target_minutes(&self) -> u64 {
        if self.target_minutes > 0 {
            self.target_minutes
        } else {
            self.minutes()
        }
    }
}

impl<'a> IntoIterator for &'a Session {
    type Item = &'a Segment;
    type IntoIter = std::slice::Iter<'a, Segment>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.segments).into_iter()
    }
}

impl Segment {
    fn new(name: impl Into<String>) -> Self {
        Segment { name: name.into(), ..Default::default() }
    }

    /// Create a slide from a chapter. If `recurse` is true, sub-items of this
    /// chapter are included in this slide as well.
    fn add_slide(
        &mut self,
        frontmatter: Frontmatter,
        chapter: &mut Chapter,
        recurse: bool,
    ) -> anyhow::Result<()> {
        let mut slide = Slide::new(frontmatter, chapter);

        if recurse {
            slide.add_sub_chapters(chapter)?;
        }
        self.slides.push(slide);
        Ok(())
    }

    /// Return the total duration of this segment (the sum of the durations of
    /// the enclosed slides).
    pub fn minutes(&self) -> u64 {
        self.into_iter().map(|s| s.minutes()).sum()
    }

    pub fn outline(&self) -> String {
        let mut slides = Table::new(["Slide".into(), "Duration".into()]);
        for slide in self {
            if slide.minutes() == 0 {
                continue;
            }
            slides.add_row([slide.name.clone(), duration(slide.minutes())]);
        }
        format!(
            "This segment should take about {}. It contains:\n\n{}",
            duration(self.minutes()),
            slides,
        )
    }
}

impl<'a> IntoIterator for &'a Segment {
    type Item = &'a Slide;
    type IntoIter = std::slice::Iter<'a, Slide>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.slides).into_iter()
    }
}

impl Slide {
    fn new(frontmatter: Frontmatter, chapter: &Chapter) -> Self {
        let mut slide = Self { name: chapter.name.clone(), ..Default::default() };
        slide.add_frontmatter(&frontmatter);
        slide.push_source_path(&chapter.source_path);
        slide
    }

    fn add_frontmatter(&mut self, frontmatter: &Frontmatter) {
        self.minutes += frontmatter.minutes.unwrap_or(0);
    }

    fn push_source_path(&mut self, source_path: &Option<PathBuf>) {
        if let Some(source_path) = &source_path {
            self.source_paths.push(source_path.clone());
        }
    }

    /// Add sub-chapters of this chapter to this slide (recursively).
    fn add_sub_chapters(&mut self, chapter: &mut Chapter) -> anyhow::Result<()> {
        for sub_slide in &mut chapter.sub_items {
            let BookItem::Chapter(sub_slide) = sub_slide else {
                continue;
            };
            let (frontmatter, content) = split_frontmatter(sub_slide)?;
            sub_slide.content = content;

            if frontmatter.course.is_some() || frontmatter.session.is_some() {
                anyhow::bail!(
                    "{:?}: sub-slides may not have 'course' or 'session' set",
                    sub_slide.path
                );
            }
            self.add_frontmatter(&frontmatter);
            self.push_source_path(&sub_slide.source_path);
            self.add_sub_chapters(sub_slide)?;
        }
        Ok(())
    }

    /// Determine whether the given chapter is a sub-chapter of this slide.
    pub fn is_sub_chapter(&self, chapter: &Chapter) -> bool {
        // The first `source_path` in the slide is the "parent" chapter, so anything
        // else is a sub-chapter.
        chapter.source_path.as_ref() != self.source_paths.get(0)
    }

    /// Return the total duration of this slide.
    pub fn minutes(&self) -> u64 {
        self.minutes
    }
}
