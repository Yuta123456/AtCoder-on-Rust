use proconio::input;
use std::cmp::max;
fn main(){
    input!{
        n:usize,
        a:[i64;n]
    }
    let mut dp = vec![vec![0;n as usize];n as usize];
    let mut b = a;
    for i in 0..n{
        dp[i][i] = b[i];
    }
    println!("{}", cul(0, n-1, &mut dp, &mut b));
}
fn cul(l:usize, r:usize, dp: &mut Vec<Vec<i64>>, a: &mut Vec<i64>) -> i64{
    if (dp[l][r] != 0){
        return dp[l][r];
    }
    dp[l][r] = max(a[l] - cul(l+1, r,dp,a), a[r] - cul(l, r-1, dp, a));
    return dp[l][r];
}