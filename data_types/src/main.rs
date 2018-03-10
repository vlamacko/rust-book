fn main() {
    // NOTE: _ is prepended to all variables to avoid unused variable warnings. 

    /* Type inference
     * - Rust is statically typed language.
     * - type does not need to be always annotated.
     * - if multiple types are possible, you must annotate the type.
     */

    let _x = 42; // Type infered. 
    let _x: u32 = "42".parse().expect("Not a number!");

    /* Scalar types
     * - represents a single value.
     * - four primary scalar types:
     *      - integers
     *      - floats
     *      - Booleans
     *      - characters
     */
    
    // Integers - all types and literals

    let _x: i8 = -10;
    let _x: u8 = b'A';
    
    let _x: i16 = -0xff;
    let _x: u16 = 0xff;

    let _x: i32 = -0o77; // Default because it is generally fastest.
    let _x: u32 = 0o77;

    let _x: i64 = -0b1111_0000;
    let _x: u64 = 0b1111_0000;

    let _x: isize = 1_234_456;
    let _x: usize = 1_234_456;

    // Floating-point types

    let _x = 2.0;      // f64; default because of more precision.
    let _y: f32 = 3.0; // f32

    // Numeric operations

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    // The Boolean type

    let _t = true; // implicit type
    let _f: bool = false; // explicit annotation

    // The Character type

    let _c = 'z';
    let _z = 'Z';
    let _heart_eyed_cat = '😻'; // Unicode support present; not ASCII only.

    /* Compound types
     * - two primitives: tuples and arrays.
     */

    // Tuples 

    let tup: (i32, f64, u8) = (500, 6.4, 1); // It is possible to mix types.
    
    let (_x, y, _z) = tup; // Deconstructing a tuple value.
    println!("The value of y is: {}", y);

    let _five_hundred   = tup.0; // Accessing a tuple element directly.
    let _six_point_four = tup.1; // Tuples are zero indexed.
    let _one            = tup.2;

    // Arrays - always fixed in length like C. 

    let a = [1, 2, 3, 4, 5]; // All elements must be of same type.
    
    let _first  = a[0];
    let _second = a[1];

    // `let element = a[10]` will result in an out of bounds error at runtime.
}
