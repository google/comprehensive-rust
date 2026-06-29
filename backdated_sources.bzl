"""Back-dating and book assembly logic for Comprehensive Rust.

Translations (stored as `.po` files) are not in sync with the main
English content. To ensure the translations apply correctly, we build
the translations from the date when the translation was last updated,
as denoted by the `POT-Creation-Date` header.

We do this by creating isolated, read-only external repositories for
each required point in time. This ensures the local workspace remains
clean and the builds are hermetic.
"""

load("@toml.bzl", "toml")
load("@yaml.bzl", "yaml")

def _book_sources_repo_impl(repository_ctx):
    """Creates a self-contained repository with filtered book.toml and source files."""
    lang = repository_ctx.attr.lang
    commit = repository_ctx.attr.commit
    renderers = repository_ctx.attr.renderers

    # Obtain/extract content files.
    if commit:
        # Run git archive to extract src/, third_party/, and book.toml from the commit.
        archive = repository_ctx.path("archive.tar.gz")
        result = repository_ctx.execute(
            ["git", "-C", repository_ctx.workspace_root, "archive"] +
            ["--output", archive] + [commit] + ["src/", "third_party/", "book.toml"],
        )
        if result.return_code != 0:
            fail("Failed to run git archive for commit {}: {}".format(commit, result.stderr))

        repository_ctx.extract(archive)
        repository_ctx.delete(archive)

        # Read pristine book.toml from the archive.
        pristine_path = repository_ctx.path("book.toml")
        pristine_content = repository_ctx.read(pristine_path)

        # Delete it so we can write our patched version.
        repository_ctx.delete(pristine_path)

        pristine_book = toml.decode(pristine_content)
    else:
        # For current HEAD, symlink local src and third_party.
        repository_ctx.symlink(repository_ctx.path("%s/src" % repository_ctx.workspace_root), "src")
        repository_ctx.symlink(repository_ctx.path("%s/third_party" % repository_ctx.workspace_root), "third_party")
        pristine_book = None

    # Always symlink theme from local workspace to keep it up to date.
    repository_ctx.symlink(repository_ctx.path("%s/theme" % repository_ctx.workspace_root), "theme")

    # Read template/base book.toml.
    base_book_path = repository_ctx.path(repository_ctx.attr.book)
    book = toml.decode(repository_ctx.read(base_book_path))

    # Apply pristine Rust/settings if it is backdated.
    if pristine_book and "rust" in pristine_book:
        book["rust"] = pristine_book["rust"]

    # Patch language-specific settings.
    book["book"]["language"] = lang
    book["output"]["html"]["site-url"] = "/comprehensive-rust/{}/".format(lang)
    book["output"]["html"].pop("redirect", None)

    book["output"].pop("linkcheck", None)
    book["output"].pop("linkcheck2", None)

    book["preprocessor"]["gettext"]["po-dir"] = "../../po"
    book["build"]["extra-watch-dirs"] = ["../../po"]

    book["output"]["pandoc"]["disabled"] = repository_ctx.which("pandoc") == None

    # Apply language-specific pandoc variables from .github/pandoc/<lang>.yaml.
    override_path = repository_ctx.path("%s/.github/pandoc/%s.yaml" % (repository_ctx.workspace_root, lang))
    if override_path.exists:
        override = yaml.get_value(yaml.parse(repository_ctx.read(override_path)))
        book["output"]["pandoc"]["profile"]["pdf"]["variables"].update(override["variables"])

    # Filter output renderers.
    filtered_output = {}
    for r in renderers:
        if r in book["output"]:
            config = book["output"][r]
            if type(config) == "dict":
                config = dict(config)
                config["disabled"] = False
            filtered_output[r] = config
    book["output"] = filtered_output

    # Write book.toml.
    repository_ctx.file("book.toml", toml.encode(book), executable = False)

    # Clean up any nested BUILD files to prevent package boundaries from blocking globs.
    if commit:
        result = repository_ctx.execute(
            ["find", ".", "-type", "f", "(", "-name", "BUILD", "-o", "-name", "BUILD.bazel", ")", "-delete"],
        )
        if result.return_code != 0:
            fail("Failed to clean up BUILD files: {}".format(result.stderr))

    # Generate BUILD.bazel file.
    repository_ctx.file("BUILD.bazel", """
filegroup(
    name = "content",
    srcs = glob(["src/**", "third_party/**", "theme/**"]),
    visibility = ["//visibility:public"],
)

exports_files(
    ["book.toml"],
    visibility = ["//visibility:public"],
)
""", executable = False)

book_sources_repo = repository_rule(
    implementation = _book_sources_repo_impl,
    attrs = {
        "lang": attr.string(mandatory = True, doc = "The language for the book."),
        "commit": attr.string(doc = "The Git commit to archive content from (optional)."),
        "book": attr.label(mandatory = True, doc = "Label of the template book.toml file."),
        "renderers": attr.string_list(mandatory = True, doc = "The list of renderers to keep."),
    },
    doc = "Creates a self-contained repository with a filtered book.toml and sources.",
)

def _hub_repo_impl(repository_ctx):
    """Creates a hub repository that provides aliases for all languages."""
    content = ""
    for name, actual in repository_ctx.attr.targets.items():
        content += """alias(
    name = "{}",
    actual = "{}",
    visibility = ["//visibility:public"],
)\n\n""".format(name, actual)
    repository_ctx.file("BUILD.bazel", content, executable = False)

hub_repo = repository_rule(
    implementation = _hub_repo_impl,
    attrs = {
        "targets": attr.string_dict(mandatory = True, doc = "Map of alias name to actual target."),
    },
    doc = "Creates a hub repository that provides aliases for all languages.",
)

def _extract_date(module_ctx, po_path):
    """Parses the POT-Creation-Date from a .po file header."""
    header_chunk = module_ctx.read(po_path)[:10000]
    for line in header_chunk.splitlines():
        if "POT-Creation-Date:" in line:
            return line.split("POT-Creation-Date: ")[1].strip('"\\n')
    return "now"

def _backdated_sources_extension_impl(module_ctx):
    """Module extension that orchestrates creation of all back-dated repositories."""
    lang_configs = [struct(name = "en", commit = "")]

    for mod in module_ctx.modules:
        for tag in mod.tags.language:
            po = module_ctx.path(tag.po)
            name = po.basename
            if name.endswith(".po"):
                name = name[:-3]

            if name == "en":
                continue

            date = _extract_date(module_ctx, po)

            rev_list = module_ctx.execute(
                ["git", "-C", po.dirname, "rev-list", "-n", "1", "--before", date, "HEAD"],
            )
            if rev_list.return_code != 0:
                fail("Failed to get commit for {} at {}: {}".format(po, date, rev_list.stderr))

            commit = rev_list.stdout.strip()
            if not commit:
                rev_parse = module_ctx.execute(
                    ["git", "-C", po.dirname, "rev-parse", "HEAD"],
                )
                commit = rev_parse.stdout.strip()

            lang_configs.append(struct(
                name = name,
                commit = commit,
            ))

    # Generate filtered book.toml + sources repositories per language and renderer.
    for cfg in lang_configs:
        renderers = ["html", "pandoc"]
        if cfg.name == "en":
            renderers += ["exerciser", "xgettext", "linkcheck2"]

        for r in renderers:
            book_sources_repo(
                name = "book_%s_%s" % (cfg.name, r),
                lang = cfg.name,
                commit = cfg.commit,
                book = "@@//:book.toml",
                renderers = [r],
            )

    # Populate hub targets.
    hub_targets = {}
    for cfg in lang_configs:
        renderers = ["html", "pandoc"]
        if cfg.name == "en":
            renderers += ["exerciser", "xgettext", "linkcheck2"]

        for r in renderers:
            hub_targets["%s_%s_book" % (cfg.name, r)] = "@book_%s_%s//:book.toml" % (cfg.name, r)
            hub_targets["%s_%s_content" % (cfg.name, r)] = "@book_%s_%s//:content" % (cfg.name, r)

    hub_repo(
        name = "backdated_sources",
        targets = hub_targets,
    )

backdated_sources = module_extension(
    implementation = _backdated_sources_extension_impl,
    tag_classes = {
        "language": tag_class(
            attrs = {
                "po": attr.label(mandatory = True, doc = "Label of the .po file."),
            },
            doc = "Defines a language to be included in the translation build.",
        ),
    },
    doc = "A module extension to manage back-dated translation source repositories.",
)
