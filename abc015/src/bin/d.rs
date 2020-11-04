use std::cmp::max;
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input!{
        w:i64,
        n:i64,
        k:i64,
        data:[(i64,i64);n]
    }
    let mut dp = [[[0;51];100001];51];
    for i in 1..n+1{
        let (width, value) = data[(i-1) as usize];
        println!("{} {}", width, value);
        for j in 0..w+1{
            for l in 1..k+1{
                if j - width >= 0{
                    dp[i as usize][j as usize][l as usize] = max(dp[(i-1) as usize][(j - width) as usize][(l-1)  as usize] + value, dp[(i-1) as usize][j as usize][l as usize])
                }else{
                    dp[i as usize][j as usize][l as usize] = max(dp[(i-1)  as usize][j as usize][l as usize],dp[i as usize][j as usize][l as usize])
                }
            }
        }
    }
    println!("{}", dp[n as usize][w as usize][k as usize]);
}
