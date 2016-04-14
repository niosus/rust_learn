// we MUST define types for functions
fn print_sum(x: i32, y: i32) -> i32 {
    println!("x + y = {}", x + y);
    // we can just write `return x + y;` here.
    // Considered poor style in rust if used in the end of fn.
    x + y // rust complains if there is no return on at least one execution path
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1 // there is no semicolon here! This is a statement.
          // it is fine. No return needed here.
}

fn main() {
    let (x, y) = (1, 2);
    println!("x = {}, y = {}", x, y);
    println!("shadowing parameters");
    let (x, y) = ("hello", "world");
    println!("x = {}, y = {}", x, y);

    let mut c = 10;
    println!("c = {}", c);
    c = 20;
    println!("changed mutable c = {}", c);

    let c = 30;
    println!("shadowing to immutable c = {}", c);
    // c = 40; // this becomes illigal after shadowing

    // let k; // we need to tell what type is the implicit const
    let k: i32; // we get a warning that there is a unused binding
    // println!("print k: {}", k); // nope, cannot use unitialized binding

    k = 10; // first binding - all fine
    println!("print k: {}", k);
    // k = 20; // assinging immutable binding

    let sum = print_sum(k, c);
    println!("print sum: {}", sum);

    let z = add_one(k);
    println!("print z: {}", z);

    // bind pointer to function:
    let f = add_one;
    println!("six = {}", f(5));

    // arrays are cool
    let a = [1, 2, 3];
    println!("length a = {}, a[1] = {}", a.len(), a[1]);
    // define size and initial value
    let a = [0; 20];
    println!("length a = {}, a[1] = {}", a.len(), a[1]);

    // slicing!
    let a = [1, 2, 3, 4, 5];
    let middle = &a[2..4];
    println!("printing slice");
    for n in middle {
        print!("{} ", n);
    }
    println!("");

    // and tupples
    let x = ("foo", "bar", "buzz");
    println!("x = ({}, {}, {})", x.0, x.1, x.2);
}
