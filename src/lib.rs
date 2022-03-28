// Copyright 2020 Andrew Todd

/*
This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! # Badsort
//!
//! A BOGO Sort implementation I wrote when I was super bored.

#![feature(is_sorted)]

use rand::prelude::*;

/// Sorts the given list using
/// the bogosort algorithm.
///
/// # Examples
///
/// ```
/// use badsort::bogosort;
/// 
/// let mut arr = vec![4, 5, 2, 1, 3];
/// 
/// bogosort(&mut arr);
///
/// assert_eq!(arr[0], 1);
/// ```
pub fn bogosort<T: PartialOrd>(input: &mut [T]) {
    if input.is_empty() {
        return;
    }
    let mut rng = rand::thread_rng();
    
    while !input.iter().is_sorted() {
        input.shuffle(&mut rng);
    }
}

#[test]
fn small_random() {
    let mut test = vec![4, 2, 1, 5, 3];
    let oracle = vec![1, 2, 3, 4, 5];

    bogosort(&mut test);

    assert_eq!(oracle, test);
}

#[test]
fn small_alternating() {
    let mut test = vec![1, 2, 1, 2, 1];
    let oracle = vec![1, 1, 1, 2, 2];

    bogosort(&mut test);

    assert_eq!(oracle, test);
}

#[test]
fn small_same() {
    let mut test = vec![0;5];
    let oracle = vec![0;5];

    bogosort(&mut test);

    assert_eq!(oracle, test);
}

#[test]
fn empty() {
    let mut test: Vec<i32> = Vec::new();
    let oracle: Vec<i32> = Vec::new();
    bogosort(&mut test);
    assert_eq!(oracle, test);
}

#[test]
fn single() {
    let mut test = vec![1];
    let oracle = vec![1];
    bogosort(&mut test);
    assert_eq!(oracle, test);
}

#[test]
fn small_sorted() {
    let mut test = vec![1, 2, 3, 4, 5];
    let oracle = vec![1, 2, 3, 4, 5];

    bogosort(&mut test);

    assert_eq!(oracle, test);
}
