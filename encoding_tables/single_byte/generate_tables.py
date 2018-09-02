#!/usr/bin/env python3

def generate_ascii_ext_encoding(in_path, out_path):
    in_file = open(in_path)
    out_file = open(out_path, mode='w')

    # Load the table from the file.
    table = []
    for line in in_file:
        parts = line.split()
        if len(parts) >= 2 and not parts[0].startswith("#"):
            byte = int(parts[0])
            unicode = int(parts[1], 16)
            table += [(byte, unicode)]
    table.sort()

    # Create the decode table
    dec_table = []
    i = 0
    for item in table:
        while i < item[0]:
            dec_table += [None]
            i += 1
        dec_table += [item[1]]
        i += 1
    while len(dec_table) < 128:
        dec_table += [None]

    # Create the encode table
    enc_table = []
    seen = set()  # Used to eliminate duplicate code points.
    for item in table:
        if not item[1] in seen:
            seen &= {item[1]}
            enc_table += [(item[1], item[0] + 128)]
    enc_table.sort()

    # Write out shared code.
    out_file.write(
"""// This file is auto-generated.  Please see `encoding_tables/single_byte/`
// from the root directory for the files that generate this.

use {DecodeResult, EncodeResult};

pub fn decode_to_str<'a>(
    input: &[u8],
    out_buffer: &'a mut [u8],
    is_end: bool
) -> DecodeResult<'a> {
    super::ascii_ext_decode_to_str(&DECODE_TABLE, input, out_buffer, is_end)
}

pub fn encode_from_str<'a>(
    input: &str,
    out_buffer: &'a mut [u8],
    is_end: bool
) -> EncodeResult<'a> {
    super::ascii_ext_encode_from_str(&ENCODE_TABLE, input, out_buffer, is_end)
}\n
"""
    )

    # Write out decode table
    out_file.write("const DECODE_TABLE: [char; 128] = [")
    for (i, c) in enumerate(dec_table):
        if i % 8 == 0:
            out_file.write("\n    ")
        if c is None:
            out_file.write("'�',")
        else:
            out_file.write("'\\u{{{:04X}}}',".format(c))
    out_file.write("\n];\n\n")

    # Write out encode table
    out_file.write("const ENCODE_TABLE: [(char, u8); {}] = [".format(len(enc_table)))
    for (i, pair) in enumerate(enc_table):
        if i % 4 == 0:
            out_file.write("\n    ")
        out_file.write("('\\u{{{:04X}}}', 0x{:02X}), ".format(pair[0], pair[1]))
    out_file.write("\n];\n")


if __name__ == "__main__":
    root = "../../src/generated/single_byte"

    # Generate the table files.
    generate_ascii_ext_encoding(
        "index-ibm866.txt",
        root + "/ibm866_tables.rs.inc",
    )

    generate_ascii_ext_encoding(
        "index-ibm866.txt",
        root + "/ibm866_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-1.txt",
        root + "/iso-8859-1_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-2.txt",
        root + "/iso-8859-2_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-3.txt",
        root + "/iso-8859-3_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-4.txt",
        root + "/iso-8859-4_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-5.txt",
        root + "/iso-8859-5_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-6.txt",
        root + "/iso-8859-6_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-7.txt",
        root + "/iso-8859-7_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-8.txt",
        root + "/iso-8859-8_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-9.txt",
        root + "/iso-8859-9_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-10.txt",
        root + "/iso-8859-10_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-tis-620.txt",
        root + "/tis-620_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-13.txt",
        root + "/iso-8859-13_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-14.txt",
        root + "/iso-8859-14_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-15.txt",
        root + "/iso-8859-15_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-iso-8859-16.txt",
        root + "/iso-8859-16_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-koi8-r.txt",
        root + "/koi8-r_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-koi8-u.txt",
        root + "/koi8-u_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-macintosh.txt",
        root + "/macintosh_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-874.txt",
        root + "/windows-874_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-1250.txt",
        root + "/windows-1250_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-1251.txt",
        root + "/windows-1251_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-1252.txt",
        root + "/windows-1252_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-1253.txt",
        root + "/windows-1253_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-1254.txt",
        root + "/windows-1254_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-1255.txt",
        root + "/windows-1255_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-1256.txt",
        root + "/windows-1256_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-1257.txt",
        root + "/windows-1257_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-windows-1258.txt",
        root + "/windows-1258_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "index-x-mac-cyrillic.txt",
        root + "/x-mac-cyrillic_tables.rs.inc",
    )
