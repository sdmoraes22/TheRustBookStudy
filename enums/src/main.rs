enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

struct QuitMessage;

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);

struct ChangeColorMessage(i32, i32, i32);

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEn {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0:1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("::1"),
    };

    let home_en = IpAddrEn::V4(String::from("129.0.0.1"));

    let loopback_en = IpAddrEn::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

    m.call();

    let some_number = Some(5);
    let some_char = Some("a");

    let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    //
    // let some = x + y;
}
