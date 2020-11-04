use proconio::input;
const MOD:i64 = 998244353;
const INF:i64 = 998244360;
fn main(){
    input!{
        a:usize,
        b:usize,
        c:usize,
        d:usize
    }
    let mut dp = vec![vec![INF;d+1];c+1];
    dp[a][b] = 1;
    for i in 0..a{
        for j in 0..b{
            dp[i][j] = 0;
        }
    }
    println!("{}", cul(a, b, c, d, &mut dp));
    
    for i in 0..c+1 {
        for j in 0..d+1 {
            print!("{} ", dp[i][j]);
        }
        println!("");
    }
    
}
fn cul(a:usize, b:usize, y:usize, x:usize, dp: &mut Vec<Vec<i64>>) -> i64{
    if x == 0 || y == 0{
        return 0;
    }
    if dp[y][x] != INF {
        return dp[y][x];
    }
    //println!("{} {}", x, y);
    if y == 3 && x == 2 {
        println!("{} {}", cul(a, b, y-1, x, dp) * (x as i64), cul(a,b, y, x-1, dp) * (y as i64));
    }
    let mut c = (cul(a, b, y-1, x, dp) * (x as i64) + cul(a,b, y, x-1, dp) * (y as i64)) % MOD;
    if !(x == b || y == a){
        c -= x as i64;
    }
    if c < 0 {
        c = 0;
    }
    dp[y][x] = c;
    return dp[y][x];
}
