use proconio::input;
fn main() { 
    input!{
        n:usize,
        k:u64,
        a:[usize;n]
    }
    let mut doubling = vec![vec![0;n]; 62];
    for i in 0..n{
        doubling[0][i] = a[i] - 1;
    }
    for i in 1..61{
        for j in 0..n{
            let next = doubling[i-1][j];
            doubling[i][j] = doubling[i-1][next];
        }
    }
    let mut ans = 0;
    let mut now_k:u64 = k;
    for i in (0..62).rev() {
        if (1_u64 << i) <= now_k{
            now_k -= 1_u64 << i;
            ans = doubling[i][ans];
        }
    }
    println!("{}", ans+1);
}
