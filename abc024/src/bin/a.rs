use proconio::input;
fn main(){
    input!{
        a:usize,
        b:usize,
        c:usize,
        k:usize,
        s:usize,
        t:usize
    }
    let mut ans = a*s + b*t;
    if s + t >= k{
        ans -= (s + t) * c;
    }
    println!("{}", ans);
}
