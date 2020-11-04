use proconio::input;
use std::cmp::min;

const INF : i64 = 10010010010010;
fn main(){
    input!{
        n:usize,
        a:[i64;n]
    }
    let mut slime_acc = vec![0;n+1];
    slime_acc[1] = a[0];
    for i in 2..n+1{
        slime_acc[i] += slime_acc[i-1] + a[i-1];
    }
    let mut dp = vec![vec![-1;n];n];
    for i in 0..n{
        dp[i][i] = 0;
    }
    println!("{}", cul(&mut slime_acc, &mut dp, 0, n-1));
    /*
    for i in 0..n{
        for j in 0..n{
            print!("{} ", dp[i][j]);
        }
        println!("");
    }
    */
}
fn cul(slime_acc:&mut Vec<i64> , dp:&mut Vec<Vec<i64>>, l:usize, r:usize) -> i64{
    if (dp[l][r] != -1){
        return dp[l][r];
    }
    let mut c = INF;
    for i in l..r {
        //println!("{}", cul(slime_acc, dp, l, i) + cul(slime_acc, dp, i+1, r) + slime_acc[r+1] - slime_acc[l]);
        c = min(c, cul(slime_acc, dp, l, i) + cul(slime_acc, dp, i+1, r) + slime_acc[r+1] - slime_acc[l]);
    }
    dp[l][r] = c;
    return dp[l][r];
}