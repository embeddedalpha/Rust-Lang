fn main() {
    println!("Operators in Rust!");


    /*********************************************** Arithmetic Operators ***********************************************/

    let a:f32 = 5.0;
    let b:f32 = 10.0;

    let mut result:f32;              

    result = a + b;                     // Addition
    println!("{}", result);

    result = a - b;                     //  Substraction
    println!("{}", result);

    result = a / b;                     //  Division
    println!("{}", result);

    result = a * b;                     //  Multiplication
    println!("{}", result);

    result = b % a;                     //  Modulus
    println!("{}", result);

    /*********************************************** Relational Operators ***********************************************/


    if a > b                            //  Greater
    {
        println!("a is greater than b");
    }
    else
    {
        println!("a is smaller than b");
    }

    if a < b                            //  Smaller
    {
        println!("a is greater than b");
    }
    else
    {
        println!("a is smaller than b");

    }



    //Rest are same as C so no need to dive deep into them.








}
