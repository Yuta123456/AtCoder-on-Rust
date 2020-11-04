use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
fn main(){
    input!{
        s:Chars,
        t:Chars,
    }
    let mut ans = t.len();
    for i in 0..s.len() {
        let mut count_dif_char = 0;
        if i + t.len() > s.len() {
            break;
        }
        for j in 0..t.len() {
            if &s[i + j] != &t[j] {
                count_dif_char += 1;
            }
        }
        if count_dif_char < ans {
            ans = count_dif_char;
        }
    }
    println!("{}", ans);
}
