#!/usr/bin/env python3


def index_to_bytes(i):
    """ Map the WHATWG index back to the original ShiftJIS bytes.
    """
    lead = i // 188;
    lead_offset = 0x81 if lead < 0x1F else 0xC1
    trail = i % 188
    trail_offset = 0x40 if trail < 0x3F else 0x41
    return (lead + lead_offset, trail + trail_offset)


def bytes_to_index(lead, tail):
    """ Map a pair of ShiftJIS bytes to the WHATWG index.
    """
    lead_offset = 0x81 if lead < 0xA0 else 0xC1
    tail_offset = 0x40 if tail < 0x7F else 0x41
    return (lead - lead_offset) * 188 + tail - tail_offset


def deduped_table(table):
    """ De-deplicate the codepoint entries in the from-disk table according to
        WHATWG specs.
    """
    # Remove all indices in [8272, 8835].
    tmp_table_1 = [(i, c) for (i, c) in table if i < 8272 or i > 8835]

    # For the rest, remove all duplicates except the first.
    new_table = []
    seen = set()
    for (index, codepoint) in tmp_table_1:
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


def generate_shiftjis_tables(in_path, out_path):
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


def generate_shiftjis_test_data(in_path, dec_in_path, dec_out_path, enc_in_path, enc_out_path):
    table = load_table_file(in_path)

    # Generate decoding test files.
    decode_in_file = open(dec_in_path, "wb")
    decode_out_file = open(dec_out_path, "wb")
    for ascii_byte in range(1, 129):
        # In
        decode_in_file.write(bytes([ascii_byte]))
        decode_in_file.write(bytes("\n", 'utf-8'))
        # Out
        decode_out_file.write(chr(ascii_byte).encode('utf-8'))
        decode_out_file.write(bytes("\n", 'utf-8'))
    for (index, codepoint) in table:
        # In
        decode_in_file.write(bytes(index_to_bytes(index)))
        decode_in_file.write(bytes("\n", 'utf-8'))
        # Out
        decode_out_file.write(chr(codepoint).encode('utf-8'))
        decode_out_file.write(bytes("\n", 'utf-8'))

    # Generate encoding test files.
    encode_in_file = open(enc_in_path, "wb")
    encode_out_file = open(enc_out_path, "wb")
    for ascii_byte in range(1, 129):
        # In
        encode_in_file.write(chr(ascii_byte).encode('utf-8'))
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
    generate_shiftjis_tables(
        "index-jis0208.txt",
        table_root + "/shiftjis_whatwg_tables.rs.inc",
    )

    # Generate the test data files.
    generate_shiftjis_test_data(
        "index-jis0208.txt",
        test_root + "/shiftjis_whatwg_test_decode_in.txt",
        test_root + "/shiftjis_whatwg_test_decode_out.txt",
        test_root + "/shiftjis_whatwg_test_encode_in.txt",
        test_root + "/shiftjis_whatwg_test_encode_out.txt",
    )
