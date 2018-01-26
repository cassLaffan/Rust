const MAX_POINTS: u32 = 100_000;//have to be in camel case apparently
//toying with variables
//veriables in this language are by default immutable
fn main() {
    println!("The constant in this program is: {}", MAX_POINTS);

    let x = 5;
    println!("The value of x is {}", x);

    let x = x + 1;;
    println!("The value of x is {}", x);
    //testing shadowing

    let x = x + 1;

    println!("The new value of x is: {}", x);

    let spaces = "    ";//is a string

    let spaces = spaces.len();//returns an int

    println!("The length of the spaces variable is: {}", spaces);
    //messing with typing
    //u32 is an integer type
    //u indicates that its unsigned, signed starts with i
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("Unsigned int is {}", guess);

    let a = 2.4;
    let b: f32 = 3.0;//f32, a floating type that has 32 bits, single percision
                    //defaults to 32
    //will only print the decimal if it's not 0
    println!("Floating type number, 64 bits: {}, 32 bits: {}", a, b);

    //basic math in Rust

    let sum = a + b;

    println!("The sum of {} and {} is {}", a, b, sum);

    let different = a - b;

    println!("The difference of {} and {} is {}", a, b, different);

    let quotient = a / b;

    println!("The quoetient of {} divided by {} is {}", a, b, quotient);

    let remainder = a % b;

    println!("The remainder of {} divided by {} is {}", a, b, remainder);

    let t = true;

    let f: bool = false;
    //don't need brackets
    if f{
        println!{"t is {}", t}
    }
    else{
        println!("f is {}", f);
    }

    //fun with tuples

    let tup: (i32, f32, u8) = (500, a, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = x.0;

    let a_point_four = a;

    let one = z.1;

    println!("{}, {}, {}", five_hundred, a_point_four, one);

    //interesting! So a actually defaulted to an f32, unlike what I thought
    //it defaulted to
    //if I declare it as a f64 in its declaration, we get errors saying that
    //Rust cannot perform operations on a 64 and 32 bit float together


}
