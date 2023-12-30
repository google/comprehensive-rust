// Copyright 2023 Google LLC
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

#![allow(dead_code)]

// ANCHOR: solution
// ANCHOR: Package
#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// A representation of a software package.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    // ANCHOR_END: Package
    // ANCHOR: as_dependency
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    fn as_dependency(&self) -> Dependency {
        // ANCHOR_END: as_dependency
        Dependency {
            name: self.name.clone(),
            version_expression: self.version.clone(),
        }
    }
}

// ANCHOR: PackageBuilder
/// A builder for a Package. Use `build()` to create the `Package` itself.
struct PackageBuilder(Package);

impl PackageBuilder {
    // ANCHOR_END: PackageBuilder
    // ANCHOR: new
    fn new(name: impl Into<String>) -> Self {
        // ANCHOR_END: new
        Self(Package {
            name: name.into(),
            version: "0.1".into(),
            authors: vec![],
            dependencies: vec![],
            language: None,
        })
    }

    // ANCHOR: version
    /// Set the package version.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }
    // ANCHOR_END: version

    // ANCHOR: authors
    /// Set the package authors.
    fn authors(mut self, authors: Vec<String>) -> Self {
        // ANCHOR_END: authors
        self.0.authors = authors;
        self
    }

    // ANCHOR: dependency
    /// Add an additional dependency.
    fn dependency(mut self, dependency: Dependency) -> Self {
        // ANCHOR_END: dependency
        self.0.dependencies.push(dependency);
        self
    }

    // ANCHOR: language
    /// Set the language. If not set, language defaults to None.
    fn language(mut self, language: Language) -> Self {
        // ANCHOR_END: language
        self.0.language = Some(language);
        self
    }

    // ANCHOR: build
    fn build(self) -> Package {
        self.0
    }
}
// ANCHOR_END: build

// ANCHOR: main
fn main() {
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");
    let log =
        PackageBuilder::new("log").version("0.4").language(Language::Rust).build();
    println!("log: {log:?}");
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");
}
// ANCHOR_END: main
