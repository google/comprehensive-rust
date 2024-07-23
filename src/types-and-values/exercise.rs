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

// Omitting `return` is covered later.
#[allow(clippy::needless_return)]
// ANCHOR: solution
// ANCHOR: fib
fn fib(n: u32) -> u32 {
    // ANCHOR_END: fib
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

// ANCHOR: main
fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
// ANCHOR_END: main
