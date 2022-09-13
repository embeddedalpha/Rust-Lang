fn main() {
    println!("Learn about Rust Variables!");

    /************************************************ Integers ************************************************/

    // Types of integers and how to declare them:

    let _var_u8:u8    = 1;    //Unsigned 8 bit 
    let _var_u16:u16  = 2;    //Unsigned 16 bit 
    let _var_u32:u32  = 3;    //Unsigned 32 bit 
    let _var_u64:u64  = 4;    //Unsigned 64 bit 
    let _var_arch:usize = 5;   //Takes the size of architecture the code is running on

    
    let _var_i8:i8    = 6;    //Signed 8 bit 
    let _var_i16:i16  = 7;    //Signed 16 bit 
    let _var_i32:i32  = 8;    //Signed 32 bit 
    let _var_i64:i64  = 9;    //Signed 64 bit 
    let _var_arch:isize = 10;   //Takes the size of architecture the code is running on


    /************************************************ Float ************************************************/
    
    //Float can be of 2 types, f32 and f64

    let _var_f32:f32 = 1.4;
    let _var_f64:f64 = 2.8;

    //Above variables are immutable, once declared, they cannot be changed. Variables in rust are immutable by default.
    //They can be made mutable by adding the keyword mut before the declaration.

    let mut _var_mut_u8:u8    = 1;    //mutable unsigned 8 bit 

    _var_mut_u8 = 45;


}
