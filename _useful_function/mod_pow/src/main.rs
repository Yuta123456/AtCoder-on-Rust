fn mod_pow(x:usize, y:usize, m:usize) -> usize{
    let a = x;
    let b = y;
    if b == 0{
        return 1;
    }else if b % 2 == 1 {
        return a * (mod_pow(a, b-1, m) % m);
    }else{
        let k = mod_pow(a, b/2, m) % m;
        return (k * k) % m;
    }
}
fn main(){
    println!("{}", mod_pow(500,1000000000, 1000000007));
}