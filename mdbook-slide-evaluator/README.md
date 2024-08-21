mdbook-slide-evaluator allows you to evaluate the rendered slides. This way one
can find if there is too much content on the slides and if sorted by size one
can focus on the worst violations first.

# How to run

## Start a WebDriver compatible browser

### Alternative: Docker

Start a
[selenium docker container](https://github.com/SeleniumHQ/docker-selenium?tab=readme-ov-file#quick-start)
and mount the book folder into the container at `/book/`:

```
$ docker run -d -p 4444:4444 -p 7900:7900 --volume /path/to/my/workspace/comprehensive-rust/book:/book --shm-size="2g" selenium/standalone-chromium:latest
```

As the tool is running with a different base directory, you can use a relative
directory e.g., `../book/`:

```
$ cargo run -- ../book
```

### Alternative: Local WebDriver browser with `webdriver-manager`

Use [webdriver-manager](https://pypi.org/project/webdriver-manager/) to install
a e.g., a `chromedriver` onto your system with:

```
$ pip install selenium webdriver-manager
$ python3
from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.chrome.options import Options
from webdriver_manager.chrome import ChromeDriverManager

driver = webdriver.Chrome(service=Service(ChromeDriverManager().install(), port=4444))
# end the session when you are done.
```

You can provide the absolute path here as the browser has the same view on the
filesystem:

```
$ cargo run -- /path/to/my/workspace/comprehensive-rust/book
```

## Run mdbook-slide-size

If a screenshot directory is provided, the tool can also create screenshots to
evaluate this manually. The tool always recursively grabs all `*.html` files
from the given directory and processes it.

```
cargo run -- --screenshot-dir screenshots ../book/html/
```

# Roadmap

To avoid a `docker mount`, try to build a data uri from the given slide. This
has the challenge that this contains links to local stylesheets that have to be
included. `css_inline` can be used for that and this already works (kind of). If
someone wants to take a stab at this, feel free to contact the author.
