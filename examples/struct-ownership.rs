/*
    AltaModa Technologies, LLC

    struct-ownership example
 */

// This structure contains an 64-bit integer. The entire struct can be stored on the stack.
struct NoHeapElements {
    val: i64,
}


// Rust offers `str` which can be on the stack and `String` which is allocated on the heap. Think of 
//  Rust's `str` as a fixed length char array, and `String` as a dynamically sized char array.
// This structure contains a String which is allocated on the heap. An instance of this struct may be
//   on the stack and `desc` is a reference to the heap-allocated String.
struct AllHeapElements {
    desc: String,
}


fn main () {

    // A struct that does not use the heap
    let n = NoHeapElements { val: 74 };
    println!("\nThe variable `n` is type NoHeapElements");
    println!("`n` and elements of NoHeapElements are on the stack");
    println!("n.val: {}", n.val);

    let a = AllHeapElements {
        desc: "String (not str) is always on the heap".to_string(),
    };
    println!("\nThe variable `a` is type AllHeapElements");
    println!("`a` is on the stack; elements of AllHeapElements are on the heap");
    println!("a.desc = {}", a.desc);

    // This code fails to compile because `a2` has taken ownership of
    //  the struct reference from `a`. Rust refers to this as
    //  ownership _moving_ from `a` to `a2`. Uncommenting these lines
    //  see the "borrow after move" error on compilation.
    // let a2 = a;
    // println!("a.desc = {}", a.desc);

    println!();
}