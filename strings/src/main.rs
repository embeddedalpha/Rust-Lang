fn main() {
    println!("Lets take a look at strings!");

    /*
    There are two types of strings in Rust, literal and objects.
    String Literals are used when the size of string is known at
    compile time.
     */
    
    let first_name:&str = "Kunal";
    let last_name:&str = "Salvi";
    let location:&str = "Arlington,Texas";

    print!("My name is {} {} and",first_name,last_name);
    println!("and I live in {}",location);

    /*
    String Object:
    * Not a part of core language
    * Defined in public structure in standard library pub struct String
    * String Objects are mutable and UTF-8 encoded
    * Allocated in heap
     */


     //Create an empty string 
     let empty_string = String::new();
     println!("Empty String with length {}",empty_string.len());


     //Create a string with a default value:
     let default_string = String::from("Default");
     println!("Default String with length {}",default_string.len());


     /*
     Common Methods:
     1.  new()                           Creates a new string
     2.  to_string()                     Converts the given value to a String
     3.  replace()                       Replace all matches of a pattern with another string
     4.  as_str()                        Extracts  a string slice containing the entire string
     5.  push()                          Appends the given character to the end of the string
     6.  push_str()                      Appends a given string slice onto the end of a string
     7.  len()                           Returns the length of the string in bytes
     8.  trim()                          Returns a string slice with leading and trailing whitespace removed.
     9.  split_whitespace()              Spilts a string splice by whitespace and returns an iterator.
     10. split()                         Returns an iterator over substring of this string, separated by characters matched by a pattern.
     11. chars()                         Returns an iterator over the chars of a string slice.
     */

     /********************************************* Method: New **********************************************/

     let mut var_string_new =   String::new();

     /********************************************* Method: push_str **********************************************/

     var_string_new.push_str("String pushed in variable");
     println!("{}",var_string_new);

    /********************************************* Method: len **********************************************/

    let var_length = var_string_new.len();
    println!("Length = {}",var_length);

    /********************************************* Method: to_string **********************************************/

    let mut _var_string_literal = "Hello my name is Kunal!".to_string();
    println!("String: {}",_var_string_literal);
    println!("Length = {}",_var_string_literal.len());

    _var_string_literal.push_str(" I live in Texas");
    println!("Pushed String = {}",_var_string_literal);

    /********************************************* Method: replace **********************************************/

    let _var_replace = _var_string_literal.replace("Kunal", "Ziran");
    println!("{}",_var_replace);

    /********************************************* Method: push **********************************************/

    let mut _var_push = String::from(" I have apple");
    println!("{}",_var_push);
    _var_push.push('s');
    _var_push.push(' ');
    println!("{}",_var_push);

    /********************************************* Method: trim **********************************************/

    println!("Untrimmed = {}",_var_push);
    println!("Untrimmed length = {}",_var_push.len());

    println!("Trimmed = {}",_var_push.trim());
    println!("Trimmed length = {}",_var_push.trim().len());

    /********************************************* Method: split_whitespace **********************************************/

    let mut i:u8 = 1;
    
    for token in _var_push.split_whitespace(){
        println!("token {} = {}",i,token);
        i += 1;
    }

    /********************************************* Method: split **********************************************/

    let mut _var_split_string = String::new();

    _var_split_string.push_str(" Kunal, Ziran, Hasnain, Umang");
    for token in _var_split_string.split(","){
        println!("{}",token);
    }


    let tokens:Vec<&str> = _var_split_string.split(",").collect();
    println!("First Name = {}",tokens[0]);


    /********************************************* Method: chars **********************************************/

    for n in _var_split_string.chars(){
        println!("{}",n);
    }

}

