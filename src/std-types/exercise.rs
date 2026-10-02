// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0
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

#![allow(unused_variables, dead_code)]
// ANCHOR: solution
use std::collections::HashMap;
use std::hash::Hash;

/// Counter counts the number of times each value of type T has been seen.
struct Counter<T> {
    counts: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T> {
    /// Create a new Counter.
    fn new() -> Self {
        Counter { counts: HashMap::new() }
    }

    /// Count an occurrence of the given value.
    fn count(&mut self, item: T) {
        *self.counts.entry(item).or_default() += 1;
    }

    /// Return the number of times the given value has been seen.
    fn times_seen(&self, item: T) -> u64 {
        self.counts.get(&item).copied().unwrap_or_default()
    }
}

// ANCHOR: main
fn main() {
    let mut charctr = Counter::new();
    charctr.count(' ');
    charctr.count('a');
    charctr.count('¥');
    charctr.count('a');
    println!(
        "most common character: {:?}",
        charctr.counts.keys().max_by_key(|&c| charctr.times_seen(*c))
    );

    let mut intctr = Counter::new();
    intctr.count(13);
    intctr.count(14);
    intctr.count(16);
    intctr.count(14);
    intctr.count(14);
    intctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", intctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}
// ANCHOR_END: main
