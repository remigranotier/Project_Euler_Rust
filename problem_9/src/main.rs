fn calcul() -> u32 {
    let res;
    for a in 1..1001 {
        for b in 1..(1001-a) {
            for c in 1..(1001-a-b) {
                if a*a + b*b == c*c && a+b+c == 1000 {
                    res = a*b*c;
                    println!("{} {} {}", a, b, c);
                    return res;
                }
            }
        }
    }
    0
}

fn main() {
    println!("{}", calcul());
}
