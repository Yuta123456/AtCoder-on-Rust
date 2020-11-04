use proconio::input;
fn main(){
    input!{
        d:usize,
        t:usize,
        s:usize
    }
    if d > t * s {
        println!("No");
    }else{
        println!("Yes");
    }

}
