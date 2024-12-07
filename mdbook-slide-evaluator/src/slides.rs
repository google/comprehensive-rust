// Copyright 2024 Google LLC
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

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Ok;
use log::debug;

/// a slide is a page in the book
#[derive(Debug, Clone)]
pub struct Slide {
    pub filename: Arc<Path>,
}

/// a book is a collection of slides
pub struct Book {
    /// the path to the root directory of this book
    _source_dir: PathBuf,
    /// the collection of slides
    slides: Vec<Slide>,
}

impl Book {
    /// create a book from all html files in the source_dir
    pub fn from_html_slides(
        source_dir: PathBuf,
        ignore_redirects: bool,
    ) -> anyhow::Result<Book> {
        let mut slides = vec![];
        let files = glob::glob(&format!(
            "{}/**/*.html",
            source_dir.to_str().expect("invalid path")
        ))?;
        for file in files {
            let file = file?;
            if ignore_redirects && file_is_redirect(&file)? {
                debug!("slide {file:?} is a redirect page");
                continue;
            }
            let slide = Slide { filename: file.into() };
            debug!("add {:?}", slide);
            slides.push(slide);
        }
        debug!("processing {} slides", slides.len());
        Ok(Book { _source_dir: source_dir, slides })
    }

    /// return a reference to the slides of this book
    pub fn slides(&self) -> &[Slide] {
        &self.slides
    }
}

const HTML_REDIRECT_PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Redirecting...</title>"#;

/// check if the file is starting with the mdbook redirect page.
/// This method is optimized to not read the entire file but only the start
fn file_is_redirect(filename: &PathBuf) -> anyhow::Result<bool> {
    let mut file = File::open(filename)?;
    // create a buffer with the exact length of the text that is checked
    let mut file_start_buffer = [0u8; HTML_REDIRECT_PAGE.len()];
    // read only the part that is relevant
    file.read_exact(&mut file_start_buffer)?;
    Ok(file_start_buffer.eq(HTML_REDIRECT_PAGE.as_bytes()))
}
