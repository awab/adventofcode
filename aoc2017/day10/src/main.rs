use std::fs::File;
use std::io::Read;

/* Part 1
fn main() {
    let lengths = vec![157,222,1,2,177,254,0,228,159,140,249,187,255,51,76,30];
    let mut data: Vec<_> = (0..=255).collect();
    let mut index = 0;
    let mut skip_size = 0;

    for len in lengths {
        let start = index;
        let end = (index + len - 1) % data.len();  // -1 since we're 0 based
        reverse(&mut data, start, end, len / 2);

        index += len;
        index += skip_size;
        if index > data.len() - 1 {
            index = index % data.len();
        }

        skip_size += 1;
    }

    println!("Checksum: {}", (data[0] as u32) * (data[1] as u32));
}
*/

fn main() {
    let filename = "input.txt";
    let mut lengths = vec![];
    let _ = File::open(&filename).and_then(|mut f| f.read_to_end(&mut lengths));
    let additional_bytes = vec![17u8, 31u8, 73u8, 47u8, 23u8];
    for byte in additional_bytes {
        lengths.push(byte);
    }

    let mut data: Vec<_> = (0..=255).collect();
    let mut index: usize = 0;
    let mut skip_size = 0;

    for _ in 1..=64 {
        for len in &lengths {
            let len = *len as usize;
            let start = index;
            let end = (index + len - 1) % data.len();  // -1 since we're 0 based
            reverse(&mut data, start, end, len / 2);
    
            index += len;
            index += skip_size;
            if index > data.len() - 1 {
                index = index % data.len();
            }
    
            skip_size += 1;
        }
    }

    let mut hash = "".to_string();
    let mut hashindex = 0;
    let mut hashsubindex = 0;
    for hashindex in 0..16 {
        let mut accumulator = 0;
        for hashsubindex in 0..16 {
            if hashsubindex == 0 {
                accumulator = data[(hashindex * 16) + hashsubindex];
            } else {
                accumulator ^= data[(hashindex * 16) + hashsubindex];
            }
        }

        hash.push_str(&format!("{:02x}", accumulator));
    }

    println!("Hash: {}", hash);
}

fn reverse(data: &mut Vec<u8>, mut start: usize, mut end: usize, length: usize) {
    let mut index = 0;
    
    loop {
        if index == length {
            break;
        }

        let temp = data[start];
        data[start] = data[end];
        data[end] = temp;

        start += 1;
        if start == data.len() {
            start = 0;
        }

        if end == 0 {
            end = data.len() - 1;
        } else {
            end -= 1;
        }

        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_works() {
        let mut data: Vec<u8> = (0..5).collect();
        reverse(&mut data, 0, 4, 2);
        assert_eq!(data, vec![4, 3, 2, 1, 0]);

        reverse(&mut data, 2, 1, 2);
        assert_eq!(data, vec![1, 2, 3, 4, 0]);

        reverse(&mut data, 1, 0, 2);
        assert_eq!(data, vec![2, 1, 0, 4, 3]);

        reverse(&mut data, 1, 0, 0);
        assert_eq!(data, vec![2, 1, 0, 4, 3]);

        reverse(&mut data, 1, 2, 1);
        assert_eq!(data, vec![2, 0, 1, 4, 3]);
    }
}