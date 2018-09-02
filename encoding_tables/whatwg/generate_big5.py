#!/usr/bin/env python3


def index_to_bytes(i):
    """ Map the WHATWG index back to the original BIG5 bytes.
    """
    lead = i // 157 + 0x81
    trail = i % 157
    offset = 0x40 if trail < 0x3f else 0x62
    return (lead, trail + offset)


def bytes_to_index(lead, tail):
    """ Map a pair of BIG5 bytes to the WHATWG index.
    """
    offset = 0x40 if tail < 0x7F else 0x62
    return (lead - 0x81) * 157 + (tail - offset)
    

def deduped_table(table):
    """ De-deplicate the codepoint entries in the from-disk table according to
        WHATWG specs.
    """
    tmp_table_1 = [x for x in table]

    # Remove all except the _last_ entries for code points U+2550, U+255E,
    # U+2561, U+256A, U+5341, and U+5345.
    tmp_table_1.reverse()
    tmp_table_2 = []
    seen = set()
    for (index, codepoint) in tmp_table_1:
        if codepoint not in seen:
            if codepoint in [0x2550, 0x255E, 0x2561, 0x256A, 0x5341, 0x5345]:
                seen.add(codepoint)
            tmp_table_2 += [(index, codepoint)]
    tmp_table_2.reverse()

    # For the rest, remove all except the _first_ entries.
    new_table = []
    seen = set()
    for (index, codepoint) in tmp_table_2:
        if codepoint not in seen:
            seen.add(codepoint)
            new_table += [(index, codepoint)]

    return new_table


def load_table_file(path):
    in_file = open(path)
    table = []
    for line in in_file:
        parts = line.split()
        if len(parts) >= 2 and not parts[0].startswith("#"):
            byte = int(parts[0])
            unicode = int(parts[1], 16)
            table += [(byte, unicode)]
    table.sort()
    return table


def generate_big5_tables(in_path, out_path):
    table = load_table_file(in_path)

    # Create the decode table
    dec_table = []
    i = 0
    for (index, codepoint) in table:
        while i < index:
            dec_table += [None]
            i += 1
        dec_table += [codepoint]
        i += 1

    # Create the encode table
    enc_table = [(codepoint, index_to_bytes(index)) for (index, codepoint) in deduped_table(table)]
    enc_table.sort()

    # Write file.
    out_file = open(out_path, mode='w')
    out_file.write(
"""// This file is auto-generated.  Please see `encoding_tables/whatwg/`
// from the root directory for the files that generate this.

"""
    )

    # Write out decode table
    out_file.write("const DECODE_TABLE: [char; {}] = [".format(len(dec_table)))
    for (i, c) in enumerate(dec_table):
        if i % 8 == 0:
            out_file.write("\n    ")
        if c is None:
            out_file.write("'�',")
        else:
            out_file.write("'\\u{{{:04X}}}',".format(c))
    out_file.write("\n];\n\n")

    # Write out encode table
    out_file.write("const ENCODE_TABLE: [(char, [u8; 2]); {}] = [".format(len(enc_table)))
    for (i, pair) in enumerate(enc_table):
        if i % 3 == 0:
            out_file.write("\n    ")
        out_file.write("('\\u{{{:X}}}', [0x{:X}, 0x{:X}]), ".format(pair[0], pair[1][0], pair[1][1]))
    out_file.write("\n];\n")


def generate_big5_test_data(in_path, dec_in_path, dec_out_path, enc_in_path, enc_out_path):
    table = load_table_file(in_path)

    # Generate decoding test files.
    decode_in_file = open(dec_in_path, "wb")
    decode_out_file = open(dec_out_path, "wb")
    for ascii_byte in range(1, 128):
        # In
        decode_in_file.write(bytes([ascii_byte]))
        decode_in_file.write(bytes("\n", 'utf-8'))
        # Out
        decode_out_file.write(bytes([ascii_byte]))
        decode_out_file.write(bytes("\n", 'utf-8'))
    for (index, codepoint) in table:
        # In
        decode_in_file.write(bytes(index_to_bytes(index)))
        decode_in_file.write(bytes("\n", 'utf-8'))
        # Out
        decode_out_file.write(chr(codepoint).encode('utf-8'))
        decode_out_file.write(bytes("\n", 'utf-8'))
    for (index, codepoints) in [
        (1133, (0xCA, 0x304)),
        (1135, (0xCA, 0x30C)),
        (1164, (0xEA, 0x304)),
        (1166, (0xEA, 0x30C)),
    ]:
        # In
        decode_in_file.write(bytes(index_to_bytes(index)))
        decode_in_file.write(bytes("\n", 'utf-8'))
        # Out
        decode_out_file.write(chr(codepoints[0]).encode('utf-8'))
        decode_out_file.write(chr(codepoints[1]).encode('utf-8'))
        decode_out_file.write(bytes("\n", 'utf-8'))

    # Generate encoding test files.
    encode_in_file = open(enc_in_path, "wb")
    encode_out_file = open(enc_out_path, "wb")
    for ascii_byte in range(1, 128):
        # In
        encode_in_file.write(bytes([ascii_byte]))
        encode_in_file.write(bytes("\n", 'utf-8'))
        # Out
        encode_out_file.write(bytes([ascii_byte]))
        encode_out_file.write(bytes("\n", 'utf-8'))
    for (index, codepoint) in deduped_table(table):
        # In
        encode_in_file.write(chr(codepoint).encode('utf-8'))
        encode_in_file.write(bytes("\n", 'utf-8'))
        # Out
        encode_out_file.write(bytes(index_to_bytes(index)))
        encode_out_file.write(bytes("\n", 'utf-8'))


if __name__ == "__main__":
    table_root = "../../src/generated/whatwg"
    test_root = "../../tests/test_data/whatwg"

    # Generate the table files.
    generate_big5_tables(
        "index-big5.txt",
        table_root + "/big5_whatwg_tables.rs.inc",
    )

    # Generate the test data files.
    generate_big5_test_data(
        "index-big5.txt",
        test_root + "/big5_whatwg_test_decode_in.txt",
        test_root + "/big5_whatwg_test_decode_out.txt",
        test_root + "/big5_whatwg_test_encode_in.txt",
        test_root + "/big5_whatwg_test_encode_out.txt",
    )        
