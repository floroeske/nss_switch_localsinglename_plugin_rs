extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libnss;

use std::net::IpAddr;
use dns_lookup::lookup_host;
use libnss::host::{AddressFamily, Addresses, Host, HostHooks};
use libnss::interop::Response;

struct LocalSingleNameHost;
libnss_host_hooks!(localsinglename, LocalSingleNameHost);

impl HostHooks for LocalSingleNameHost {

    fn get_all_entries() -> Response<Vec<Host>> {

        // eprintln!("NSSRUST: get_all_entries");

        // Return an empty vector
        let empty_hosts: Vec<Host> = Vec::new();
        Response::Success(empty_hosts)

        /*
        Response::Success(vec![Host {
            name: "test.example".to_string(),
            addresses: Addresses::V4(vec![Ipv4Addr::new(177, 42, 42, 42)]),
            aliases: vec!["other.example".to_string()],
        }])
        */
    }

    fn get_host_by_addr(_addr: IpAddr) -> Response<Host> {

        // eprintln!("NSSRUST: get_host_by_addr");
        Response::NotFound

        /*
        match addr {
            IpAddr::V4(addr) => {
                if addr.octets() == [177, 42, 42, 42] {
                    Response::Success(Host {
                        name: "test.example".to_string(),
                        addresses: Addresses::V4(vec![Ipv4Addr::new(177, 42, 42, 42)]),
                        aliases: vec!["other.example".to_string()],
                    })
                } else {
                    Response::NotFound
                }
            }
            _ => Response::NotFound,
        }
        */
    }

    fn get_host_by_name(name: &str, family: AddressFamily) -> Response<Host> {

        // eprintln!("NSSRUST: get_host_by_name");

        if name.contains(".local") {
            // eprintln!("NSSRUST: The string contains .local.");
            return Response::NotFound
        } else {
            // eprintln!("NSSRUST: The string does not contain .local");
        }

        if name.contains('.') {
            // eprintln!("NSSRUST: The string contains a dot.");
            return Response::NotFound
        } else {
            // eprintln!("NSSRUST: The string does not contain a dot.");
        }

        let mut local_name = String::from(name);
        local_name.push_str(".local");

        // eprintln!("NSSRUST: adding '.local'. New name: '{}'", local_name);

        match lookup_host(&local_name) {
            Ok(ips) => {
                // println!("NSSRUST: Host '{}' resolved to IP addresses:", local_name);
                for address in ips {
                    if address.is_ipv4() {
                        // eprintln!("NSSRUST: is ipv4 {}", address);
                    }

                    match address {
                        IpAddr::V4(ipv4) => {
                            // ip is an Ipv4Addr
                            // println!("NSSRUST: IPv4 address: {}", ipv4);

                            if address.is_ipv4() && family == AddressFamily::IPv4 {
                                return Response::Success(Host {
                                    name: local_name.to_string(),
                                    addresses: Addresses::V4(vec![ipv4]),
                                    aliases: vec!["test.example".to_string(), "other.example".to_string()],
                                });
                            }

                        }
                        IpAddr::V6(_) => {
                            // ip is an Ipv6Addr
                            // println!("NSSRUST: IPv6 address");
                        }
                    }

                }
            }
            Err(_rr) => {
                // println!("NSSRUST: Error resolving host '{}': {}", local_name, err);
            }
        }

        Response::NotFound
    }
}
