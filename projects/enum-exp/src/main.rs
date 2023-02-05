
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        //some method body
    }
}

let m = Message::Write(String::from("hello"));
m.call();

