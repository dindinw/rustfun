//
// Rust is a statically typed language: without actually running the program, the compiler checks
// that every possible path of execution will use values only in ways consistent with their types.
// This allows Rust to catch many programming mistakes early, and is crucial to Rust’s safety
// guarantees.
//

#[allow(dead_code)]
fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

// 1. Given the function’s return type, it’s obvious that v must be a Vec<i16>, a vector of 16-bit
//    signed integers; no other type would work. 
// 2. Type inference gives back much of the legibility of dynamically typed languages, while still
//    catching type errors at compile time.
// 3. Rust’s generic functions give the language a degree of the same flexibility, while still
//    catching all type errors at compile time. (vs. Python and JavaScript, flexibile but difficult 
//    to detect type errors early )
// 4. generic functions are just as efficient as their nongeneric counterparts
#[allow(dead_code)]
fn build_vector2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
#[test]
fn test_build_vector(){
    assert_eq!(Some(&10i16),build_vector().get(0));
    assert_eq!(Some(&10),   build_vector().get(0));
    assert_eq!(Some(&20i16),build_vector().get(1));

    assert_eq!(Some(&10),   build_vector2().get(0));
    assert_eq!(Some(&10i16),build_vector2().get(0));
}


fn main() {
    println!("basic types!");

    // 6. arithmetic operation overflowed
    //    let x = big_val + 1;  
    //    wrap to a negative number
    let big_val = std::i32::MAX;
    let x = big_val.wrapping_add(1);  // ok
    assert_eq!( big_val,   2147483647);
    assert_eq!(       x,  -2147483648);

    // 7. The prefixes 0x, 0o, and 0b designate hexadecimal, octal, and binary literals.
    assert_eq!(   0xcafeu32,  51966);
    assert_eq!(       0o106,     70);
    assert_eq!( 0b0010_1010,     42);

    // 8. big number  
    assert_eq!( 20_922_789_888_000u64, 20922789888000);

    // 9. u8 byte literal
    assert_eq!(b'a', 97);
    assert_eq!(b'A', 65);
    assert_eq!(b'*', 42u8);

    // 10. isize, usize, the same size as an address on the machine
    assert_eq!(137isize,  137);
    assert_eq!(137usize,  137);
    assert_eq!(-0b0101_0010isize, -82);
    assert_eq!( 0xffff_fc00usize, 4_294_966_272);

    // 11. floating-point numbers
    assert_eq!(1.61803f32, 1.61803);
    assert_eq!(6.0221e23f64, 6.0221e23);

    // 12. Unicode character
    assert_eq!('ಠ','\u{CA0}');
    assert_eq!('A','\x41');
    assert_eq!('字', '\u{5B57}');

    // 13. Tuple, mixed types allowed
    assert_eq!(('a',b'A',0x61),('\x61',65,97));

    // 14.  unit (empty) tuple 
    assert_eq!((),());
    assert_eq!((1),(1));

    // 15.  limitations
    assert_eq!(    std::i8::MAX,                  127);
    assert_eq!(    std::i8::MIN,                 -128);
    assert_eq!(   std::i16::MAX,                32767);
    assert_eq!(   std::i16::MIN,               -32768);
    assert_eq!(   std::i32::MAX,           2147483647);
    assert_eq!(   std::i32::MIN,          -2147483648);
    assert_eq!(   std::i64::MAX,  9223372036854775807);
    assert_eq!(   std::i64::MIN, -9223372036854775808);
    assert_eq!( std::isize::MAX,  9223372036854775807);
    assert_eq!( std::isize::MIN, -9223372036854775808);

    assert_eq!(    std::u8::MAX,           (127<<1)+1); //255
    assert_eq!(    std::u8::MIN,                    0);   
    assert_eq!(   std::u16::MAX,         (32767<<1)+1); //65,535
    assert_eq!(   std::u16::MIN,                    0);
    assert_eq!(   std::u32::MAX,    (2147483647<<1)+1); //4,294,967,295
    assert_eq!(   std::u32::MIN,                    0);
    assert_eq!(   std::u64::MAX, 18446744073709551615); 
    assert_eq!(   std::u64::MIN,                    0);
    assert_eq!( std::usize::MAX, 18446744073709551615); 
    assert_eq!( std::usize::MIN,                    0);

    
    // 16.  Characters require a backslash 
    assert_eq!(b'\'', 39   ); // Single quote, '
    assert_eq!(b'\'', 0x27 ); 
    assert_eq!(b'\\', 92   ); // Backslash, \
    assert_eq!(b'\\', 0x5c ); 
    assert_eq!(b'\n', 10   ); // Newline
    assert_eq!(b'\n', 0x0a ); 
    assert_eq!(b'\r', 13   ); // Carriage return
    assert_eq!(b'\r', 0x0d ); 
    assert_eq!(b'\t', 9    ); // Tab
    assert_eq!(b'\t', 0x09 ); 


    // 17.  Type Casts
    assert_eq!(   10_i8  as u16,    10_u16); // in range
    assert_eq!( 2525_u16 as i16,  2525_i16); // in range

    assert_eq!(   -1_i16 as i32,    -1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended
    
    // 17.1 truncation when type casting
    // Conversions that are out of range for the destination
    // produce values that are equivalent to the original modulo 2^N,
    // where N is the width of the destination in bits. This
    // is sometimes called "truncation".
    assert_eq!( 1000_i16 as  u8,   232_u8);
    assert_eq!(65535_u32 as i16,    -1_i16);

    assert_eq!(   -1_i8  as u8,    255_u8);
    assert_eq!(  255_u8  as i8,     -1_i8);

    // 18.  The standard library provides some basic operations 
    //      for the basic types 
    assert_eq!(2u16.pow(4), 16);            // exponentiation
    assert_eq!((-4i32).abs(), 4);           // absolute value
    assert_eq!(0b101101u8.count_ones(), 4); // population count

    assert_eq!(2_usize.pow(32), 4294967296);
    assert_eq!(2_usize.pow(32), 2_usize<<31);
    assert_eq!(std::usize::MAX, (((2_usize<<62)-1)<<1)+1);
    assert_eq!(2_u64.pow(32),   2_u64<<31);

    assert_eq!(std::u64::MAX.count_ones(),64);
    assert_eq!(std::i64::MAX.count_ones(),63);
    assert_eq!(std::i64::MAX, 0x7fff_ffff_ffff_ffff_i64);
    assert_eq!(0xff,0b1111_1111);
    assert_eq!(0x7f,0b0111_1111);

}
