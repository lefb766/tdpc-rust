#[derive(Copy, Clone)]
struct G(u64);

const DIV: u64 =  1000000007u64;

impl std::ops::Add for G {
    type Output = G;
    
    fn add(self, other: G) -> G {
        let G(a) = self;
        let G(b) = other;
        
        G((a + b) % DIV)
    }
}

pub fn f(k: u64, n: u64) -> u64 {
    // assume 0-based indexing
    let n = n - 1;
    
    let mut m = &mut [G(1); 1000][..(k as usize)];
    
    for x in k..(n+1) {
        // We have a_{x-1} calculated.
        let sum = m.iter().fold(G(0), |a, b| a + *b);
        
        let index = x % k;
        m[index as usize] = sum;
    }
    
    let G(ret) = m[(n % k) as usize];
    
    ret
}

#[cfg(test)]
mod tests {
    use super::f;
    
    #[test]
    fn sample1() {
        assert_eq!(55, f(2, 10));
    }
    
    #[test]
    fn sample2() {
        assert_eq!(105, f(3, 10));
    }
}
