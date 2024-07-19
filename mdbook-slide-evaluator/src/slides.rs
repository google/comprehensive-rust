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
    source_dir: PathBuf,
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
            let slide = Slide { filename: Into::<Arc<Path>>::into(file?) };
            debug!("add {:?}", slide);
            slides.push(slide);
        }
        Ok(Book { source_dir, slides })
    }

    /// return a reference to the slides of this book
    pub fn slides(&self) -> &[Slide] {
        &self.slides
    }
}
