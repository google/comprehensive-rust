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

use std::path::PathBuf;

use clap::Parser;
use log::{debug, info};
use mdbook_slide_evaluator::evaluator::{Evaluator, SlidePolicy};
use mdbook_slide_evaluator::slides::Book;
use tokio_util::sync::CancellationToken;
use url::Url;

#[derive(Parser)]
#[command(version, about, arg_required_else_help(true))]
struct Args {
    /// the URI of the webdriver
    #[arg(long, default_value_t=String::from("http://localhost:4444"))]
    webdriver: String,
    /// the XPath to element that is evaluated
    #[arg(long, default_value_t=String::from(r#"//*[@id="content"]/main"#))]
    element: String,
    /// take screenshots of the content element if provided
    #[arg(short, long)]
    screenshot_dir: Option<PathBuf>,
    /// a base url that is used to render the files (relative to source_dir).
    /// if you mount the slides at source_dir into / in a webdriver docker
    /// container you can use the default
    #[arg(long, default_value_t=Url::parse("file:///").unwrap())]
    base_url: Url,
    /// exports to csv file if provided, otherwise to stdout
    #[arg(long)]
    export: Option<PathBuf>,
    /// allows overwriting the export file
    #[arg(long, default_value_t = false)]
    overwrite: bool,
    /// the height of the webclient that renders the slide
    #[arg(long, default_value_t = 1920)]
    webclient_width: u32,
    /// the width of the webclient that renders the slide
    #[arg(long, default_value_t = 1080)]
    webclient_height: u32,
    /// max width of a slide
    #[arg(long, default_value_t = 750)]
    width: usize,
    /// max height of a slide - default height/width values have 16/9 ratio
    #[arg(long, default_value_t = 1333)]
    height: usize,
    /// if set only violating slides are shown
    #[arg(long, default_value_t = false)]
    violations_only: bool,
    /// directory of the book that is evaluated
    source_dir: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pretty env receives log level from RUST_LOG env variable
    pretty_env_logger::init();

    let args = Args::parse();

    // gather information about the book from the filesystem
    let book = Book::from_html_slides(args.source_dir.clone())?;

    // create a new webclient that is used by the evaluator
    let webclient: fantoccini::Client =
        fantoccini::ClientBuilder::native().connect(&args.webdriver).await?;
    // use a defined window size for reproducible results
    webclient.set_window_size(args.webclient_width, args.webclient_height).await?;

    let cancellation_token = CancellationToken::new();

    let slide_policy =
        SlidePolicy { max_width: args.width, max_height: args.height };

    // create a new evaluator (connects to the provided webdriver)
    let evaluator = Evaluator::new(
        webclient.clone(),
        &args.element,
        args.screenshot_dir,
        args.base_url,
        args.source_dir.to_path_buf(),
        cancellation_token.clone(),
        slide_policy,
    );

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("received CTRL+C");
        // send a cancel signal
        cancellation_token.cancel();
    });

    // evaluate each slide
    let score_results = evaluator.eval_book(book).await?;

    if let Some(export_file) = args.export {
        score_results.export_csv(
            &export_file,
            args.overwrite,
            args.violations_only,
        )?;
    } else {
        score_results.export_stdout(args.violations_only);
    }

    // close webclient as otherwise the unclosed session cannot be reused
    debug!("closing webclient");
    webclient.close().await?;
    Ok(())
}
