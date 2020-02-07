fn main() {
    let mut i = 1;
    loop {
        let mut boole = true;
        for j in 1..21 {
            if i%j != 0 {
                boole = false;
                break;
            }
        }
        if boole {
            println!("{}", i);
            break;
        }
        i+=1;
    }
}
