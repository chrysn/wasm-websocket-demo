extern crate webplatform;

fn main() {
    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    let h1 = document.element_create("h1").unwrap();
    h1.html_set("Hello from Rust, click me and watch the console");

    h1.on("click", move |_| {
        println!("h1 was clicked");

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

    webplatform::spin(); /* w/o, the clicks crash */
}
