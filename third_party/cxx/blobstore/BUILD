load("@rules_cc//cc:defs.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//tools/bazel:rust_cxx_bridge.bzl", "rust_cxx_bridge")

rust_binary(
    name = "demo",
    srcs = glob(["src/**/*.rs"]),
    edition = "2021",
    deps = [
        ":blobstore-sys",
        ":bridge",
        "//:cxx",
    ],
)

rust_cxx_bridge(
    name = "bridge",
    src = "src/main.rs",
    deps = [":blobstore-include"],
)

cc_library(
    name = "blobstore-sys",
    srcs = ["src/blobstore.cc"],
    deps = [
        ":blobstore-include",
        ":bridge/include",
    ],
)

cc_library(
    name = "blobstore-include",
    hdrs = ["include/blobstore.h"],
    deps = ["//:core"],
)
