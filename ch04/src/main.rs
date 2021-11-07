fn main() {
    let s = String::from("Hallo world");
    // get user input
    let mut input = String::new();
    // remove the '\n'
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // convert to uppercase
    let input = input.trim().to_uppercase();
    
}


// STACK
// first in, last out
// pushing and poping to the stack 
// known fixed size when pushing


// HEAP
// size unknown or might change 
// memory allocator finds free space, marks it and returns a pointer 
// process called allocating


// Ownership rules
// Every variable has an owner
// there can be only one owner at a time 
// when owner goes out of scope the value will be dropped

// STRING LITERAL = Hardcoded into the text of our program

