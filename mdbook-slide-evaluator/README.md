mdbook-slide-evaluator allows you to evaluate the rendered slides. This way one can find if there is too much content on the slides and if sorted by size one can focus on the worst violations first.

# How to run
## start a selenium docker container
```
docker run -d -p 4444:4444 -p 7900:7900 --volume /path/to/my/workspace/comprehensive-rust/book:/book --shm-size="2g" selenium/standalone-chromium:latest
```
## run mdbook-slide-size
If a screenshot directory is provided, the tool can also create screenshots to evaluate this manually.
The tool always recursively grabs all *.html files from the given directory and processes it.

```
cargo run --screenshot-dir screenshots ../book/html/
```

# Roadmap
To avoid a docker mount, try to build a data uri from the given slide. This has the challenge that this contains links to local stylesheets that have to be included.
css_inline can be used for that and this already works (kind of). If someone wants to take a stab at this, feel free to contact the author