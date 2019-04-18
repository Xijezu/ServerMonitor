pub struct RC4Cipher {
    x: i32,
    y: i32,
    state: [u8; 256],
}

impl Default for RC4Cipher {
    fn default() -> RC4Cipher {
        RC4Cipher {
            x: 0,
            y: 0,
            state: [0; 256],
        }
    }
}

impl RC4Cipher {
    pub fn init(&mut self, key: String) -> bool {
        let key_array = key.as_bytes();

        let key_len = key.len();
        for i in 0..self.state.len() {
            self.state[i] = i as u8;
        }

        let mut temp_key = [0u8; 256];
        let mut j = 0;
        for i in 0..256 {
            temp_key[i] = key_array[j];

            if j + 1 >= key_len {
                j = 0
            } else {
                j += 1;
            }
        }

        j = 0;
        for i in 0..256 {
            j = (j + self.state[j] as usize + temp_key[j] as usize) & 0xFF;
            self.state.swap(i, j);
        }

        self.x = 0;
        self.y = 0;
        self.skip_for(1013);

        return true;
    }

    fn skip_for(&mut self, mut par_len: i32) {
        let mut curr_x = self.x;
        let mut curr_y = self.y;

        while par_len != 0 {
            curr_x = (curr_x + 1) & 0xFF;
            let sx = self.state[curr_x as usize];
            curr_y = (curr_y + sx as i32) & 0xFF;
            self.state[curr_x as usize] = self.state[curr_y as usize];
            self.state[curr_y as usize] = sx;
            par_len -= 1;
        }

        self.x = curr_x;
        self.y = curr_y;
    }

    // The same for encoding and decoding actually
    pub fn do_cipher(&mut self, vector: &mut Vec<u8>) {
        let mut x = self.x;
        let mut y = self.y;

        for it in vector.iter_mut() {
            x = (x + 1) & 0xFF;
            let sx = self.state[x as usize];
            y = (y + sx as i32) & 0xFF;
            let sy = self.state[y as usize];
            self.state[x as usize] = sy;
            self.state[y as usize] = sx;

            *it = (*it) ^ self.state[((sx as i32 + sy as i32) & 0xFF) as usize];
        }

        self.x = x;
        self.y = y;
    }
}
