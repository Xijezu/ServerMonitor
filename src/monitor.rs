#![allow(unused_variables)]
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{thread, time};

use rc4;

#[derive(Deserialize)]
struct Server {
    name: String,
    ip: String,
    port: u16,
}

#[derive(Deserialize)]
struct ServerRegion {
    name: String,
    auth: String,
    servers: Vec<Server>,
}

#[derive(Deserialize)]
struct JSONHelperStruct {
    region: Vec<ServerRegion>,
}

pub struct Monitor {
    servers: Vec<ServerRegion>,
    update_timer: i32,
    save_list: String,
}

impl Default for Monitor {
    fn default() -> Monitor {
        Monitor {
            servers: Vec::new(),
            update_timer: 0,
            save_list: String::new(),
        }
    }
}

impl Monitor {
    pub fn init(&mut self) {
        let conf = ini::Ini::load_from_file("monitor.ini").unwrap();
        let section = conf.section(Some("monitor".to_owned())).unwrap();
        self.update_timer = section
            .get("server.update_timer")
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        self.save_list = section.get("server.save_path").unwrap().to_string();
        self.parse_json();
    }

    pub fn start(&self) {
        // That packet is somehow whack when using the 9.5.2 struct, so... yeah, let's just hardcode it
        let version_pct = vec![
            70u8, 13, 65, 1, 24, 37, 67, 186, 248, 211, 155, 115, 0, 99, 74, 88, 53, 72, 213, 23,
            125, 122, 77, 72, 231, 33, 159, 98, 220, 160, 3, 128, 70, 9, 225, 137, 107, 195, 53,
            176, 170, 34, 240, 237, 240, 72, 89, 91, 232, 12, 45, 8, 24, 43, 13, 34, 14, 146, 119,
            222, 177, 11, 57, 115, 14, 233, 197, 14, 217, 122, 70, 34, 17, 48, 178, 63, 203, 172,
            111, 8, 103, 240, 120, 14, 173, 165, 181, 117, 93, 50, 152, 65, 158, 105, 184, 249, 37,
            36, 106, 30, 47, 56, 7, 27, 69, 240, 238, 47, 127, 117, 28, 180, 14, 58, 15, 21, 48,
            20, 152, 39, 66, 133, 33, 235, 151, 58, 244, 210, 93, 246, 122, 153, 127, 66, 115, 238,
            124, 83, 169, 31, 14, 38, 71, 220, 172, 100, 48, 234, 60, 229, 61, 96, 147, 145, 28,
            218, 201, 96, 70, 228, 107, 223, 94, 142, 56, 22, 126, 158, 137, 150, 105, 141, 176,
            125, 6, 155, 135, 156, 78, 52, 160, 40, 153, 111, 64, 215, 244, 94, 24, 169, 68, 39,
            191, 29, 198, 22, 153, 95, 246, 147, 82, 209, 204, 3, 195, 201, 210, 254, 38, 179, 235,
            49, 204, 107, 90, 9, 88, 165, 33, 45, 3, 221, 131, 162, 90, 20, 157, 158, 203, 215,
            155, 66, 160, 5, 89, 130, 197, 221, 166, 169, 229, 9, 203, 234, 186, 255, 171, 14, 163,
            66, 131, 110, 102, 88, 84, 146, 95, 49, 244, 47, 92, 240, 180, 40, 163, 5, 122, 141,
            36, 131, 55, 220, 195, 0, 152, 247, 36, 107, 6, 85, 149, 213, 116,
        ];

        loop {
            for servers in &self.servers {
                for server in &servers.servers {
                    let mut decipher = rc4::RC4Cipher::default();
                    decipher.init(String::from("}h79q~B%al;k'y $E"));

                    let mut stream = TcpStream::connect((server.ip.as_str(), server.port)).unwrap();
                    let mut input_stream = stream.try_clone().unwrap();
                    let mut client_buffer = [0u8; 1024];

                    let output_stream = &mut stream;
                    output_stream.write(&version_pct).unwrap();
                    output_stream.flush().unwrap();

                    match input_stream.read(&mut client_buffer) {
                        Ok(n) => {
                            if n > 0 {
                                let mut tmp = Vec::new();
                                for i in 0..n {
                                    tmp.push(client_buffer[i]);
                                }
                                decipher.do_cipher(&mut tmp);

                                // Ty stackoverflow
                                let mut i = unsafe {
                                    std::mem::transmute::<[u8; 4], u32>([
                                        tmp[11], tmp[12], tmp[13], tmp[14],
                                    ])
                                }
                                .to_le()
                                    ^ 0xADADADAD;
                                println!("Players online at {}: {}", server.name, i);
                            }
                        }
                        _ => (),
                    }
                }
            }
            thread::sleep(time::Duration::from_secs(5));
        }
    }

    fn parse_json(&mut self) {
        let mut file = File::open("servers.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let json: serde_json::Value =
            serde_json::from_str(&data).expect("JSON was not well-formatted");

        let x: JSONHelperStruct = serde_json::from_value(json).unwrap();
        self.servers = x.region;
    }
}
