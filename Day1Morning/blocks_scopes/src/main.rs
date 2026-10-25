// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn main() {
    let z = 13;
    let x = {
        let y = 10;
        dbg!(y);
        z - y
    };
    dbg!(x);
    // dbg!(y);
}