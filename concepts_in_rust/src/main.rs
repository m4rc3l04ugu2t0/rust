const _GLOBAL_SCOPE: &str = "Foo";

fn main() {
    let mut x = 5;
    println!("{}", x);
    x = 10;
    print!("{}", x);
    let x = 100_000;
    print!("Variable shadow: {}", x);

    const _VARIABLE_CONST: &str = "Foo";

    // Scalar Types
    // Integers
    // Floating-point number
    // Booleans
    // Character

    let _a =  98_222; // Decimal 
    let _b = 0xff; // Hex
    let _c = 0o77; // Octal
    let _d = 0b1111_0000; // Binary
    let _e = b'A';

    // Compaund Types
    let tup = ("One Piece", 1000);
    let (_anime, _num) = tup;
    let _the_best = tup.0;

    let array_kkk = ["Luffy", "Zoro", "Sanji"];
    let _zoro = array_kkk.get(1);
    let _numbers = [0;8];
    let _resulr_fn = my_fn(10, 10);
}

fn my_fn(x: u32, y: u32) -> u32 {
    println!("My Fn");
    let mut sum = x + y;
    println!("{} + {} = {}", x, y, sum);

    // Control Flow
    if sum > 100 {
        return x - y;
    } else if sum == 0 {
        sum = 100;
    } else {
        sum -= 10;
    }

    let condition = true;
    let _number = if condition { 5 } else { 10 };

    sum
}