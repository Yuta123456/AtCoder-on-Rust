use proconio::input;
fn main(){
    input!{
        n:usize,
        t:usize,
        time:[usize;n]
    }
    let mut ans = 0;
    let mut last_pass = time[0];
    for i in 1..n {
        if last_pass + t < time[i] {
            ans += t;
        }else{
            ans += time[i] - last_pass;
        }
        last_pass = time[i];
    }
    ans += t;
    println!("{}", ans);
}
