use std::net::IpAddr;


fn main(){
    let home: IpAddr = "127.0.0.a"
        .parse()
        .expect("Hardcoded IP address should be valid");
}
