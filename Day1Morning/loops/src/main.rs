// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn main() {
    //WHILE
    println!("WHILE LOOP:");
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    dbg!(x);

    println!("-----------------");

    //FOR
    println!("FOR LOOP:");
    for x in 1..5 {
        dbg!(x);
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem);
    }

    println!("-----------------");

    //LOOP
    println!("LOOP LOOP:");
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }
}