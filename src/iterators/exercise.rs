// Copyright 2022 Google LLC
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

// ANCHOR: solution
// ANCHOR: prefix_matches
pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    // ANCHOR_END: prefix_matches

    let mut request_segments = request_path.split('/');

    for prefix_segment in prefix.split('/') {
        let Some(request_segment) = request_segments.next() else {
            return false;
        };
        if request_segment != prefix_segment && prefix_segment != "*" {
            return false;
        }
    }
    true

    // Alternatively, Iterator::zip() lets us iterate simultaneously over prefix
    // and request segments. The zip() iterator is finished as soon as one of
    // the source iterators is finished, but we need to iterate over all request
    // segments. A neat trick that makes zip() work is to use map() and chain()
    // to produce an iterator that returns Some(str) for each pattern segments,
    // and then returns None indefinitely.
}

// ANCHOR: unit-tests
#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
// ANCHOR_END: unit-tests

fn main() {}
