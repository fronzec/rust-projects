fn main() {
    // if
    let a = 4;
    if a > 5 {
        print!("Condition is true");
    } else {
        println!("Condition is false");
    }

    // if, else and else if
    let b = 7;
    if b%2 == 0 {
        println!("Number is divisible by 2");
    } else if b%3 == 0 {
        println!("Number is divisible by 3");
    } else if b%4 == 0 {
        println!("Number is divisible by 4");
    } else {
        println!("Number is not divisible by 2,3,4");
    }

    // If expression on a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);

    iterations();
}

fn iterations() {
    let m = [10,20,30,40,50];

    // Iterate using a while
    let mut index = 0;
    while index < 5 {
        println!("The value is {}", m[index]);
        index += 1;
    }

    // Iterate using a for loop
    for item in m.iter() {
        println!("The value is {}", item);
    }

    // Using range to iterate in countdown mode
    for number in (1..5).rev() {
        println!("Countdown value is {}",number);
    }
}
