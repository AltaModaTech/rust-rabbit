/*
    This is a simple example of _ownership_ in Rust.

    stack_vals - demonstrates stack-based values as a comparison for heap_vals.
    heap_vals - demonstrates heap-based values & how ownership of them _moves_.
*/

fn main() {
    stack_vals();
    println!();
    heap_vals();
}

fn stack_vals() 
{
    // Be default, values are put on the stack. The space they consume will be reclaimed
    //  automatically when the function goes out of scope (stack unwinds).

    // Put some values on the stack.
    let a1 = 97;
    let a2 = a1;    // a2 is an independent variable, receiving a copy of a1's value.
    let b = a1 + 1; // notice that `a1` can be accessed for assigning a value to `b`.

    println!("Vars on the stack...");
    println!("a1={}, a2={}, b={}", a1, a2, b);
}

fn heap_vals()
{
    // This code is almost identical to stack_vals except that it puts the values on 
    //  the heap. Rust's ownership rules kick in for heap-allocated memory, and
    //  some variable assignments result in _moving_ ownership of that memoery.

    // Values are put on the stack by default (as in stack_vars). Box allocates to the
    //  heap and gives a smart pointer to the value.
    let a1 = Box::new(97);
    let a2 = a1;    // `a2` takes ownership of the value stored on the heap and
                              // and `a1` is no longer valid.
    // let b = *a1 + 1;     // Uncomment this line to see the "value borrowed here 
                              // after move" error
    let b = *a2 + 1;

    // Since `a1` is no longer valid, it cannot be used for output.
    println!("Vars on the heap...");
    println!("a2={}, b={}", a2, b);
    
    // uncomment this following to see "value borrowed here after move" error
    // println!("a1={}, a2={}, b={}", a1, a2, b);

}
