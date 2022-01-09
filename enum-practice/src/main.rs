fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        addr: String,
        kind: IpAddrKind,
    }

    impl IpAddr {
        fn new(addr: String, kind: IpAddrKind) -> Self {
            Self { addr, kind }
        }
    }

    let address = String::from("127.0.0.1");
    let home = IpAddr::new(address, IpAddrKind::V4);

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
