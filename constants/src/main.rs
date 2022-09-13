fn main() {
    println!("Constants in Rust!");

    //Use 'const' keyword instead of let for declaring constants
    //Constants in Rust should be in upper case.

    const _CONST_U8: u8 = 90;

    const _CONST_F32: f32 = 89.90;

    println!("Constant u8 = {}", _CONST_U8);

    println!("Constant f32 = {}", _CONST_F32);
}
