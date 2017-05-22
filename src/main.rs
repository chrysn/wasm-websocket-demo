#[derive(Debug)]
enum Direction { North, South, East, West }

extern crate webplatform;

fn is_north(dir: Direction) -> bool {
    match dir {
        Direction::North => true,
        _ => false,
    }
}

#[no_mangle]
pub fn call_from_js() {
    println!("Called from JS");
//     panic!("Oooh, the pain!");
}

// #[no_mangle]
// pub fn register_callback() {
// }

fn main() {
    println!("Hello World");
    let points = Direction::South;
    println!("{:?}", points);
    let compass = is_north(points);
    println!("{}", compass);


    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    println!("There is a body: {:?}", body);
    let hr = document.element_create("hr").unwrap();
    hr.on("click", move |_| {
        println!("hr was clicked");

        let ls = webplatform::LocalStorageInterface {};
        let old = ls.get("counter");
        let new = match old { None => "0".to_owned(), Some(x) => x + "." };
        println!("counter is {:?}", new);
        ls.set("counter", &new);
    });
    body.append(&hr);
    body.html_append("<h1>Included from Rust</h1>");

    // NEXT STEPS: https://github.com/rust-webplatform/rust-webplatform/issues/23

    webplatform::spin(); /* w/o, the clicks crash */
}
