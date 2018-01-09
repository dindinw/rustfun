//
// 1.  `extern crate` directives make crates that we cited in our Cargo.toml available.
// 2.  #[macro_use] attribute alert we plan to use macros exported by the crate.
extern crate iron;
#[macro_use] extern crate mime;

// 3.  iron::prelude::* makes all the public names of the iron::prelude module directly visible.
use iron::prelude::*;
use iron::status;

fn main() {
    println!("Serving on http://localhost:3000...");
    // 4. pass the get_form function to Iron::new, indicating that the server should use that
    //    function to handle all requests
    Iron::new(get_form).http("localhost:3000").unwrap();
}

// 5. get_form function itself takes a mutable reference, written &mut, to a Request value
//    representing the HTTP request we’ve been called to handle.
// 6. _request parameter never be used, giving the parameter a name beginning with _ tells 
//    Rust that we expect the variable to be unused, so it shouldn’t warn about.
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // 7.  The set_mut method uses its argument’s type to decide which part of the response to set
    // 7.1 status::Ok sets the HTTP status
    response.set_mut(status::Ok);
    // 7.2 media type (by mime! macro) sets Content-Type header
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    // 8.  Rust “raw string” syntax
    // 8.1 the letter r, zero or more hash marks (that is, the # character), a double quote 
    // 8.2 then the contents of the string, 
    // 8.3 terminated by another double quote followed by the same number of hash marks
    // 8.4 no escape sequences like \" are recognized
    // 8.5 We can always ensure the string ends where we intend by using more hash marks around the
    //     quotes than ever appear in the text
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
          <input type="text" name="n"/>
          <input type="text" name="n"/>
          <button type="submit">Compute GCD</button>
        </form>
    "#);

    // 9.  IronResult<Response>, is another variant of the Result type
    // 9.1 Ok(r) for some successful Response value r, or Err(e) for some error value e. 
    Ok(response)
}
