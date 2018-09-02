#!/usr/bin/env python3

def generate_ascii_ext_encoding(in_path, out_path):
    in_file = open(in_path)
    out_file = open(out_path, mode='w')

    # Load the table from the file.
    table = []
    for line in in_file:
        parts = line.split()
        if len(parts) >= 2 and not parts[0].startswith("#"):
            byte = int(parts[0], 16)
            if parts[1].startswith("#"):
                continue
            unicode = int(parts[1], 16)
            table += [(byte, unicode)]
    table.sort()

    # Verify that it is indeed an ascii extension (i.e. all bytes <= 127
    # should exist and map 1-to-1 to the unicode codepoints with the same
    # value.
    for i in range(0, 128):
        if table[i][0] != i or table[i][1] != i:
            raise Exception("Not an ascii extension: file {}, codepoint {}".format(in_path, i))


    # Truncate to just the non-ascii entries.
    table = table[128:]

    # Create the decode table
    dec_table = []
    i = 0
    for item in table:
        while i < (item[0] - 128):
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
            enc_table += [(item[1], item[0])]
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
        "ascii_ext/ibm-cp866.txt",
        root + "/ibm-cp866_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-1.txt",
        root + "/iso-8859-1_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-2.txt",
        root + "/iso-8859-2_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-3.txt",
        root + "/iso-8859-3_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-4.txt",
        root + "/iso-8859-4_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-5.txt",
        root + "/iso-8859-5_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-6.txt",
        root + "/iso-8859-6_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-7.txt",
        root + "/iso-8859-7_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-8.txt",
        root + "/iso-8859-8_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-9.txt",
        root + "/iso-8859-9_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-10.txt",
        root + "/iso-8859-10_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-11.txt",
        root + "/iso-8859-11_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-13.txt",
        root + "/iso-8859-13_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-14.txt",
        root + "/iso-8859-14_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-15.txt",
        root + "/iso-8859-15_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/iso-8859-16.txt",
        root + "/iso-8859-16_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/koi8-r.txt",
        root + "/koi8-r_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/koi8-u.txt",
        root + "/koi8-u_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/mac-roman.txt",
        root + "/mac-roman_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/mac-cyrillic.txt",
        root + "/mac-cyrillic_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp874.txt",
        root + "/windows-cp874_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp1250.txt",
        root + "/windows-cp1250_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp1251.txt",
        root + "/windows-cp1251_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp1252.txt",
        root + "/windows-cp1252_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp1253.txt",
        root + "/windows-cp1253_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp1254.txt",
        root + "/windows-cp1254_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp1255.txt",
        root + "/windows-cp1255_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp1256.txt",
        root + "/windows-cp1256_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp1257.txt",
        root + "/windows-cp1257_tables.rs.inc",
    )
    generate_ascii_ext_encoding(
        "ascii_ext/windows-cp1258.txt",
        root + "/windows-cp1258_tables.rs.inc",
    )
