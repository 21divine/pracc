# Char, Bool and Unit

### Char
1. 🌟

// Make it work
use std::mem::size_of_val; // size_of_val measues a size of bytes of specific variable
fn main() 
{
    let c1: char = 'a'; // c1 -> variable of type 'char'
    assert_eq!(size_of_val(&c1),4); 
    //println!("{}", size_of_val(&c1)); // -> will print 4 (value of 'a' on ASCII table)
    let c2 = '中';
    //println!("{}", size_of_val(&c2));
    assert_eq!(size_of_val(&c2),4); 

    println!("Xico, i did it!!");
} 


2. 🌟


// Make it work
fn main() 
{
    let c1: char = '中'; // just changed the " " for ' '. ' ' are for char. " " are for strings
    print_char(c1);
} 

fn print_char(c : char) 
{
    println!("{}", c);
}


### Bool
3. 🌟


// Make println! work
fn main() {
    let _f: bool = false; // f: hols a value of false

    let t = true;
    if t 
    {
        println!("Success!");
    }
} 


4. 🌟


// Make it work
fn main() 
{
    let f: bool = false;
    let t: bool = true && false;
    assert_eq!(t, f);

    println!("Success!");
}



### Unit type
5. 🌟🌟


// Make it work, don't modify `implicitly_ret_unit` !
fn main() 
{
    let _v: () = (); // type () = unity type

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit()); // needeed to underscore "_" v to make it work. to return an unity type;

    println!("Success!");
}

fn implicitly_ret_unit() 
{
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}


6. 🌟🌟 What's the size of the unit type?


// Modify `4` in assert to make it work 

use std::mem::size_of_val;
fn main() 
{
    let unit: () = (); //unit = ()
    assert!(size_of_val(&unit) == 0); // unity types are of size 0.

    println!("Success!");
}


> You can find the solutions [here](https://github.com/sunface/rust-by-practice)(under the solutions path), but only use it when you need it
