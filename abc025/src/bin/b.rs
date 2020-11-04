use proconio::input;
fn main() {
    input!{
        n:usize,
        a:isize,
        b:isize,
        query:[(String, isize);n]
    }
    let mut cur_position :isize= 0;
    for i in 0..n{
        let direction = &query[i].0;
        let distance = &query[i].1;
        if *direction == "East"{
            if *distance < a {
                cur_position += a;
            }else if *distance <= b{
                cur_position += *distance;
            }else{
                cur_position += b;
            }
        }else{
            if *distance < a {
                cur_position -= a;
            }else if *distance <= b{
                cur_position -= *distance;
            }else{
                cur_position -= b;
            }
        }
    }
    if cur_position < 0{
        println!("West {}", cur_position*(-1));
    }else if cur_position == 0{
        println!("0");
    }else{
        println!("East {}", cur_position);
    }
}
