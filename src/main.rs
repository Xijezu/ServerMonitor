#![allow(unused_variables)]

mod rc4;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process;
use std::thread;

fn main() {
    // That packet is somehow whack when using the 9.5.2 struct, so... yeah, let's just hardcode it
    let version_pct = vec![
        70u8, 13, 65, 1, 24, 37, 67, 186, 248, 211, 155, 115, 0, 99, 74, 88, 53, 72, 213, 23, 125,
        122, 77, 72, 231, 33, 159, 98, 220, 160, 3, 128, 70, 9, 225, 137, 107, 195, 53, 176, 170,
        34, 240, 237, 240, 72, 89, 91, 232, 12, 45, 8, 24, 43, 13, 34, 14, 146, 119, 222, 177, 11,
        57, 115, 14, 233, 197, 14, 217, 122, 70, 34, 17, 48, 178, 63, 203, 172, 111, 8, 103, 240,
        120, 14, 173, 165, 181, 117, 93, 50, 152, 65, 158, 105, 184, 249, 37, 36, 106, 30, 47, 56,
        7, 27, 69, 240, 238, 47, 127, 117, 28, 180, 14, 58, 15, 21, 48, 20, 152, 39, 66, 133, 33,
        235, 151, 58, 244, 210, 93, 246, 122, 153, 127, 66, 115, 238, 124, 83, 169, 31, 14, 38, 71,
        220, 172, 100, 48, 234, 60, 229, 61, 96, 147, 145, 28, 218, 201, 96, 70, 228, 107, 223, 94,
        142, 56, 22, 126, 158, 137, 150, 105, 141, 176, 125, 6, 155, 135, 156, 78, 52, 160, 40,
        153, 111, 64, 215, 244, 94, 24, 169, 68, 39, 191, 29, 198, 22, 153, 95, 246, 147, 82, 209,
        204, 3, 195, 201, 210, 254, 38, 179, 235, 49, 204, 107, 90, 9, 88, 165, 33, 45, 3, 221,
        131, 162, 90, 20, 157, 158, 203, 215, 155, 66, 160, 5, 89, 130, 197, 221, 166, 169, 229, 9,
        203, 234, 186, 255, 171, 14, 163, 66, 131, 110, 102, 88, 84, 146, 95, 49, 244, 47, 92, 240,
        180, 40, 163, 5, 122, 141, 36, 131, 55, 220, 195, 0, 152, 247, 36, 107, 6, 85, 149, 213,
        116,
    ];

    // RC4 initialization
    let mut decipher = rc4::RC4Cipher::default();
    decipher.init(String::from("}h79q~B%al;k'y $E"));

    // Connect or die
    let mut stream =
        TcpStream::connect(("174.35.123.8", 4514)).unwrap_or_else(|error| process::exit(-2));
    let mut input_stream = stream.try_clone().unwrap();

    thread::spawn(move || {
        let mut client_buffer = [0u8; 1024];

        loop {
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
                        println!("Players online: {}", i);
                        process::exit(-1)
                    }
                }
                _ => process::exit(-1),
            }
        }
    });

    let output_stream = &mut stream;
    output_stream.write(&version_pct).unwrap();
    output_stream.flush().unwrap();

    // Dirty hack so the program doesn't close..
    let mut user_buffer = String::new();
    io::stdin().read_line(&mut user_buffer).unwrap();
}
