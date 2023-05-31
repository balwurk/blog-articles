use std::{net::UdpSocket, usize};
use std::str;
use base64::{Engine as _, engine::general_purpose};

fn deconstruct_packet(buffer: &[u8]) -> Vec<u8> {
    let mut query: Vec<u8> = Vec::<u8>::new();
    let mut base_offset:usize = 0;
    let mut label_length: usize = buffer[0] as usize;
    let mut skip_rest:bool = false;

    buffer.iter().enumerate().for_each(|(pos,value)| {
        if !skip_rest {
            if pos == (base_offset + label_length + 1) {
                if *value == 0u8{
                    skip_rest = true;
                }
                label_length = *value as usize;
                base_offset = pos;
                query.push(0x2eu8);// hexadecimal representation of '.' in unsigned 8 bit
            } else {
                query.push(*value);
            }
        }
    });
    query
}

fn main() {
    let b64engine = general_purpose::STANDARD;
    let socket = UdpSocket::bind("0.0.0.0:53").unwrap();
    let mut buffer: [u8;100] = [0u8;100];
    println!("[+] Initialization complete - waiting for requests");
    loop{
        let (_amt, _src) = socket.recv_from(&mut buffer).unwrap();

        let data = &buffer[12..];

        let query: Vec<u8> = deconstruct_packet(data);

        let query_string: &str = match str::from_utf8(&query[1..]) {
            Ok(query_string) => query_string,
            Err(_error) => panic!("invalid utf8 characters in the payload, do not try to parse directly"),
        };

        let label_section: &[u8] = &query[1..query[0] as usize +1];

        println!("query string: {} \nonly the labeled_section: {}",query_string, 
            match str::from_utf8(label_section){
                Ok(string_label) => string_label,
                Err(_error) => "Invalid utf8 characters in the first label section"
            }
        );

        match b64engine.decode(label_section) {
            Ok(resulting_section) => match str::from_utf8(&resulting_section) {
                Ok(message) => println!("{}",message),
                Err(_error) => println!("Non ascii bytes in the encoded query"),
            },
            Err(_error) => println!("Invalid base64 string"),
        };

        buffer.fill(0u8);
    }
}
