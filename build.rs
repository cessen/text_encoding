use std::env;
use std::fs::File;
use std::io::{BufRead, Read, Write};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Generate BIG5 tables.
    generate_big5_tables(
        File::open("encoding_tables/big5_whatwg.txt").unwrap(),
        File::create(&Path::new(&out_dir).join("big5_whatwg_tables.rs")).unwrap(),
    ).unwrap();

    // Generate all of the single byte encoding tables and wrapper code.
    {
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-ibm866.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("ibm866.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-2.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-2.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-3.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-3.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-4.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-4.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-5.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-5.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-6.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-6.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-7.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-7.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-8.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-8.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-9.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-9.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-10.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-10.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-tis-620.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("tis-620.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-13.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-13.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-14.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-14.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-15.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-15.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-iso-8859-16.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("iso-8859-16.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-koi8-r.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("koi8-r.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-koi8-u.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("koi8-u.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-macintosh.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("macintosh.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-874.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-874.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-1250.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-1250.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-1251.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-1251.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-1252.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-1252.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-1253.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-1253.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-1254.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-1254.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-1255.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-1255.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-1256.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-1256.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-1257.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-1257.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-windows-1258.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("windows-1258.rs")).unwrap(),
        ).unwrap();
        generate_single_byte_encoding_from_index(
            File::open("encoding_tables/index-x-mac-cyrillic.txt").unwrap(),
            File::create(&Path::new(&out_dir).join("x-mac-cyrillic.rs")).unwrap(),
        ).unwrap();
    }
}

/// Generates tables for BIG5 encoding conversion.  Input should be a file
/// with hex-encoded (e.g. 0xA040) BIG5 in a column on the left and
/// corresponding hex-encoded Unicode scalar values on the right.  Lines
/// starting with # are ignored.
fn generate_big5_tables<R: Read, W: Write>(in_file: R, mut out_file: W) -> std::io::Result<()> {
    // Load table into memory.
    let table = {
        let in_file = std::io::BufReader::new(in_file);
        let mut table = Vec::new();
        for line in in_file.lines() {
            let tmp = line.unwrap();
            let line = tmp.trim();
            if line.starts_with("#") || line == "" {
                continue;
            }

            let elements: Vec<_> = line.split_whitespace().collect();
            if elements.len() >= 2 {
                let big5 = u32::from_str_radix(&elements[0][2..], 16).unwrap();
                let unicode = std::char::from_u32(
                    u32::from_str_radix(&elements[1][2..], 16).unwrap(),
                ).unwrap();
                table.push((big5, unicode));
            }
        }
        table
    };

    // Build the decode table.
    let dec_table = {
        let mut table = table.clone();
        table.sort_by_key(|v| v.0);
        table.dedup_by_key(|v| v.0);
        let mut i = 0;
        let mut dec_table: Vec<char> = Vec::new();
        for (big5_code, unicode) in &table {
            let index = {
                let byte_1 = big5_code >> 8;
                let byte_2 = big5_code & 0xFF;
                let lead = (byte_1 - 0x81u32) * 157u32;
                let offset = if byte_2 < 0x7fu32 { 0x40u32 } else { 0x62u32 };
                lead + byte_2 - offset
            };
            while i < index as usize {
                dec_table.push('�');
                i += 1;
            }
            dec_table.push(*unicode);
            i += 1;
        }
        dec_table
    };

    // Build the encode table.
    let enc_table = {
        let mut table = table.clone();
        table.sort_by_key(|v| v.1);
        table.dedup_by_key(|v| v.1);
        let mut enc_table: Vec<(char, [u8; 2])> = Vec::new();
        for (big5_code, unicode) in &table {
            let big5_bytes = [(big5_code >> 8) as u8, (big5_code & 0xFF) as u8];
            enc_table.push((*unicode, big5_bytes));
        }
        enc_table
    };

    // Write encode table.
    out_file.write_all(
        format!(
            r#"
const ENCODE_TABLE: [(char, [u8; 2]); {}] = [
"#,
            enc_table.len()
        ).as_bytes(),
    )?;

    for (ii, (unicode, big5_bytes)) in enc_table.iter().enumerate() {
        if ii % 4 == 0 && ii != 0 {
            out_file.write_all("\n".as_bytes())?;
        }
        out_file.write_all(
            format!(
                "('\\u{{{:X}}}', [0x{:X}, 0x{:X}]), ",
                *unicode as u32, big5_bytes[0], big5_bytes[1],
            ).as_bytes(),
        )?;
    }

    out_file.write_all(
        format!(
            r#"
];
"#
        ).as_bytes(),
    )?;

    // Write decode table.
    out_file.write_all(
        format!(
            r#"
const DECODE_TABLE: [char; {}] = [
"#,
            dec_table.len()
        ).as_bytes(),
    )?;

    for (i, c) in dec_table.iter().enumerate() {
        if i % 8 == 0 && i != 0 {
            out_file.write_all("\n".as_bytes())?;
        }
        out_file.write_all(format!("'\\u{{{:X}}}', ", *c as u32).as_bytes())?;
    }

    out_file.write_all(
        format!(
            r#"
];
"#
        ).as_bytes(),
    )?;

    Ok(())
}

/// Generates tables for single-byte encoding conversion from text indexes in
/// the WHATWG encoding-standard single-byte index format.
fn generate_single_byte_encoding_from_index<R: Read, W: Write>(
    in_file: R,
    mut out_file: W,
) -> std::io::Result<()> {
    let in_file = std::io::BufReader::new(in_file);

    // Collect the table.
    let table = {
        let mut table = ['�'; 128];
        for line in in_file.lines() {
            let tmp = line.unwrap();
            let line = tmp.trim();
            if line.starts_with("#") || line == "" {
                continue;
            }

            let elements: Vec<_> = line.split_whitespace().collect();
            if elements.len() >= 2 {
                let index = elements[0].parse::<usize>().unwrap();
                assert!(index <= 127);
                let code = std::char::from_u32(u32::from_str_radix(&elements[1][2..], 16).unwrap())
                    .unwrap();
                table[index] = code;
            }
        }
        table
    };

    // Build the reverse table.
    let rev_table = {
        let mut rev_table = vec![];
        for (i, c) in table.iter().enumerate() {
            if *c != '�' {
                rev_table.push((*c, 128 + i));
            }
        }
        rev_table.sort_by_key(|x| x.0);
        rev_table
    };

    // Write shared code.
    out_file.write_all(
        format!(
            r#"
use {{DecodeResult, EncodeResult}};

pub fn encode_from_str<'a>(input: &str, output: &'a mut [u8]) -> EncodeResult<'a> {{
    super::single_byte_encode_from_str(&ENCODE_TABLE, input, output)
}}

pub fn decode_to_str<'a>(input: &[u8], output: &'a mut [u8]) -> DecodeResult<'a> {{
    super::single_byte_decode_to_str(&DECODE_TABLE, input, output)
}}
"#
        ).as_bytes(),
    )?;

    // Write encode table.
    out_file.write_all(
        format!(
            r#"
const ENCODE_TABLE: [(char, u8); {}] = [
"#,
            rev_table.len()
        ).as_bytes(),
    )?;

    for (c, i) in rev_table.iter() {
        out_file.write_all(format!("('\\u{{{:04X}}}', 0x{:02X}), ", *c as u32, i).as_bytes())?;
    }

    out_file.write_all(
        format!(
            r#"
];
"#
        ).as_bytes(),
    )?;

    // Write decode table.
    out_file.write_all(
        format!(
            r#"
const DECODE_TABLE: [char; 128] = [
"#
        ).as_bytes(),
    )?;

    for c in table.iter() {
        out_file.write_all(format!("'\\u{{{:04X}}}', ", *c as u32).as_bytes())?;
    }

    out_file.write_all(
        format!(
            r#"
];
"#
        ).as_bytes(),
    )?;

    Ok(())
}

//===========================================================================
// UTILITES
//
// These are not used during the normal build process, but rather can be
// temporarily added to generate data that we want to use elsewhere.
//===========================================================================

/// Converts the WHATWG BIG5 table into a more typical BIG5 table format,
/// so we can use the same parsing code for all of the BIG5 tables.
#[allow(dead_code)]
fn whatwg_table_to_big5_table<R: Read, W: Write>(
    in_file: R,
    mut out_file: W,
) -> std::io::Result<()> {
    let in_file = std::io::BufReader::new(in_file);

    // Collect the table.
    let table = {
        let mut table = Vec::new();
        for line in in_file.lines() {
            let tmp = line.unwrap();
            let line = tmp.trim();
            if line.starts_with("#") || line == "" {
                continue;
            }

            let elements: Vec<_> = line.split_whitespace().collect();
            if elements.len() >= 2 {
                let index = elements[0].parse::<usize>().unwrap();
                let code = std::char::from_u32(u32::from_str_radix(&elements[1][2..], 16).unwrap())
                    .unwrap();
                table.push((index, code));
            }
        }
        table.sort_by_key(|v| v.0);
        table
    };

    // Convert and write the table.
    for (i, c) in table.iter() {
        let (lead, trail) = {
            let lead = i / 157 + 0x81;
            let trail = i % 157;
            let offset = if trail < 0x3F { 0x40 } else { 0x62 };
            (lead as u8, (trail + offset) as u8)
        };
        out_file.write_all(format!("0x{:02X}{:02X} 0x{:X}\n", lead, trail, *c as u32).as_bytes())?;
    }

    Ok(())
}

/// Generates basic test files for the BIG5 encoders/deconders from their
/// table files.
#[allow(dead_code)]
fn generate_big5_test_files<R: Read, W: Write>(
    in_file: R,
    mut decode_in: W,
    mut decode_out: W,
    mut encode_in: W,
    mut encode_out: W,
) -> std::io::Result<()> {
    let in_file = std::io::BufReader::new(in_file);

    // Collect the table.
    let mut table = {
        let mut table = Vec::new();
        for line in in_file.lines() {
            let tmp = line.unwrap();
            let line = tmp.trim();
            if line.starts_with("#") || line == "" {
                continue;
            }

            let elements: Vec<_> = line.split_whitespace().collect();
            if elements.len() >= 2 {
                let big5 = u32::from_str_radix(&elements[0][2..], 16).unwrap();
                let unicode = std::char::from_u32(
                    u32::from_str_radix(&elements[1][2..], 16).unwrap(),
                ).unwrap();
                table.push((big5, unicode));
            }
        }
        table
    };

    // Write the big5 decode file.  This is the input for decoding test.
    for ascii_byte in 1..127u8 {
        decode_in.write_all(&[ascii_byte])?;
        decode_in.write_all("\n".as_bytes())?;
    }
    for (big5_code, _) in table.iter() {
        let code_bytes = [(*big5_code >> 8) as u8, (*big5_code & 0xFF) as u8];
        decode_in.write_all(&code_bytes)?;
        decode_in.write_all("\n".as_bytes())?;
    }

    // Write the utf8 decode file.  This is the output for decoding test.
    for ascii_byte in 1..127u8 {
        decode_out.write_all(&[ascii_byte])?;
        decode_out.write_all("\n".as_bytes())?;
    }
    for (_, unicode) in table.iter() {
        decode_out.write_all(format!("{}\n", unicode.encode_utf8(&mut [0u8; 4])).as_bytes())?;
    }

    // Dedup the table for encoding test data.
    table.sort_by_key(|v| v.1);
    table.dedup_by_key(|v| v.1);
    table.sort_by_key(|v| v.0);

    // Write the big5 decode file.  This is the input for decoding test.
    for ascii_byte in 1..127u8 {
        encode_out.write_all(&[ascii_byte])?;
        encode_out.write_all("\n".as_bytes())?;
    }
    for (big5_code, _) in table.iter() {
        let code_bytes = [(*big5_code >> 8) as u8, (*big5_code & 0xFF) as u8];
        encode_out.write_all(&code_bytes)?;
        encode_out.write_all("\n".as_bytes())?;
    }

    // Write the utf8 decode file.  This is the output for decoding test.
    for ascii_byte in 1..127u8 {
        encode_in.write_all(&[ascii_byte])?;
        encode_in.write_all("\n".as_bytes())?;
    }
    for (_, unicode) in table.iter() {
        encode_in.write_all(format!("{}\n", unicode.encode_utf8(&mut [0u8; 4])).as_bytes())?;
    }

    Ok(())
}
