fn main() {
    let racine: u64 = 600851475143_f32.sqrt() as u64;
    let mut current: u64 = 600851475143;
    let mut maxdiv = 1;
    for i in 2..(racine+1) {
        while current%i==0 {
            current /= i;
            maxdiv = i;
        }
    }
    print!("{}", maxdiv);
}
