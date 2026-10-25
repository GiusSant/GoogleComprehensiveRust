// Copyright 2024 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn main() {
    //ARRAYS
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("a: {a:#?}");
    //a[get_index()] = 0;

    //TUPLES
    let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);

    //ARRAY ITERATION
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    //PATTERNS
    let tuple = (1, 5, 3);
    println!(
        "{tuple:?} : {}",
        if check_order(tuple) {"Ordered"} else {"Not ordered"}
    );
}

fn get_index() -> usize {
    6
}

fn check_order(tuple: (i32, i32, i32)) -> bool {
    let (left, middle, right) = tuple;
    left < middle && middle < right
}