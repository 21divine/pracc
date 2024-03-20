# Tuple
1. 🌟 Elements in a tuple can have different types. Tuple's type signature is `(T1, T2, ...)`, where `T1`, `T2` are the types of tuple's members.


fn main() 
{
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

// Make it work
fn main() 
{
    let t: (&str, &str, &str)= ("i", "am", "sunface");
    assert_eq!(t.2,"sunface");

    println!("Success!");
}


3. 🌟 Long tuples  cannot be printed


// Fix the error
fn main() 
{
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}


4. 🌟 Destructuring tuple with pattern.

fn main() 
{
    let tup: (i32, f64, &str) = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}


5. 🌟🌟 Destructure assignments.

fn main()
{
    let (x, y, z);

    // Fill the blank
    (y, z, x) = (1, 2, 3);
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}


6. 🌟🌟 Tuples can be used as function arguments and return values


fn main() 
{
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2,3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

> You can find the solutions [here](https://github.com/sunface/rust-by-practice/blob/master/solutions/compound-types/tuple.md)(under the solutions path), but only use it when you need it