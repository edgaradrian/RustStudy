fn main() {
    //Integers
    println!("***** Integers *****");
    let my_first_int32: i32 = 100;
    println!("my First Int 32: {}",my_first_int32);
    let my_first_u_int32: u32 = 1000;
    println!("my First UInt 32: {}",my_first_u_int32);

    //Floats
    println!("***** Floats ******");
    let my_first_float32: f32 = 100.101;
    println!("My first float 32 is {}",my_first_float32);
    let my_first_float64: f64 = 1000.64;
    println!("My first float 64 is {}",my_first_float64);

    //Numeric Operations
    println!("***** Adition *****");
    let sum = 1 + 2 + 3 + 4 + 5;
    println!("the sum 1 + 2 + 3 + 4 + 5: {}",sum);

    println!("***** Subtraction *****");
    let difference = 104 - 40;
    println!("The difference 104 - 40: {}",difference);

    println!("***** Multiplication *****");
    let product = 8 * 4;
    println!("The product 8 * 4: {}",product);

    println!("***** Division *****");
    let quotient = 32.1 / 5.0;
    println!("The quotient {}",quotient);

    println!("***** Remainder *****");
    let remainder = 43 % 5;
    println!("The remainder is {}",remainder);

    //Booleans
    println!("***** Booleans *****");
    let t = true;
    println!("The t value is: {}",t);
    let f: bool = false;
    println!("The f value is: {}",f);

    //Character
    println!("***** Character *****");
    let c = 'z';
    let z = 'Z';
    println!("The character c is {}",c);
    println!("The character z is: {}",z);

    //Tuples
    println!("***** Tuples *****");
    let my_first_tuple: (i32, f32, u8) = (276, 10.58, 0);
    let (x, y, z) = my_first_tuple;
    println!("the Tuple is ({}, {}, {})",x,y,z);

    let point = (1, 2, 3);
    println!("point.x: {}", point.0);
    println!("point.y: {}", point.1);
    println!("point.z: {}", point.2);

}//main
