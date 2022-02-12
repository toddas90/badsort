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

use rand::prelude::*;

pub fn sort<T: Ord>(input: &mut [T]) {
    let mut rng = rand::thread_rng();
    while !is_sorted(input) {
        input.shuffle(&mut rng);
    }
}

fn is_sorted<T: Ord>(input: &[T]) -> bool {
    for i in 0..input.len() - 1 {
        if input[i] > input[i + 1] {
            return false;
        }
    }
    true
}

#[test]
fn small_random() {
    let mut test = vec![4, 2, 1, 5, 3];
    let oracle = vec![1, 2, 3, 4, 5];

    sort(&mut test);

    assert_eq!(oracle, test);
}

#[test]
fn small_same() {
    let mut test = vec![0, 0, 0];
    let oracle = vec![0, 0, 0];

    sort(&mut test);

    assert_eq!(oracle, test);
}
