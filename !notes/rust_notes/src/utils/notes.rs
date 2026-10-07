use rand::{rng, Rng};

pub fn run() {
    // Variables

    // let x;               |    // const PI: f64 = 3.14;
    // x = '2';             |
    // Immutable (default)  |    // Immutable
    // No Type Declaration  |    // Type Declaration
    // Runtime              |    // Compile-Time
    // Shadowing            |    // No Shadowing

    // DataTypes and Size

    use std::any::type_name_of_val;
    use std::mem::size_of_val;

    let a = 'a';
    println!("{}", type_name_of_val(&a));
    println!("{}", size_of_val(&a));
    let b = true;
    println!("{}", type_name_of_val(&b));
    println!("{}, in Byte(s)", size_of_val(&b));
    let c = "This is sentence.";
    println!("{}", type_name_of_val(&c));
    println!("{}", size_of_val(&c));
    let d = 67;
    println!("{}", type_name_of_val(&d));
    println!("{}", size_of_val(&d));
    let e = 1.0;
    println!("{}", type_name_of_val(&e));
    println!("{}", size_of_val(&e));

    // Conditionals
    // if(true){};
    // to use as ternary op, do not use return and semicolon
    // match

    // Arrays // Fix len, Same Type
    let my_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let _length: usize = my_arr.len();

    // Tuple // Fix len, Multiple Types
    let tuple = ("Arpit", 21, 'A');
    println!("Name: {}\nAge: {}\nGrade: {}", tuple.0, tuple.1, tuple.2);

    // Loops

    loop {
        break;
    }
    while false {}
    for _i in 0..10 {}

    // Strings
    let mut my_string = "Hello World".to_string();
    my_string.push_str("!");
    println!("{}", my_string);

    // Casting === as
    // Emuns

    // Vector
    let my_vec: Vec<i128> = Vec::new();
    let mut my_vec2 = vec![1, 2, 3, 45, 67, 0];
    my_vec.is_empty();
    my_vec2.push(77);

    // Random
    println!("Random Number: {}", rng().random_range(0..=7));

    // Input

    // use std::io;
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("msg on Error");
    println!("Input: \n\"{}\"", input.trim_end());
}
