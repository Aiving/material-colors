use super::color::blue_from_argb;
use super::color::green_from_argb;
use super::color::red_from_argb;
use super::color::Argb;

const HASH: u8 = b'#';

fn to_hex(n: u8) -> String {
    let s = format!("{:x}", n);

    if s.len() == 1 {
        String::from("0") + &s
    } else {
        s
    }
}

pub(crate) fn hex_from_argb(argb: Argb) -> String {
    let red = red_from_argb(argb);
    let blue = blue_from_argb(argb);
    let green = green_from_argb(argb);

    format!("{}{}{}", to_hex(red), to_hex(green), to_hex(blue))
}

pub fn argb_from_hex<T: Into<String>>(hex: T) -> Argb {
    let hex: String = hex.into();

    from_hex(hex.as_bytes()).unwrap()
}

pub(crate) fn from_hex(s: &[u8]) -> Result<[u8; 4], ()> {
    let mut buff: [u8; 6] = [0; 6];
    let mut buff_len = 0;

    for b in s {
        if !b.is_ascii() || buff_len == 6 {
            return Err(());
        }

        let bl = b.to_ascii_lowercase();

        if bl == HASH {
            continue;
        }

        if bl.is_ascii_hexdigit() {
            buff[buff_len] = bl;
            buff_len += 1;
        } else {
            return Err(());
        }
    }

    if buff_len == 3 {
        buff = [buff[0], buff[0], buff[1], buff[1], buff[2], buff[2]];
    }

    let hex_str = core::str::from_utf8(&buff).map_err(|_| ())?;
    let hex_digit = u32::from_str_radix(hex_str, 16).map_err(|_| ())?;

    Ok(hex_digit_to_rgb(hex_digit))
}

fn hex_digit_to_rgb(num: u32) -> [u8; 4] {
    let r = num >> 16;
    let g = (num >> 8) & 0x00FF;
    let b = num & 0x0000_00FF;
  
    [255, r as u8, g as u8, b as u8]
}
