# Reference and Borrowing

### Reference
1. 🌟


fn main() 
{
   let x = 5;
   // Fill the blank
   let p: &i32 = &x;
    // accessing data with the pointers
   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}


2. 🌟


fn main() 
{
    let x: i32 = 5;
    let y: &i32 = &x;

    // Modify this line only
    assert_eq!(5, *y); // * to access the value

    println!("Success!");
}


3. 🌟


// Fix error
fn main() 
{
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}


4. 🌟


// Fix error
fn main() 
{
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}


5. 🌟🌟


fn main() {
    let mut s: String = String::from("hello, "); // 's' remain the owner of the data;

    // Fill the blank to make it work
    let p: &mut String = &mut s;
    
    p.push_str("world");

    println!("Success!");
}

#### Ref
`ref` can be used to take references to a value, similar to `&`.

6. 🌟🌟🌟


fn main() 
{
    let c: char= '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c; // ref = &

    assert_eq!(*r1, *r2); //references r1 and r2 should hold the same data;
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String 
{
    format!("{:p}", r)
}

### Borrowing rules
7. 🌟


// Remove something to make it work
// Don't remove a whole line !
fn main() 
{
    let mut s: String = String::from("hello");
// we can only have 1 mutable reference at a time
    let r1 = &s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    println!("s: {}", s);

    println!("Success!");
}


#### Mutability
8. 🌟 Error: Borrow an immutable object as mutable


fn main() {
    // Fix error by modifying this line
    let mut s: String = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}


9. 🌟🌟 Ok: Borrow a mutable object as immutable


// This code has no errors!
fn main() 
{
    let mut s: String = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}


### NLL
10. 🌟🌟


// Comment one line to make it work
fn main()
{
    let mut s: String = String::from("hello, ");

    let r1: &mut String= &mut s;
    r1.push_str("world");
    let r2: &mut String = &mut s;
    r2.push_str("!");
    
    //println!("{}",r1);
}


11. 🌟🌟


fn main() 
{
    let mut s: String = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
}
```

> You can find the solutions [here](https://github.com/sunface/rust-by-practice/blob/master/solutions/ownership/borrowing.md)(under the solutions path), but only use it when you need it