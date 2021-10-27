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
}
