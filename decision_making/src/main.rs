fn main() {
    println!("Hello, world!");


    let i = 5;

    if i <= 10
    {
        println!("5 is less than 10");
    }


    if i < 3
    {
        println!("i is less than 3")
    }
    else
    {
        println!("i is greater than 3");
    }

    //Nested If-else

    if i < 3
    {
        println!("{}",i);
    }
    else if i > 3
    {
        println!("{}",i);

    }

    //Match Statement

    let state_code = "MH";
    let state = match state_code {
       "MH" => {println!("Found match for MH"); "Maharashtra"},
       "KL" => "Kerala",
       "KA" => "Karnadaka",
       "GA" => "Goa",
       _ => "Unknown"
    };
    println!("State name is {}",state);

    let state_next = "FIRST";

    let state_current = match state_next{

        "FIRST" => {println!("First"); "First"},
        "SECOND" => "Second",
        _   => "Unkown"
    };

    println!("State = {}", state_current);



}
