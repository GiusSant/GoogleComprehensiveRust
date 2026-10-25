// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn main() {
    println!("result: {}", interproduct(120, 100, 248));
}