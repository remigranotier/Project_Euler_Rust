fn main() {
    let sum: i32 = (1..101).sum();
    let square_of_sum: i32= sum.pow(2);
    let sum_of_squares: i32 = (1..101).map(|x| x*x).sum();
    println!("{}", square_of_sum- sum_of_squares);
}
