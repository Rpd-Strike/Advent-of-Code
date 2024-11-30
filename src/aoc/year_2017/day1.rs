use std::fmt::Display;

use crate::utils::{input_lines_iter, split_single_digits};

pub fn solution_one() -> impl Display {
    let digits = input_lines_iter().next().unwrap();
    let mut digits = split_single_digits(digits);
    let n = digits.len();
    digits.push(digits[0]);
    let mut sum = 0;
    for i in 0..n {
        if digits[i] == digits[i + 1] {
            sum += digits[i];
        }
    }

    sum
}

pub fn solution_two() -> impl Display {
    let digits = input_lines_iter().next().unwrap();
    let mut digits: Vec<_> = digits
        .chars()
        .into_iter()
        .map(|c| (c as u32) - ('0' as u32))
        .collect();
    // println!("{digits:?}");
    let n = digits.len();
    digits.push(digits[0]);
    let mut sum = 0;
    for i in 0..n {
        let mut urm = i + n / 2;
        if urm >= n {
            urm -= n;
        }
        if digits[i] == digits[urm] {
            sum += digits[i];
        }
    }

    sum
}
