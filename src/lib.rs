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

use rand::*;
use rayon::prelude::*;

#[allow(unused_assignments)]
pub fn bubble_sort(lst: & Vec<i32>) -> Vec<i32> {
    let mut clean_pass = false;
    let mut sorted: Vec<i32> = Vec::new();
    sorted = lst.to_vec();
    for i in 1..sorted.len() {
        if clean_pass {
            break;
        } else {
            clean_pass = true;
            let mut j = 0;
            while j < (sorted.len() - i) {
                if sorted[j] > sorted[j + 1] {
                    sorted.swap(j, j+1);
                    clean_pass = false;
                }
                j+= 1;
            }
        }
    }
    sorted
}

pub fn merge_sort_par(lst: &Vec<i32>) -> Vec<i32> {
    if lst.len() < 2 {
        lst.to_vec();
    }
    let mid = (lst.len() + 1) /2;
    let (left, right) = lst.split_at(mid);

    let (left, right) = rayon::join(|| merge_sort(& left.to_vec()),
                                    || merge_sort(& right.to_vec()));

    let halfsort = [left, right].concat();
    let sorted = merge(& halfsort, mid);

    sorted
}

pub fn merge_sort(lst: & Vec<i32>) -> Vec<i32> {
    if lst.len() < 2 {
        return lst.to_vec();
    }
    let mid = (lst.len() + 1) / 2;
    let (left, right) = lst.split_at(mid);

    let left = merge_sort(& left.to_vec());
    let right = merge_sort(& right.to_vec());

    let halfsort = [left, right].concat();
    let sorted = merge(& halfsort, mid);

    return sorted;
}

fn merge(lst: & Vec<i32>, mid: usize) -> Vec<i32> {
    let size = lst.len();
    let mut left_index = 0;
    let mut right_index = mid;
    let mut sorted: Vec<i32> = Vec::new();

    for _i in 0..size {
        if right_index >= size {
            sorted.push(lst[left_index]);
            left_index += 1;
        } else if left_index >= mid {
            sorted.push(lst[right_index]);
            right_index += 1;
        } else if lst[left_index] < lst[right_index] {
            sorted.push(lst[left_index]);
            left_index += 1;
        } else {
            sorted.push(lst[right_index]);
            right_index += 1;
        }
    }
    return sorted;
}

pub fn bogo_sort(lst: & Vec<i32>) -> Vec<i32> {
    let mut sorted = Vec::new();
    sorted.clone_from(lst);
    while is_sorted(& sorted) == false {
        shuffle(&mut sorted);
        // println!("Shuffled vec: {:?}", sorted);
    }
    return sorted;
}

fn is_sorted(lst: & Vec<i32>) -> bool {
    for i in 0..lst.len() - 1 {
        if lst[i] > lst[i + 1] {
            return false;
        }
    }
    return true;
}

fn shuffle(lst: &mut Vec<i32>) {
    let n = lst.len();
    for i in 0..n {
        lst.swap(i, thread_rng().gen::<usize>() % n);
    }
}