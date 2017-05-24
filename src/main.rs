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
    let h1 = document.element_create("h1").unwrap();
    h1.html_set("Hello from Rust, click me!");
    h1.on("click", move |_| {
        println!("h1 was clicked");

        let ls = webplatform::LocalStorageInterface {};
        let old = ls.get("counter");
        let new = match old { None => "0".to_owned(), Some(x) => x + "." };
        println!("counter is {:?}", new);
        ls.set("counter", &new);


        let s = document.websocket_create("wss://echo.websocket.org/").unwrap();
        let s = std::rc::Rc::new(s);
        s.addEventListener_message_string(|message| { println!("Message string: {:?}", message); });
        s.addEventListener_message_binary(|message| { println!("Message binary: {:?}", message); });
        let s_for_callback = s.clone(); // with lexical scoping, we could (almost ;-) avoid all that, but this is a simple example anyway
        s.addEventListener_open(move || {
            println!("Connected");
            s_for_callback.send("Hello World");
            let some_data = [10, 20, 30, 40, 200, 0, 99];
            s_for_callback.send_binary(&some_data);
        });
    });
    body.append(&h1);

    // NEXT STEPS: https://github.com/rust-webplatform/rust-webplatform/issues/23

    webplatform::spin(); /* w/o, the clicks crash */
}
