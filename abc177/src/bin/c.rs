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
        n:usize,
        a:[usize;n]
    }
    let MOD = 1000000007;
    let N : usize = n;
    let mut cumulative_sum = Vec::new();
    for i in 0..n {
        cumulative_sum.push(0);
    }

    cumulative_sum[0] = a[0];
    for i in 1..n {
        cumulative_sum[i] = ( a[i] + cumulative_sum[i-1] ) % MOD;
    }
    let mut ans = 0;
    for i in 0..N {
        ans = ( ans + ( a[i] * cumulative_sum[n-1] - cumulative_sum[i]) % MOD); 
    }
    println!("{}", ans );

}
