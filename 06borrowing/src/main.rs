//
//  Rust also has nonowning pointer types called references, which have no effect on their
//  referents’ lifetimes.
//  Note:
//  1.) no reference can possibly outlive the value it points to
//  2.) what you have borrowed, you must eventually return to its owner.
//  3.) The references themselves are nothing special—under the hood, they’re just addresses.
//      But the rules that keep them safe are novel to Rust
//  4.) function by-val vs. by-ref. it’s especially important in Rust, because it spells out
//      how ownership is affected.
//      by-val -> moves ownership of the value to the function
//      by-ref -> borrow ownership
//
//  References come in two kinds:
//  1.) A shared reference lets you read but not modify its referent.  &T
//      Shared references are Copy.
//  2.) A mutable reference lets you both read and modify the value.  &mut T
//      Mutable references are not Copy.
//
//
use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

// 1.  Reference Rules
// 1.1 Iterating over a shared reference to a HashMap is defined to produce shared references
//     to each entry’s key and value: artist has changed from a String to a &String, and works
//     from a Vec<String> to a &Vec<String>.
// 1.2 Iterating over a shared reference to a vector is defined to produce shared references to
//     its elements, so work is now a &String.
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}
// 1.3 The mutable borrow required by the vectors’ sort method.
fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn main() {
    println!("Hello, Borrowing!");

    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
        vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
        vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
        vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);

    assert_eq!(table["Gesualdo"][0], "many madrigals");
    assert_eq!(table["Gesualdo"][1], "Tenebrae Responsoria");
    // need a &mut
    sort_works(&mut table);
    assert_eq!(table["Gesualdo"][1], "many madrigals");
    show(&table);

    // 2.  Implicity in Rust ref and de-ref
    //     Since references are so widely used in Rust, the . operator implicitly dereferences
    //     its left operand, if needed:
    //     The . operator can also implicitly borrow a reference to its left operand, if needed
    //     for a method call.

    let x = 10;
    let r = &x;             // &x is a shared reference to x
    assert!(*r == 10);      // explicitly dereference r

    let mut y = 32;
    let m = &mut y;        // &mut y is a mutable reference to y
    *m += 32;              // explicitly dereference m to set y's value
    assert!(*m == 64);     // and to see y's new value
    //assert!(y==64);      // error: can't use y, it was mutably borrowed

    struct Anime { name: &'static str, bechdel_pass: bool };
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.bechdel_pass, true);
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation"); // Equivalent to the above

    let mut v = vec![1973, 1968];
    v.sort();           // implicitly borrows a mutable reference to v
    (&mut v).sort();    // equivalent; much uglier

    // 3.  Assigning References
    //     Assigning to a Rust reference makes it point at a new value
    //
    let x = 10;
    let y = 20;
    let mut r = &x;

    if true { r = &y; }

    // This is very different from C++, where assigning to a reference stores the value in its
    // referent. There’s no way to point a C++ reference to a location other than the one it was
    // initialized with.
    assert!(*r == 10 || *r == 20);

    // 4. References to References
    //        x   y            r        rr       rrr
    // ---------------------------------------------------  stack
    //      |1000|729|       | * |     | * |     |*|
    // -------^---------------|-^-------|-^-------|-------
    //        |               | |       | |       |
    //        +---------------+ +-------+ |       |
    //                                    +-------+
    // the expression rrr.y, guided by the type of rrr, actually traverses three references to get
    // to the Point before fetching its y field.
    //
    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);
    assert_eq!(rrr.x, 1000);

    // 5. Comparing References
    // the == operator follows all the references and performs the comparison on their final
    // targets, x and y.
    // you can use std::ptr::eq, which compares them as addresses to know whether two references
    // point to the same memory
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rx == ry);                // their referents are equal
    assert!(rrx == rry);
    assert!(!std::ptr::eq(rx, ry));   // but occupy different addresses
    assert!(!std::ptr::eq(rrx, rry));

    // 6.  References Are Never Null
    //  there is no default initial value for a reference
    //  won’t convert integers to references,you can’t convert zero into a reference.
    // 6.1 Null Point
    //  In Rust, if you need a value that is either a reference to something or not, use the type
    //  Option<&T>.
    //  Rust represents None as a null pointer, and Some(r), where r is a &T value, as the nonzero
    //  address, so Option<&T> is just as efficient as a nullable pointer in C or C++, even though
    //  it’s safe

    // 7. Borrowing References to Arbitrary Expressions
    // Rust lets you borrow a reference to the value of any sort of expression at all
    // 1.) Rust makes the anonymous variable live as long as the variable the let initializes
    // 2.) the anonymous variable created to hold 1009 lasts only to the end of the assert_eq!
    // statement.
    let r = &factorial(6);
    println!("{}",r); //720
    assert_eq!(r + &1009, 1729);

    // 8. Fa; poin
    // Rust also includes two kinds of fat pointers
    // References to Slices and Trait Objects
    //
    // 9.  Reference Safety
    // 9.1 Borrowing a Local Variable
    // You can’t borrow a reference to a local variable and take it out of the variable’s scope
    // 1.) if you have a variable x, then a reference to x must not outlive x itself
    // 2.) if you store a reference in a variable r, the reference’s type must be good for the
    //   entire lifetime of the variable, from the point it is initialized to the point it goes out
    //   of scope
    //{
    //    let r;
    //    {
    //        let x = 1;
    //        r = &x;
    //    }
    //    assert_eq!(*r, 1);  // bad: reads memory `x` used to occupy
    //}
    // 3.) r reference to x with contradictory constraints on its lifetime

    // 9.2 Receiving References as Parameters
    unsafe { //you may access a mutable static only within an unsafe block.
        assert_eq!(*STASH,10);
    }
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
    unsafe {
        assert_eq!(*STASH,1000);
    }

    // 9.3 Passing References as Arguments
    let a:i32 =  100;
    g(&a);
    g(&WORTH_POINTING_AT);

    // 9.4 Returning References
    let parabola = [9, 4, 1, 0, 1, 4, 9];
    let s = smallest(&parabola);
    assert_eq!(*s, 0); // fine: parabola still alive

    // 9.5 Structs Containing References
    // Whenever a reference type appears inside another type’s definition, you must write out its
    // lifetime. 
    /*
    struct S<'a> {
        r: &'a i32
    }
    struct T<'a> {
        s: S<'a>
    }'
    */
    
    // 9.6 Distinct Lifetime Parameters
    /*
    struct S<'a> {
        x: &'a i32,
        y: &'a i32
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };   //s.x(lifetime) =  s.y(lifetime) = y(lifetime)
            r = s.x;  //s.x(lifetime) = r  -> y(lifetime) = r(lifetime) ! impossiblity
        }
    }
    */
    /*
    struct S<'a,'b> {
        x: &'a i32,
        y: &'b i32
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };   //s.x(lifetime) !=  s.y(lifetime) = y(lifetime)
            r = s.x;  //s.x(lifetime) = r  -> y(lifetime) = r(lifetime) possiblity
        }
    }
    */

    // 9.10 Omitting Lifetime Parameters
    // 1.) function doesn’t return any references (or other types that require lifetime
    //   parameters), then you never need to write out lifetimes for your parameters. 
    //   Rust just assigns a distinct lifetime to each spot that needs one. 
    // 2.) do return references or other types with lifetime parameters
    //  - If there’s only a single lifetime that appears ->  return value must be that one
    //  - If there are multiple lifetimes among your parameters -> you spell out what’s going on
    // 3.) if your function is a method on some type and takes its self parameter by reference
    //   Rust assumes that self’s lifetime is the one to give everything in your return value.
    

    let x = Vec::new();
    let t = StringTable{ elements:x };
    assert_eq!(t.find_by_prefix("t"),None);

    let mut x = Vec::new();
    x.push("test".to_string());
    let t = StringTable{ elements:x };
    assert_eq!(t.find_by_prefix("t"),Some(&"test".to_string()));

    // 9.11 Sharing Versus Mutation
    //
    /*
    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    let aside = v;  // move vector to aside
    r[0];           // bad: uses `v`, which is now uninitialized
    */
    /*
    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    r[0];              
    let aside = v;  // bad: can't move, because it borrow by r 
    */
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0];              
    }
    let _aside = v;  // move ok, r goes out of scope earlier, 
                     // the reference’s lifetime ends before v is moved aside  
    // 9.12 
    //
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);   // extend wave with another vector
    extend(&mut wave, &tail);   // extend wave with an array

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
    assert_eq!(head, vec![0.0, 1.0]);
    assert_eq!(tail, [0.0, -1.0]);
    /*
    extend(&mut wave, &wave);
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0,
                          0.0, 1.0, 0.0, -1.0]);
*/

    //Rust is all about transferring the pain of understanding your program from the future to the present. It works unreasonably well: not only can Rust force you to understand why your program is thread-safe, it can even require some amount of high-level architectural design.

}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0 .. self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}


// 1.) STASH lives for the program’s entire execution, the reference type it holds must have a
// lifetime of the same length; Rust calls this the 'static lifetime.'
static mut STASH: &i32 = &10;

// 2.) the function need to accept a reference that has a 'static lifetime,
//     storing such a reference in STASH can’t create a dangling pointer
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

// 3.) same with fn g(p: &i32)
// it will not save p anywhere that might outlive the call
fn g<'a>(p: &'a i32) {
   println!("{}",*p);
}

// 4.) From smallest’s signature, we can see that its argument and return value must have
// the same lifetime,
// Lifetimes in function signatures let Rust assess the relationships between the references
// you pass to the function and those the function returns, and ensure they’re being used 
// safely.
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}
