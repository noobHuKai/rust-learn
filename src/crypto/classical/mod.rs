//! #古典密码

// 凯撒加密
fn caesar_shifts(message: String, offset: u8) -> String {
    let mut out_buff = String::new();
    for mut c in message.bytes() {
        if c >= b'A' && c <= b'Z' {
            c = (c - b'A' + offset) % 26 + b'A';
        } else if c >= b'a' && c <= b'z' {
            c = (c - b'a' + offset) % 26 + b'a';
        }
        out_buff.push(c as char);
    }
    out_buff
}

fn caesar_shifts_map(message: String, offset: u8) -> String {
    message
        .chars()
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
fn rail_fence_cipher(message: String, step: usize) -> String {
    let mut out_buff = String::new();
    for i in 0..step {
        let mut j = i;
        while j < message.len() {
            out_buff.push_str(&message[j..j + 1]);
            j += step
        }
    }
    out_buff
}

// 维吉尼亚密码
fn vigenere_cipher(message: String, secret: String) -> String {
    let a = secret.to_uppercase();
    let secret = a.as_bytes();
    let mut out_buff = String::new();
    for (index, mut c) in message.bytes().enumerate() {
        let i = index % secret.len();
        if c >= b'A' && c <= b'Z' {
            c = (c + secret[i] - 2 * b'A') % 26 + b'A';
        } else if c >= b'a' && c <= b'z' {
            c = (c + secret[i] - b'a' - b'A') % 26 + b'a';
        }
        out_buff.push(c as char);
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
    println!(
        "Vigenere Cipher result : {:?}",
        vigenere_cipher("HelloWorldHuKai".to_string(), String::from("HuKai"))
    );
}
