use std::io;

fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 5;
    println!("The value of the constant is: {THREE_HOURS_IN_SECONDS}");
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x after shadowing is : {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // addition
    let sum = 5 + 10;
    println!("The addition vaue is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The subraction value is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The multiplication value is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The quotient value is: {quotient}");
    let floored = 2 / 3;
    println!("The floored value is: {floored}");

    //remainder
    let remainder = 43 % 5;
    println!("The remainder value is: {remainder}");

    // Boolean Type
    let t = true;
    println!("The t value is: {t}");
    let f: bool = false;
    println!("The f value is: {f}");

    // Character Type
    let c = 'z';
    println!("The c value is: {c}");
    let z: char = 'Z';
    println!("The z value is: {z}");
    let heart_eyed_cat = ')';
    println!("The hec value is: {heart_eyed_cat}");

    // Compound Type

    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The tuple type value is: {:?}", tup);
    let tuptwo = (502, 6.4, 2);
    println!("The tuple 2 type value is: {:?}", tuptwo);

    let (_x, _y, _z) = tuptwo;
    println!("The shadow of tuptwo calling 2nd value is: {_y}");

    // Array Type
    let a = [1, 2, 3, 4, 5];
    println!("The a array values are: {:?}", a);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The months array values are: {:?}", months);

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The b array values are: {:?}", b);
    let c = [0; 5];
    println!("The c array values are: {:?}", c);

    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
