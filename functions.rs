# Functions
1. 🌟🌟🌟


fn main() 
{
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s: i32 = sum(x, y);
    // s = x + y

    assert_eq!(s, 3);

    println!("{}", s);
    println!("Success!");
}

fn sum(x: i32, y: i32) 
{
    x + y
}



2. 🌟

fn main() 
{
   print();
}

// Replace i32 with another type
fn print() 
{
   println!("Success!");
}



3. 🌟🌟🌟


// Solve it in two ways
// DON'T let `println!` work
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! 
{
    // Implement this function, don't modify the fn signatures
    panic!()   
}


### Diverging functions 
Diverging functions never return to the caller, so they may be used in places where a value of any type is expected.

4. 🌟🌟

fn main() 
{
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! 
{
    panic!()    
}


5. 🌟🌟

fn main() 
{
    // FILL in the blank
    
    //let b = __;
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}


> You can find the solutions [here](https://github.com/sunface/rust-by-practice/blob/master/solutions/basic-types/functions.md)(under the solutions path), but only use it when you need it
