#![allow(unused)]

use newton::*;

fn main() {
    let arr = [1,2,3];
    let var = 12;
    let res: i32 = arr.iter().zip(arr.iter()).map(|(a, b)| a*b).sum();
    println!("{res}");
}
