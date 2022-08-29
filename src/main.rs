fn main() {
    // Numbers from 1 to 25
    for num in 1..26 {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{:?}", num)
        }
    }
}
