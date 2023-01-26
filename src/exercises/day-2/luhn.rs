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

// ANCHOR: luhn
pub fn luhn(cc_number: &str) -> bool {
    // ANCHOR_END: luhn
    let mut numbers: Vec<u32> = cc_number
        .chars()
        .map(|x| x.to_digit(10))
        .flatten()
        .collect();

    if numbers.len() < 2 {
        return false;
    }

    for n in numbers.iter_mut().rev().skip(1).step_by(2) {
        let double = *n * 2;
        *n = double % 10 + double / 10;
    }

    numbers.iter().sum::<u32>() % 10 == 0
}


fn main() {
    let cc_number = "1234 5678 1234 5670";
    println!(
        "Is {} a valid credit card number? {}",
        cc_number,
        if luhn(cc_number) { "yes" } else { "no" }
    );
}

// ANCHOR: unit-tests
#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
// ANCHOR_END: unit-tests
