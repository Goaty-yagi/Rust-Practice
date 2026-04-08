fu main() {
    enum IpAddrKind {
        v4,
        v6
    }
    let four IpAddrKind =IpAddrKind::v4;
    let six IpAddrKind = IpAddrKind::v6;

    fn route(ip_kind: IpAddrKind) {}

    route(ip_kind: IpAddrKind::v4);
    route(ip_kind: IpAddrKind::v6);

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home: IpAddr = IpAddr{
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1")
    }
    let loopback: IpAddr = IpAddr{
        kind: IpAddrKind::v6,
        address: String::from("::1")
    }

    // different way to define enum
    enum IpAddrKind2 {
        v4(String),
        v6(String)
    }
    // Using Enums
    let home: IpAddrKind2::v4(String::form("127.0.0.1"));
    let loopback: IpAddrKind2::v6(String::form("..1"));

    enum IpAddrKind3 {
        v4(u8,u8,u8,u8),
        v6(String)
    }

    // Enhanced Enums
    let home2: IpAddrKind3::v4(127,0,0,1);
}




