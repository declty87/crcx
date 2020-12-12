use std::env;
use std::fs;
use std::path::Path;
use std::vec::Vec;

fn make_table32(p: u32) -> Vec<u32> {
    (0u32..256)
        .map(|i| -> u32 {
            let mut c = i;
            for _ in 0..8 {
                c = if c & 1 != 0 { p ^ (c >> 1) } else { c >> 1 };
            }
            c
        })
        .collect()
}

fn main() {
    let crc32_table: Vec<_> = make_table32(0xedb88320);
    let crc32c_table: Vec<_> = make_table32(0x82f63b78);
    let crc32k_table: Vec<_> = make_table32(0xeb31d82e);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("table.rs");

    fs::write(
        &dest_path,
        format!(
            "
pub const CRC32_TABLE: [u32; 256] = {:?};
pub const CRC32C_TABLE: [u32; 256] = {:?};
pub const CRC32K_TABLE: [u32; 256] = {:?};
",
            crc32_table, crc32c_table, crc32k_table
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
