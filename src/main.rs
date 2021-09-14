/*use passgen::{az, nums, symbs, AZ};
use std::env;
use std::fs;

fn main() {


    println!("{:?}", password);
    println!("{:?}", password);
}*/

mod gen;

fn main(){
    let args = gen::start();
    let pass = gen::gen(args);
    println!("{:?}", pass);
}
