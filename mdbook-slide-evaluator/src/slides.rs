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

use std::path::{Path, PathBuf};
use std::sync::Arc;

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
    pub fn from_html_slides(source_dir: PathBuf) -> anyhow::Result<Book> {
        let mut slides = vec![];
        let files = glob::glob(&format!(
            "{}/**/*.html",
            source_dir.to_str().expect("invalid path")
        ))?;
        for file in files {
            let slide = Slide { filename: file?.into() };
            debug!("add {:?}", slide);
            slides.push(slide);
        }
        Ok(Book { _source_dir: source_dir, slides })
    }

    /// return a reference to the slides of this book
    pub fn slides(&self) -> &[Slide] {
        &self.slides
    }
}
