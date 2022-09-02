fn main() {
    let number = 3;

    if number % 4 == 0{
        println!("число делится на 4");
    }else if number % 3 == 0{
        println!("число делится на 3");
    }else if number % 2 == 0{
        println!("число делится на 2");
    }else{
        println!("Число не делится на 4, 3 и 2");
    }

    let condition = true;
    let number = if condition {
        5
    }else {
        6
    };
    println!("number = {}", number);

    
}
