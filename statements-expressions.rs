# Statements and Expressions

### Examples

fn main() 
{
    let x: u32 = 5_u32;

    let y: u32 = // y vai ser o resultado de toda a expressão dentro dos { } 
    {
        let x_squared = x * x; 
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x // = y // value of y in the end. // omit the ; here to assign this operation result to ' y '
     
        };

    let z: u32= {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x // doesnt return a value to z // ; in the end doesnt return the value to the variable Z
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}


### Exercises
1. 🌟🌟

// Make it work with two ways
fn main() 
{
   let v: i32 = 
   {
       let mut x: i32 = 1;
       x += 2; // x = x + 2 // x= 3 but we cant return a variable assignment
       x // value of X is then assigned to v
   };

   assert_eq!(v, 3); // 2nd way -> jut change the 3 for an unity type
   //assert_eq!(v, ());

   println!("Success!");
}


2. 🌟
```rust,editable

fn main() {
   let v = (let x = 3); // this is a sintaxe error
   // we can do
   // let v: i32 =
   // {
   //     let x = 3;
   //     x
   // };

   assert!(v == 3);

   println!("Success!");
}


3. 🌟

fn main() {
    let s: i32 = sum(1 , 2); // sum = x+y
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 
{
    x+y // right way; without semicolon
    //x + y; // this ';' means nothing will return from this function, the ; ends the operation right there; 
}

> You can find the solutions [here](https://github.com/sunface/rust-by-practice)(under the solutions path), but only use it when you need it