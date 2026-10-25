// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 { 
        gcd(b, a % b)
    } else { 
        return a; 
    }
}

fn main() {
    dbg!(gcd(143, 52));
}