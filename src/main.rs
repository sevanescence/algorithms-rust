use std::{fmt::Debug, collections::VecDeque};

extern crate mcclient;

fn merge_sort<T: Copy + PartialOrd + Debug>(a: Vec<T>) -> Vec<T> {
    if a.len() <= 1 {
        return a;
    }

    let left = merge_sort(a[..a.len()/2].to_vec());
    let right = merge_sort(a[a.len()/2..].to_vec());

    merge(left, right)
}

fn merge<T: Copy + PartialOrd + Debug>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    println!("{left:?} , {right:?}");
    let mut rs: Vec<T> = vec![];
    let mut left = VecDeque::from(left);
    let mut right = VecDeque::from(right);

    while !left.is_empty() && !right.is_empty() {
        if left.front().unwrap() < right.front().unwrap() {
            rs.push(left.pop_front().unwrap());
        }
        else {
            rs.push(right.pop_front().unwrap());
        }
    }

    while !left.is_empty() {
        rs.push(left.pop_front().unwrap());
    }
    while !right.is_empty() {
        rs.push(right.pop_front().unwrap());
    }
    
    rs
}

fn main() {
    let v: Vec<i32> = vec![5, 2, 3, 7, 3, 9, 6, 4, 5];

    println!("{:?}", merge_sort(v));
}
