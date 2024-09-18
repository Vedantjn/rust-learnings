#[derive(Debug)]
enum IpAddrKind {
    V4, 
    V6
}

fn main() {
    let four = IpAddrKind::V4;
    route("1.2.3.4", four);
}

// 192. 168.1.1 etc.
fn route(ip: &str, kind: IpAddrKind) {
    println!("Routing the request to IP {ip} of kind {kind:?}");
}

// -------------------------------------------------------------------------

#[derive(Debug)]
enum IpAddrKind {
    V4, 
    V6
}

struct IpAddress {
    address: String,
    kind: IpAddrKind
}

fn main() {
    let google_address = IpAddress {
        address: String::from("1.2.3.4"),
        kind: IpAddrKind::V4
    }
    route(google_address);
}

// 192. 168.1.1 etc.
fn route(ip: IpAddress) {
    println!("Routing the request to IP {} of kind {:?}", ip.address, ip.kind);
}

// -------------------------------------------------------------------------

#[derive(Debug)]
enum IpAddrKind {
    V4, 
    V6
}

struct IpAddress {
    address: String,
    kind: IpAddrKind
}

impl IpAddress {
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            kind: IpAddrKind::V4,
        }
    }
}

fn main() {
    let google_address = IpAddress::new("1.2.3.4");
    route(google_address);
}

// 192. 168.1.1 etc.
fn route(ip: IpAddress) {
    println!("Routing the request to IP {} of kind {:?}", ip.address, ip.kind);
}

// -------------------------------------------------------------------------

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    route(home);
}

// 192. 168.1.1 etc.
fn route(ip: IpAddrKind) {
    println!("Routing request to {:?}", ip);
}

// -------------------------------------------------------------------------

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    route(home);
    route(loopback);
}

// 192. 168.1.1 etc.
fn route(ip: IpAddrKind) {
    println!("Routing request to {:?}", ip);
}

// -------------------------------------------------------------------------

#[derive(Debug)]
srtuct IpV4Addr(u8, u8, u8, u8);
#[derive(Debug)]
srtuct IpV6Addr(String);

enum IpAddrKind {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

fn main() {
    let home = IpAddrKind::V4(IpV4Addr(127, 0, 0, 1));
    let loopback = IpAddrKind::V6(IpV6Addr(String::from("::1")));
    route(home);
    route(loopback);
}

// 192. 168.1.1 etc.
fn route(ip: IpAddrKind) {
    println!("Routing request to {:?}", ip);
}

// -------------------------------------------------------------------------

enum Message {
    Quite,
    Move {x: i32, y: i32},
    Write(u8),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let op = Option::Some(1);
    // let op = Some(1);
    let m = Message::Write(1);
}

// -------------------------------------------------------------------------

enum File {}

struct Message {
    data: String,
    File: Option<>,
}

fn main() {
    
}

// -------------------------------------------------------------------------

fn main() {
    let op = Some(1).is_some(); // boolean return krega
}

// -------------------------------------------------------------------------

// it won't work without unwrap
fn main() {
    let op: Option<i32> = Some(1); // boolean return krega
    let x = 2;

    let sum = x + op.unwrap(0); // it'll panic if default value is not provided inside unwrap

    println!("Sum = {sum}");
}