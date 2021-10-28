fn main() {
    let mut x = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let spaces = "    ";
    let spaces = spaces.len();

    println!("{}", spaces);

    let overflow: u8 = 200;
    let overflow = overflow.wrapping_add(57);
    let overflow = overflow.overflowing_add(u8::MAX);

    println!("{:?}", overflow);

    control_flow_fun()
}

// This function return five
fn five() -> i32 {
    5
}

// This function adds one to x and returns it
fn plus_one(x: i32) -> i32 {
    x + 1
}

// Control flow
fn control_flow_fun() {
    let number = 3;

    if number < 5 {
        // do something
    } else if number % 3 == 0 {
        // do more fun
    } else {
        // do something else
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    let mut count = 0;
    // label outer loop 
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn return_value_loop)() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}