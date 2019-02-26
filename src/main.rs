extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use std::env;
use std::io::Write;
use std::process;
use std::fmt;


fn join<'a, T>(delimiter: char, iterator: &mut Iterator<Item=T>) -> String
    where T: std::fmt::Display {
    let mut result = String::new();

    if let Some(element) = iterator.next() {
        result.push_str(format!("{}", element).as_ref());

        for element in iterator {
            result.push(delimiter);
            result.push_str(format!("{}", element).as_ref());
        }
    }
    result
}

struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
        where T: ?Sized + AsRef<[u8]> + 'a
    {
        HexSlice(data.as_ref())
    }
}

impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iterator: &mut Iterator<Item=&u8> = &mut self.0.iter();
        write!(f, "{}", join(' ', &mut iterator))?;
        Ok(())
    }
}

fn get_interface_name() -> String {
    let iface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            writeln!(std::io::stderr(), "USAGE: etherdump <NETWORK INTERFACE>").unwrap();
            process::exit(1);
        }
    };
    iface_name
}

fn get_interface_by_name(iface_name: String) -> NetworkInterface {
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();
    interface
}

fn main() {
    use pnet::datalink::Channel::Ethernet;

    let iface_name = get_interface_name();
    let interface = get_interface_by_name(iface_name);

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("etherdump: unhandled channel type: {}"),
        Err(e) => panic!("etherdump: unable to create channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                println!("{}", HexSlice::new(&packet));
            }
            Err(e) => panic!("etherdump: unable to receive packet: {}", e),
        }
    }
}


