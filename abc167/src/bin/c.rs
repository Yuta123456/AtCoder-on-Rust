use proconio::input;
use std::cmp::min;
fn main() {
    input!{
        n:usize,
        m:usize,
        x:usize,
        text:[[usize;m+1];n]
    }
    let mut ans = 1000000000;
    for bit in 0..(1<<n){
        let mut cost = 0;
        let mut skill = vec![0; m];
        for i in 0..n{
            if (bit>>i) & 1 == 0 {
                continue;
            }
            cost += text[i][0];
            for j in 0..m{
                skill[j] += text[i][j+1];
            }
        }
        let mut flag = true;
        for i in 0..m{
            if skill[i] < x{
                flag = false;
                break;
            }
        }
        if flag {
            ans = min(ans, cost);
        }
    }
    if ans == 1000000000{
        println!("{}", -1);
    }else{
        println!("{}", ans);
    }

}
