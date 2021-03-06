// This file is auto-generated.  Please see `encoding_tables/single_byte/`
// from the root directory for the files that generate this.

use {DecodeResult, EncodeResult};

pub fn decode_to_str<'a>(
    input: &[u8],
    out_buffer: &'a mut [u8],
) -> DecodeResult<'a> {
    super::ascii_ext_decode_to_str(&DECODE_TABLE, input, out_buffer)
}

pub fn encode_from_str<'a>(
    input: &str,
    out_buffer: &'a mut [u8],
) -> EncodeResult<'a> {
    super::ascii_ext_encode_from_str(&ENCODE_TABLE, input, out_buffer)
}

const DECODE_TABLE: [char; 128] = [
    '\u{0080}','\u{0081}','\u{0082}','\u{0083}','\u{0084}','\u{0085}','\u{0086}','\u{0087}',
    '\u{0088}','\u{0089}','\u{008A}','\u{008B}','\u{008C}','\u{008D}','\u{008E}','\u{008F}',
    '\u{0090}','\u{0091}','\u{0092}','\u{0093}','\u{0094}','\u{0095}','\u{0096}','\u{0097}',
    '\u{0098}','\u{0099}','\u{009A}','\u{009B}','\u{009C}','\u{009D}','\u{009E}','\u{009F}',
    '\u{00A0}','\u{0104}','\u{02D8}','\u{0141}','\u{00A4}','\u{013D}','\u{015A}','\u{00A7}',
    '\u{00A8}','\u{0160}','\u{015E}','\u{0164}','\u{0179}','\u{00AD}','\u{017D}','\u{017B}',
    '\u{00B0}','\u{0105}','\u{02DB}','\u{0142}','\u{00B4}','\u{013E}','\u{015B}','\u{02C7}',
    '\u{00B8}','\u{0161}','\u{015F}','\u{0165}','\u{017A}','\u{02DD}','\u{017E}','\u{017C}',
    '\u{0154}','\u{00C1}','\u{00C2}','\u{0102}','\u{00C4}','\u{0139}','\u{0106}','\u{00C7}',
    '\u{010C}','\u{00C9}','\u{0118}','\u{00CB}','\u{011A}','\u{00CD}','\u{00CE}','\u{010E}',
    '\u{0110}','\u{0143}','\u{0147}','\u{00D3}','\u{00D4}','\u{0150}','\u{00D6}','\u{00D7}',
    '\u{0158}','\u{016E}','\u{00DA}','\u{0170}','\u{00DC}','\u{00DD}','\u{0162}','\u{00DF}',
    '\u{0155}','\u{00E1}','\u{00E2}','\u{0103}','\u{00E4}','\u{013A}','\u{0107}','\u{00E7}',
    '\u{010D}','\u{00E9}','\u{0119}','\u{00EB}','\u{011B}','\u{00ED}','\u{00EE}','\u{010F}',
    '\u{0111}','\u{0144}','\u{0148}','\u{00F3}','\u{00F4}','\u{0151}','\u{00F6}','\u{00F7}',
    '\u{0159}','\u{016F}','\u{00FA}','\u{0171}','\u{00FC}','\u{00FD}','\u{0163}','\u{02D9}',
];

const ENCODE_TABLE: [(char, u8); 128] = [
    ('\u{0080}', 0x80), ('\u{0081}', 0x81), ('\u{0082}', 0x82), ('\u{0083}', 0x83), 
    ('\u{0084}', 0x84), ('\u{0085}', 0x85), ('\u{0086}', 0x86), ('\u{0087}', 0x87), 
    ('\u{0088}', 0x88), ('\u{0089}', 0x89), ('\u{008A}', 0x8A), ('\u{008B}', 0x8B), 
    ('\u{008C}', 0x8C), ('\u{008D}', 0x8D), ('\u{008E}', 0x8E), ('\u{008F}', 0x8F), 
    ('\u{0090}', 0x90), ('\u{0091}', 0x91), ('\u{0092}', 0x92), ('\u{0093}', 0x93), 
    ('\u{0094}', 0x94), ('\u{0095}', 0x95), ('\u{0096}', 0x96), ('\u{0097}', 0x97), 
    ('\u{0098}', 0x98), ('\u{0099}', 0x99), ('\u{009A}', 0x9A), ('\u{009B}', 0x9B), 
    ('\u{009C}', 0x9C), ('\u{009D}', 0x9D), ('\u{009E}', 0x9E), ('\u{009F}', 0x9F), 
    ('\u{00A0}', 0xA0), ('\u{00A4}', 0xA4), ('\u{00A7}', 0xA7), ('\u{00A8}', 0xA8), 
    ('\u{00AD}', 0xAD), ('\u{00B0}', 0xB0), ('\u{00B4}', 0xB4), ('\u{00B8}', 0xB8), 
    ('\u{00C1}', 0xC1), ('\u{00C2}', 0xC2), ('\u{00C4}', 0xC4), ('\u{00C7}', 0xC7), 
    ('\u{00C9}', 0xC9), ('\u{00CB}', 0xCB), ('\u{00CD}', 0xCD), ('\u{00CE}', 0xCE), 
    ('\u{00D3}', 0xD3), ('\u{00D4}', 0xD4), ('\u{00D6}', 0xD6), ('\u{00D7}', 0xD7), 
    ('\u{00DA}', 0xDA), ('\u{00DC}', 0xDC), ('\u{00DD}', 0xDD), ('\u{00DF}', 0xDF), 
    ('\u{00E1}', 0xE1), ('\u{00E2}', 0xE2), ('\u{00E4}', 0xE4), ('\u{00E7}', 0xE7), 
    ('\u{00E9}', 0xE9), ('\u{00EB}', 0xEB), ('\u{00ED}', 0xED), ('\u{00EE}', 0xEE), 
    ('\u{00F3}', 0xF3), ('\u{00F4}', 0xF4), ('\u{00F6}', 0xF6), ('\u{00F7}', 0xF7), 
    ('\u{00FA}', 0xFA), ('\u{00FC}', 0xFC), ('\u{00FD}', 0xFD), ('\u{0102}', 0xC3), 
    ('\u{0103}', 0xE3), ('\u{0104}', 0xA1), ('\u{0105}', 0xB1), ('\u{0106}', 0xC6), 
    ('\u{0107}', 0xE6), ('\u{010C}', 0xC8), ('\u{010D}', 0xE8), ('\u{010E}', 0xCF), 
    ('\u{010F}', 0xEF), ('\u{0110}', 0xD0), ('\u{0111}', 0xF0), ('\u{0118}', 0xCA), 
    ('\u{0119}', 0xEA), ('\u{011A}', 0xCC), ('\u{011B}', 0xEC), ('\u{0139}', 0xC5), 
    ('\u{013A}', 0xE5), ('\u{013D}', 0xA5), ('\u{013E}', 0xB5), ('\u{0141}', 0xA3), 
    ('\u{0142}', 0xB3), ('\u{0143}', 0xD1), ('\u{0144}', 0xF1), ('\u{0147}', 0xD2), 
    ('\u{0148}', 0xF2), ('\u{0150}', 0xD5), ('\u{0151}', 0xF5), ('\u{0154}', 0xC0), 
    ('\u{0155}', 0xE0), ('\u{0158}', 0xD8), ('\u{0159}', 0xF8), ('\u{015A}', 0xA6), 
    ('\u{015B}', 0xB6), ('\u{015E}', 0xAA), ('\u{015F}', 0xBA), ('\u{0160}', 0xA9), 
    ('\u{0161}', 0xB9), ('\u{0162}', 0xDE), ('\u{0163}', 0xFE), ('\u{0164}', 0xAB), 
    ('\u{0165}', 0xBB), ('\u{016E}', 0xD9), ('\u{016F}', 0xF9), ('\u{0170}', 0xDB), 
    ('\u{0171}', 0xFB), ('\u{0179}', 0xAC), ('\u{017A}', 0xBC), ('\u{017B}', 0xAF), 
    ('\u{017C}', 0xBF), ('\u{017D}', 0xAE), ('\u{017E}', 0xBE), ('\u{02C7}', 0xB7), 
    ('\u{02D8}', 0xA2), ('\u{02D9}', 0xFF), ('\u{02DB}', 0xB2), ('\u{02DD}', 0xBD), 
];
