fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is devisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("nubmer is not divisible by 4, 3, 2");
    }

    let condition = true;
    let number_two = if condition { 5 } else { 6 };

    println!("The value of number is: {number_two}");
}
