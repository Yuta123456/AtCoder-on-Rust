use proconio::input;
fn main() {
    input! {
        mut a_1: i32,
        mut a_2: i32,
        mut a_3: i32
    }
    if a_1 + a_2 + a_3 >= 22 {
        println!("bust");
    }else{
        println!("win");
    }
}