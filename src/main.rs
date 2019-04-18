mod rc4;

fn main() {
    let mut cipher = rc4::RC4Cipher::default();
    cipher.init(String::from("}h79q~B%al;k'y $E"));

    let mut res = vec![74u8, 65, 73, 74];
    cipher.do_cipher(&mut res);

    println!("The result is {}", String::from_utf8(res).unwrap());
}
