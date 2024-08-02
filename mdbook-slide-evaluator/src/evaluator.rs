use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Ok};
use fantoccini::elements::Element;
use fantoccini::Client;
use log::{debug, info, warn};
use serde::Serialize;
use url::Url;

use crate::slides::{Book, Slide};

/// An Evaluator is used to render a book that is a collection of slides
/// and extract information from an element on that page. It further can
/// take a screenshot of this element and store it. A webclient instance is
/// created on creation and dropped once the Evaluator is dropped.
pub struct Evaluator<'a> {
    /// webclient used to render html
    webclient: Client,
    /// selector for the element that is scored
    element_selector: fantoccini::wd::Locator<'a>,
    /// store screenshot in this directory if provided
    screenshot_dir: Option<PathBuf>,
    /// html base uri to the source_dir used as a prefix for each page
    html_base_url: Url,
    /// base directory for all processed files
    source_dir: PathBuf,
}

/// element coordinates returned by the browser
#[derive(Debug)]
struct ElementSize {
    /// the width of the element
    width: f64,
    /// the height of the element
    height: f64,
}

impl From<(f64, f64, f64, f64)> for ElementSize {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self { width: value.2, height: value.3 }
    }
}

#[derive(Debug)]
/// holds the evaluation result for a slide
pub struct EvaluationResult {
    /// metadata about the slide
    slide: Slide,
    /// the size of the main content element
    element_size: ElementSize,
}

/// holds all evaluation results for a book
pub struct EvaluationResults {
    /// metadata about the book
    book: Book,
    /// the collected evaluation results
    results: Vec<EvaluationResult>,
}

#[derive(Serialize)]
struct ExportFormat {
    filename: PathBuf,
    element_width: usize,
    element_height: usize,
}

impl EvaluationResults {
    /// export the evaluation results to the given csv file, overwrites if
    /// allowed
    pub fn export_csv(&self, file: &Path, overwrite: bool) -> anyhow::Result<()> {
        if file.exists() && !overwrite {
            Err(anyhow!(
                "Not allowed to overwrite existing evaluation results at {}",
                file.display()
            ))?;
        };

        let mut csv_writer = csv::Writer::from_path(file)?;
        for result in &self.results {
            csv_writer.serialize(ExportFormat {
                filename: (*result.slide.filename).to_path_buf(),
                element_width: result.element_size.width.round() as usize,
                element_height: result.element_size.height.round() as usize,
            })?;
        }
        Ok(())
    }

    /// dump the results to stdout
    pub fn export_stdout(&self) {
        for result in &self.results {
            println!(
                "{}: {}x{}",
                result.slide.filename.display(),
                result.element_size.width,
                result.element_size.height
            );
        }
    }
}

impl<'a> Evaluator<'_> {
    /// create a new instance with the provided config.
    /// fails if the webclient cannot be created
    pub async fn new(
        webdriver: &str,
        element_selector: &'a str,
        screenshot_dir: Option<PathBuf>,
        html_base_url: Url,
        source_dir: PathBuf,
    ) -> anyhow::Result<Evaluator<'a>> {
        let webclient =
            fantoccini::ClientBuilder::native().connect(webdriver).await?;
        // use fullscreen window to avoid arbitrary window size limitations
        webclient.fullscreen_window().await?;
        let element_selector = fantoccini::Locator::XPath(element_selector);
        Ok(Evaluator {
            webclient,
            element_selector,
            screenshot_dir,
            html_base_url,
            source_dir,
        })
    }

    /// navigate the webdriver to the given url.
    /// ensure that html_base_url is set before calling this
    /// after this call the webdriver will see the content at the url
    async fn webdriver_open_url(&self, url: &Url) -> Result<(), anyhow::Error> {
        debug!("open url in webclient: {}", url);
        self.webclient.goto(url.as_str()).await?;
        Ok(())
    }

    /// evaluate the currently opened webpage return the selected content
    /// element
    async fn get_content_element_from_slide(&self) -> anyhow::Result<Element> {
        let result = self.webclient.find(self.element_selector).await?;
        Ok(result)
    }

    /// extract the element coordinates from this element
    async fn get_element_coordinates(
        &self,
        element: &Element,
    ) -> anyhow::Result<ElementSize> {
        let coordinates = Into::<ElementSize>::into(element.rectangle().await?);
        Ok(coordinates)
    }

    /// store the screenshot as png to the given path
    fn store_screenshot(
        &self,
        screenshot: Vec<u8>,
        filename: &Path,
    ) -> anyhow::Result<()> {
        let relative_filename = filename.strip_prefix(&self.source_dir)?;
        let output_filename = self
            .screenshot_dir
            .as_ref()
            .unwrap()
            .join(relative_filename.with_extension("png"));
        debug!("write screenshot to {}", output_filename.to_str().unwrap());

        // create directories if necessary
        let output_dir = output_filename.parent().unwrap();
        if !output_dir.exists() {
            debug!("creating {}", output_dir.to_str().unwrap());
            fs::create_dir_all(output_dir)?;
        }

        let mut file =
            fs::OpenOptions::new().create(true).write(true).open(output_filename)?;

        file.write_all(&screenshot)?;
        Ok(())
    }

    /// evaluate a single slide
    pub async fn eval_slide(
        &self,
        slide: &Slide,
    ) -> anyhow::Result<EvaluationResult> {
        debug!("evaluating {:?}", slide);

        let url = self.html_base_url.join(&slide.filename.display().to_string())?;
        self.webdriver_open_url(&url).await?;

        let content_element = self.get_content_element_from_slide().await?;
        let size = self.get_element_coordinates(&content_element).await?;
        if self.screenshot_dir.is_some() {
            let screenshot = content_element.screenshot().await?;
            self.store_screenshot(screenshot, &slide.filename)?;
        }
        let result = EvaluationResult { slide: slide.clone(), element_size: size };
        debug!("information about element: {:?}", result);
        Ok(result)
    }

    /// evaluate an entire book
    pub async fn eval_book(&self, book: Book) -> anyhow::Result<EvaluationResults> {
        let mut results = vec![];
        debug!("slide count: {}", book.slides().len());
        for slide in book.slides().iter() {
            let Result::Ok(result) = self.eval_slide(slide).await else {
                warn!("slide with no content - ignore: {:?}", slide);
                continue;
            };
            results.push(result);
        }
        Ok(EvaluationResults { book, results })
    }

    /// close the session to the webclient to allow reuse of the instance
    pub async fn close_client(&self) -> anyhow::Result<()> {
        self.webclient.clone().close().await?;
        Ok(())
    }
}
