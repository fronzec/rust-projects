fn main() {
    // Integers
    let a1: u32 = 1;
    let a2: u8 = 1;

    // Operations
    let c =  a1 + a2;
    let d = a1 - a2;
    let e =  a1 / a2;
    let f = a1 * a2;
    let g = a1 % a2;

    // Floats
    let sf: f32 = 12.56; // single precision
    let df: f64 = 128.12345; // double precision
    // Booleans
    let isOk: bool  = true;
    let isNotOk: bool = false;

    // Tuples
    let tup: (u32, i32) = (123, -1);
    let tup = (123, 6.5, -100);

    // Arrays
    let arr = [1,2,3,4,5];
    let arr2: [u32;3] = [1,2,3];
    ñet arr3: [99;4]; // [99,99,99,99]
}
