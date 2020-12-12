include!(concat!(env!("OUT_DIR"), "/table.rs"));

use std::io::Read;
use std::io::Result;

macro_rules! def_crc32 {
    ($name:ident, $table:ident) => {
        pub fn $name<R: Read>(r: R) -> Result<u32> {
            let mut c: u32 = 0xffffffff;
            for m in r.bytes() {
                let i = ((c ^ m? as u32) & 0xff) as usize;
                c = $table[i] ^ (c >> 8);
            }
            Ok(c ^ 0xffffffff)
        }
    };
}

def_crc32!(crc32, CRC32_TABLE);
def_crc32!(crc32c, CRC32C_TABLE);
def_crc32!(crc32k, CRC32K_TABLE);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc() {
        let m = b"123456789";
        assert_eq!(crc32(&m[..]).unwrap(), 0xcbf43926);
        assert_eq!(crc32c(&m[..]).unwrap(), 0xe3069283);
        assert_eq!(crc32k(&m[..]).unwrap(), 0x2d3dd0ae);
    }
}
