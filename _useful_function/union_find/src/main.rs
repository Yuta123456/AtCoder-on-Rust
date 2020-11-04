struct UnionFind {
    par:Vec<usize>,
    rank:Vec<usize>
}
impl UnionFind {
    fn init(n :usize) -> UnionFind {
        let mut array = vec![0;n];
        for i in 0..n {
            array[i] = i;
        }
        UnionFind {
            par:array,
            rank:vec![0;n]
        }
    }
    fn find(&mut self, x:usize) -> usize {
        if x == self.par[x] {
            return x;
        } else {
            let par = self.par[x];
            let res = self.find(par);
            self.par[x] = res;
            return res;
        }
    }
    fn same(&mut self, a:usize, b:usize) -> bool {
        return self.find(a) == self.find(b);
    }

}
fn main() {
    
}
