fn main() {
    let iterator: i32 = (1..1000).filter(|x| x%3==0 || x%5==0).sum();
    print!("{}", iterator);
}
