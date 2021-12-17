// fn main() { // Option 1
//     enum IpAddrKind {
//     V4,
//     V6,
//     }
//
//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }
//
//     let home = IpAddr {
//         kind:IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// fn main() { // This is more efficient
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }
//
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//
//     let loopback = IpAddr::V6(String::from("::1"));
// }

fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}