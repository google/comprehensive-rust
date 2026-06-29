"""Starlark Configuration Transitions for Cross-Compilation.

In Bazel, test targets (like sh_test) are host-executed. If the global
invocation sets --platforms=//platforms:aarch64-unknown-none, Bazel
attempts to run the test runners themselves on the target bare-metal
platform. Because there are no test execution toolchains for
bare-metal targets, this results in analysis failures.

To resolve this, we keep the global invocation targeting the host
platform, and use these custom rules (aarch64_binary,
thumbv7em_binary) to apply a "transition". A transition modifies the
configuration (the --platforms flag) for the dependency edge to the
target binary. This ensures:

1. The binaries are always compiled for their specific bare-metal
   target platform.

2. The parent test targets (sh_test) are built and run on the host
   platform.
"""

def _aarch64_transition_impl(settings, attr):
    return {"//command_line_option:platforms": ["//platforms:aarch64-unknown-none"]}

_aarch64_transition = transition(
    implementation = _aarch64_transition_impl,
    inputs = [],
    outputs = ["//command_line_option:platforms"],
)

def _transition_rule_impl(ctx):
    # Retrieve the files from the transitioned target
    return [DefaultInfo(files = ctx.attr.dep[0][DefaultInfo].files)]

aarch64_binary = rule(
    implementation = _transition_rule_impl,
    attrs = {
        "dep": attr.label(cfg = _aarch64_transition),
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
)

def _thumbv7em_transition_impl(settings, attr):
    return {"//command_line_option:platforms": ["//platforms:thumbv7em-none-eabihf"]}

_thumbv7em_transition = transition(
    implementation = _thumbv7em_transition_impl,
    inputs = [],
    outputs = ["//command_line_option:platforms"],
)

thumbv7em_binary = rule(
    implementation = _transition_rule_impl,
    attrs = {
        "dep": attr.label(cfg = _thumbv7em_transition),
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
)
