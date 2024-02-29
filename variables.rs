# Variables

### Binding and mutability
1. 🌟 A variable can be used only if it has been initialized.


// Fix the error below with least amount of modification to the code
#[allow(unused_variables)];
fn main() 
{
    let x: i32 = 5; 
    let y: i32; 

    assert_eq!(x, 5);
    println!("Success!");
}


2. 🌟 Use `mut` to mark a variable as mutable.

fn main() 
{
    let mut x = 1;
    x += 2; // x = x + 2  // x = 3;
    
    assert_eq!(x, 3);
    println!("Success!");
}


### Scope
A scope is the range within the program for which the item is valid.

3. 🌟 

fn main() 
{
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}


4. 🌟🌟 


// Fix the error with the use of define_x
fn main() 
{
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{}, Xico", x);
}


### Shadowing
You can declare a new variable with the same name as a previous variable, here we can say **the first one is shadowed by the second one.**

5. 🌟🌟 


// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() 
{
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

6. 🌟🌟 

// Remove a line in the code to make it compile
fn main() 
{
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let x = x; //REMOVED THIS LINE OVERHERE
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}


### Unused variables
7. Fix the warning below with :

- 🌟  Only one solution
- 🌟🌟  Two distinct solutions

> Note: none of the solutions is to remove the line `let x = 1` 

fn main() 
// 1st solution 
#[allow(unused_variables)]
{
    let x = 1; 
}
// 2nd solution
{
    let _x = 1;
}


### Destructuring
8. 🌟🌟 We can use a pattern with `let` to destructure a tuple to separate variables.

> Tips: you can use Shadowing or Mutability

// Fix the error below with least amount of modification
fn main() 
{
    let (mut x, y) = (1, 2);
    x += 2; // x = 3;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Xico!");
}


### Destructuring assignments
Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.

9. 🌟🌟

> Note: the feature `Destructuring assignments` need 1.59 or higher Rust version

```rust,editable

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y],[3,2]);

    println!("Xico!");
} 

