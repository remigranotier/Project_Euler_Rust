fn is_prime(n: i32) -> bool {
    let mut i = 3;
    let mut rbool = true;

    if n == 1 || (n > 2 && n % 2 == 0) {
        false
    } else {
        while i < n {
            if n % i == 0 {
                rbool = false;
                break;
            }
            i += 2;
        }

        rbool
    }
}

fn main() {
    print!("{}", (2..).filter(|x| is_prime(*x)).nth(10000).unwrap());
}
