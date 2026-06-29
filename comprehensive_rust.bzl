"""Macros to generate a Comprehensive Rust book."""

load("@rules_rust_mdbook//:defs.bzl", "mdbook")

def comprehensive_rust_translations(languages):
    """Generate a Comprehensive Rust book for each language."""
    for lang in languages:
        comprehensive_rust_book(name = lang)

def comprehensive_rust_book(name):
    """Generate a Comprehensive Rust book for a language.

    Targets are generated for each of the `mdbook` renderers (output
    plugins) we use, so that output format is in its own target for
    ease of use. For the name `xx`, you get the following targets:

    - `xx_html`: HTML output.
    - `xx_pandoc`: PDF output.

    For English (`name="en"`), you also get

    - `en_exerciser`: extracted exercises.
    - `en_xgettext`: the `messages.pot` file.
    - `en_lintcheck2`: output of checking links.
    """
    renderers = ["html", "pandoc"]
    if name == "en":
        renderers += ["exerciser", "xgettext", "linkcheck2"]

    po = "//po:%s.po" % name if name != "en" else None

    for renderer in renderers:
        mdbook(
            name = "%s_%s" % (name, renderer),
            book = "@backdated_sources//:%s_%s_book" % (name, renderer),
            srcs = ["@backdated_sources//:%s_%s_content" % (name, renderer)] + ([po] if po else []),
            plugins = ["//:mdbook-plugins"],
            tags = ["no-sandbox"],
            visibility = ["//visibility:public"],
        )

    native.filegroup(
        name = name,
        srcs = [":%s_%s" % (name, r) for r in renderers],
        visibility = ["//visibility:public"],
    )
