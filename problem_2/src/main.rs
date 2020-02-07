fn main() {
    let mut counter = 0;
    let mut fibo_1;
    let mut fibo_2 = 1;
    let mut fibo_current = 1;
    while fibo_current < 4_000_000 {
        fibo_1 = fibo_2;
        fibo_2 = fibo_current;
        fibo_current = fibo_1 + fibo_2;
        if fibo_current%2 == 0 {
            counter += fibo_current;
        }
    }
    print!("{}", counter);
}
