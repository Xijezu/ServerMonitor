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
    packet_version: u8,
    players: u32,
}

#[derive(Deserialize)]
struct ServerRegion {
    // name: String, // Enable when you need it
    // auth: String, // Enable when you need it
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

fn parse_result(mut result: Vec<u8>, server: &mut Server) {
    let mut decipher = rc4::RC4Cipher::default();
    decipher.init(String::from("}h79q~B%al;k'y $E"));
    decipher.do_cipher(&mut result);
    // Ty stackoverflow
    server.players = unsafe {
        std::mem::transmute::<[u8; 4], u32>([result[11], result[12], result[13], result[14]])
    }
    .to_le()
        ^ 0xADADADAD;
    println!("Players online at {}: {}", server.name, server.players);
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

    pub fn start(&mut self) {
        // That packet is somehow whack when using the 9.5.2 struct, so... yeah, let's just hardcode it
        let version_pct_v1 = vec![
            70u8, 12, 65, 1, 25, 37, 65, 186, 248, 211, 155, 115, 0, 99, 74, 88, 53, 72, 213, 23,
            125, 122, 77, 72, 231, 33, 159,
        ];
        let version_pct_v2 = vec![
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
            for mut servers in self.servers.iter_mut() {
                for mut server in servers.servers.iter_mut() {
                    let addr: String = format!("{}:{}", server.ip, server.port);
                    match TcpStream::connect_timeout(
                        &addr.parse().unwrap(),
                        time::Duration::from_secs(1),
                    ) {
                        Ok(mut stream) => {
                            match server.packet_version {
                                1 => {
                                    stream.write(&version_pct_v1).unwrap();
                                    () // Dont ask
                                }
                                2 => {
                                    stream.write(&version_pct_v2).unwrap();
                                    () // Dont ask
                                }
                                _ => (),
                            };

                            let mut client_buffer = [0u8; 1024];
                            stream
                                .set_read_timeout(Some(time::Duration::from_secs(2)))
                                .unwrap();

                            match stream.read(&mut client_buffer) {
                                Ok(n) => {
                                    if n >= 14 {
                                        parse_result(client_buffer[0..n].to_vec(), &mut server);
                                    }
                                }
                                Err(e) => {
                                    println!("Failed to receive data: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to connect to server {}: {}", server.name, e);
                        }
                    }
                }
            }
            thread::sleep(time::Duration::from_secs(60));
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
