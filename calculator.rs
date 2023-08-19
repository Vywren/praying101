//calculator.rs
extern crate winapi;
use std::ptr::null_mut as NULL;
use winapi::um::winuser;

fn main(){
    //testcases here for now
    println!("{}",add(4,5));
    println!("{}",subtract(4,5));
    println!("{}",multiply(2,4));
    println!("{}",divide(30,5));
}
fn add(a:i32,b:i32) -> i32{
    let sum: i32 = a + b;
    return sum;
}
fn subtract(a:i32,b:i32) -> i32{
    let end: i32 = a - b;
    return end;
}
fn multiply(a:i32,b:i32) -> i32{
    let product: i32 = a * b;
    return product;
}
fn divide(a:i32,b:i32) -> i32{
    let end: i32 = a / b;
    return end;
}
