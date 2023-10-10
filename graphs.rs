use std::array;

use ndarray::{array, Array, Array2};


pub fn find_islands(input: &[&[i32]]) {
    //println!("dim: {:?}", input.shape());
    //println!("{:?}", input);
    for row in 0..input.len() {
        let z = input.get(row).unwrap();
        for col in 0..z.len() {
            print!("{}", input[row][col]);
        }
        println!("");
    }
}
