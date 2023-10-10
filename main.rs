use std::array;

use permute::{permute_string, find_palindromes};
use graphs::{find_islands};


mod permute;
mod graphs;


fn main() {
    //println!("Hello, world!");
    //println!("{:?}", sum_of_2_ints(vec![1,5,2,9,3], 12));

    //permute::permute_binary(4);
    //permute_string("abcdefghijk".to_string());
    //find_palindromes("aba".to_string());


    let mut a  = &[
            &[1, 1, 0, 0, 0][..],
            &[0, 1, 0, 0, 1][..],
            &[1, 0, 0, 1, 1][..],
            &[0, 0, 0, 0, 0][..],
        ];

    println!("dim: {:?}", a.len());
    find_islands(a);
}

fn sum_of_2_ints(mut input: Vec<i32>, target: i32) -> (i32, i32) {
    input.sort();
    let mut left: usize = 0;
    let mut right: usize = input.len() - 1;
    while left < right {
        if input.get(left).unwrap() + input.get(right).unwrap() == target {
            return (*input.get(left).unwrap(), *input.get(right).unwrap());
        } else if input.get(left).unwrap() + input.get(right).unwrap() > target {
            right -= 1;
        } else {
            left += 1;
        }
    }
    return (-1, -1);
}
