// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0

fn main() {
    //SHARED REFERENCES
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(r);

    r = &b;
    dbg!(r);

    //EXCLUSIVE REFERENCES
    let mut point = (1,2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    dbg!(point);

    //SLICES
    let a: [i32; 6] = [1,2,3,4,5,6];
    println!("a:{a:?}");
    let s : &[i32] = &a[2..4];
    println!("s: {s:?}");
}