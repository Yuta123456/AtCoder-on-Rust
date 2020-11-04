use std::collections::HashSet;
use std::collections::VecDeque;
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
        n:usize,
        data:[(usize,usize,usize);n],
    }
    let mut adjacent_list = Vec::new();
    for _i in 0..n+1{
        let mut d = Vec::new();
        adjacent_list.push(d);
    }
    for i in 0..n{
        let (x,y,w) = data[i];
        adjacent_list[x].push((y,w));
        adjacent_list[y].push((x,w));
    }
    let mut finished:HashSet<usize> = HashSet::new();
    let mut queue = VecDeque::new();
    let mut ans = vec![-1;n+1];
    queue.push_front((1,0));
    while let Some((node, weight)) = queue.pop_front(){
        if finished.contains(&node){
            continue;
        }else{
            if weight % 2 == 0{
                ans[node] = 1;
            }else{
                ans[node] = 0;
            }
            for (next, w) in &adjacent_list[node]{
                queue.push_back((*next, *w));
            }
            finished.insert(node);
        }
    }
    for _i in 1..n+1{
        println!("{}",ans[_i]);
    }
}