// Remember, a struct is a collection of data types within one custom type
// An enum defines a type that can be various different things, but it has to be one
// Structs are 'ANDs' I'm this and that and that too, Enums are 'ORs' I'm this or that only

fn main() { 
    // Example, an IP address can be IPV4 and IPV6 but not both, so use an enum
    enum IpAddr {
        V4(String),
        V6(String)
    }

    // Using the enum, it's a lot less work and cleaner than making a whole stuct like above
    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

    
    // Enhanced enums
    enum IpAddr2 {
        V4(u8,u8,u8,u8),
        V6(String)
    }

    let home: IpAddr2 = IpAddr2::V4(127,0,0,1);
    let loopback: IpAddr2 = IpAddr2::V6(String::from("::1"));
}
