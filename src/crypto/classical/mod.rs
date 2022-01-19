//! #古典密码

// 凯撒加密
fn caesar_shifts(buff: String, offset: u8) -> String {
    let mut out_buff = String::new();
    for c in buff.bytes() {
        let a;
        if c >= b'A' && c <= b'Z' {
            a = (c - b'A' + offset) % 26 + b'A';
        } else if c >= b'a' && c <= b'z' {
            a = (c - b'a' + offset) % 26 + b'a';
        } else {
            a = c;
        }
        out_buff.push(a as char);
    }
    out_buff
}

fn caesar_shifts_map(buff: String, offset: u8) -> String {
    buff.chars()
        .map(|c| {
            if c >= 'A' && c <= 'Z' {
                ((c as u8 - b'A' + offset) % 26 + b'A') as char
            } else if c >= 'a' && c <= 'z' {
                ((c as u8 - b'a' + offset) % 26 + b'a') as char
            } else {
                c
            }
        })
        .collect::<String>()
}
// 栅栏加密
fn rail_fence_cipher(buff: String, step: usize) -> String {
    let mut out_buff = String::new();
    for i in 0..step {
        let mut j = i;
        while j < buff.len() {
            out_buff.push_str(&buff[j..j + 1]);
            j += step
        }
    }
    out_buff
}

#[test]
fn test_classical_encryption() {
    println!(
        "Caesar Shifts result : {:?}",
        caesar_shifts("AZaz".to_string(), 1)
    );
    println!(
        "Caesar Shifts result : {:?}",
        caesar_shifts_map("AZaz".to_string(), 1)
    );
    println!(
        "Rail Fence Cipher result : {:?}",
        rail_fence_cipher("AZaz66".to_string(), 2)
    );
}
