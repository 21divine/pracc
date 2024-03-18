# Ownership

1. 🌟🌟 

fn main() 
{
    
    let x: String = String::from("Hello world");
    //let y: string = x; 
    let y: String = x.clone(); // solution for it. clone x
    println!("{}, {}",x, y); // it will not work; we are trying to acess the variable "x" but we assigned X to Y;
}


2. 🌟🌟

fn main() 
{
    let s1: String = String::from("Hello world");
    let s2: () = take_ownership(s1);

    println!("{}", s2);
}

//fn take_ownership(s: String) // this function wont return anything
                             // unit type ()
//{
   // println!("{}", s);
//}

fn take_ownership(s: string) -> String // this function wont return anything
                             // unit type ()
{
    println!("{}", s);
    s // return the value to the function 
}



3. 🌟🌟

fn main() 
{
    let s: String = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String 
{
    let s: String = String::from("Hello world");
    
    // Convert String to Vec
    let _s = s.as_bytes(); 
    s
}


4. 🌟🌟

// Fix the error without removing any code
fn main() 
{
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)
{
    println!("{}",s)
    
}

5. 🌟🌟 

// Don't use clone ,use copy instead
fn main()
{
    //let x: (i32, i32, (), String) = (1, 2, (), "hello".to_string());
    //solution: 
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}


#### Mutability
Mutability can be changed when ownership is transferred.

6. 🌟


// make the necessary variable mutable
fn main() 
{
    let s = String::from("Hello ");
    
    //let s1 = s;
    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}
```

7. 🌟🌟🌟


fn main() 
{
    let x: Box<i32> = Box::new(5);
    
    let mut y: Box<i32>= Box::new(1)     // update this line, don't change other lines!
    
    *y = 4;
    // * operator = 'ponteiro'
    
    assert_eq!(*x, 5);

    println!("Success!");
}


### Partial move
Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at the same time. Doing this will result in a partial move of the variable, which means that parts of the variable will be moved while other parts stay. In such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.

#### Example

fn main() 
{
    #[derive(Debug)]
    struct Person // our own custom defined type 'Person'
    {
        name: String,
        age: Box<u8>,
    }

    let person: Person = Person 
    {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}


#### Exercises

8. 🌟


fn main() 
{
   let t: (String, String) = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t);
}
```

9. 🌟🌟
```rust,editable

fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (__, __) = __;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
```


> You can find the solutions [here](https://github.com/sunface/rust-by-practice/blob/master/solutions/ownership/ownership.md)(under the solutions path), but only use it when you need it
