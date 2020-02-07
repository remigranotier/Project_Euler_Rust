fn main() {
    let mut maxi = 1;
    for i in 1..1000 {
        for j in 1..1000 {
            let first_string = (i*j).to_string();
            let second_string: String = first_string.chars().rev().collect();
            if first_string.eq(&second_string) && i*j > maxi {
                maxi = i*j;
            }
        }
    }
    println!("{}", maxi);
}
