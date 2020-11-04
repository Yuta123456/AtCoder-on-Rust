use proconio::input;
use proconio::marker::{Chars};
fn main() {
    input!{
        s:Chars,
        n:usize
    }
    let mut s = s;
    s.sort();
    let mut strings = Vec::new();
    for i in 0..5{
        for j in 0..5{
            strings.push([s[i].to_string(),s[j].to_string()].join(""));
        }
    }
    println!("{}", strings[n-1]);
}
