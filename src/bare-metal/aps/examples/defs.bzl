"""Macros for defining AArch64 bare-metal APS examples."""

load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_shell//shell:sh_test.bzl", "sh_test")
load("//platforms:transition.bzl", "aarch64_binary")

def aps_example(name, deps):
    """Defines an APS bare-metal example binary and test runner.

    Args:
        name: Name of the binary target.
        deps: Dependencies for the rust_binary.
    """
    rust_binary(
        name = name,
        srcs = native.glob(["src/**/*.rs"]),
        compile_data = native.glob(["src/**/*.S"]),
        crate_root = "src/main_%s.rs" % name,
        edition = "2024",
        linker_script = ":image.ld",
        rustc_flags = [
            "-C",
            "linker=rust-lld",
        ],
        target_compatible_with = [
            "@platforms//os:none",
            "@platforms//cpu:aarch64",
        ],
        deps = deps,
    )

    native.genrule(
        name = "%s_bin" % name,
        srcs = [":%s" % name],
        outs = ["%s.bin" % name],
        cmd = "$(execpath @rust_host_tools_nightly//:rust-objcopy) -O binary $(location :%s) $@" % name,
        tools = ["@rust_host_tools_nightly//:rust-objcopy"],
    )

    aarch64_binary(
        name = "%s_bin_aarch64" % name,
        dep = ":%s_bin" % name,
    )

    sh_test(
        name = "%s_test" % name,
        srcs = ["run_test.sh"],
        args = [name],
        data = [":%s_bin_aarch64" % name],
        tags = [
            "manual",
            "qemu",
        ],
    )
