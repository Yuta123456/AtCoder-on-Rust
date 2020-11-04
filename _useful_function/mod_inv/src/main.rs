fn extgcd(a:usize, b:usize) -> i32{
    let mut r:[i32;3] = [1,0,a as i32];
    let mut w:[i32;3] = [0,1,b as i32];
    while w[2] != 1 {
        let q:i32 = r[2] / w[2];
        let r2 = w;
        let w2 = [r[0]-q*w[0],r[1]-q*w[1],r[2]-q*w[2]];
        r = r2;
        w = w2;
    }
    return w[0];
}
fn mod_inv(a:usize, m:usize) -> usize{
    let x = extgcd(a,m);
    let m = m as i32;
    return ((m + x % m) % m) as usize ;
}
fn main(){
    let m = 1000000007;
    println!("{}", (90 * mod_inv(2, m) % m));
}
