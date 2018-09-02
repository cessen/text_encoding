// This file is auto-generated.  Please see `encoding_tables/single_byte/`
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
}

const DECODE_TABLE: [char; 128] = [
    '\u{20AC}','�','�','�','�','\u{2026}','�','�',
    '�','�','�','�','�','�','�','�',
    '�','\u{2018}','\u{2019}','\u{201C}','\u{201D}','\u{2022}','\u{2013}','\u{2014}',
    '�','�','�','�','�','�','�','�',
    '\u{00A0}','\u{0E01}','\u{0E02}','\u{0E03}','\u{0E04}','\u{0E05}','\u{0E06}','\u{0E07}',
    '\u{0E08}','\u{0E09}','\u{0E0A}','\u{0E0B}','\u{0E0C}','\u{0E0D}','\u{0E0E}','\u{0E0F}',
    '\u{0E10}','\u{0E11}','\u{0E12}','\u{0E13}','\u{0E14}','\u{0E15}','\u{0E16}','\u{0E17}',
    '\u{0E18}','\u{0E19}','\u{0E1A}','\u{0E1B}','\u{0E1C}','\u{0E1D}','\u{0E1E}','\u{0E1F}',
    '\u{0E20}','\u{0E21}','\u{0E22}','\u{0E23}','\u{0E24}','\u{0E25}','\u{0E26}','\u{0E27}',
    '\u{0E28}','\u{0E29}','\u{0E2A}','\u{0E2B}','\u{0E2C}','\u{0E2D}','\u{0E2E}','\u{0E2F}',
    '\u{0E30}','\u{0E31}','\u{0E32}','\u{0E33}','\u{0E34}','\u{0E35}','\u{0E36}','\u{0E37}',
    '\u{0E38}','\u{0E39}','\u{0E3A}','�','�','�','�','\u{0E3F}',
    '\u{0E40}','\u{0E41}','\u{0E42}','\u{0E43}','\u{0E44}','\u{0E45}','\u{0E46}','\u{0E47}',
    '\u{0E48}','\u{0E49}','\u{0E4A}','\u{0E4B}','\u{0E4C}','\u{0E4D}','\u{0E4E}','\u{0E4F}',
    '\u{0E50}','\u{0E51}','\u{0E52}','\u{0E53}','\u{0E54}','\u{0E55}','\u{0E56}','\u{0E57}',
    '\u{0E58}','\u{0E59}','\u{0E5A}','\u{0E5B}','�','�','�','�',
];

const ENCODE_TABLE: [(char, u8); 97] = [
    ('\u{00A0}', 0xA0), ('\u{0E01}', 0xA1), ('\u{0E02}', 0xA2), ('\u{0E03}', 0xA3), 
    ('\u{0E04}', 0xA4), ('\u{0E05}', 0xA5), ('\u{0E06}', 0xA6), ('\u{0E07}', 0xA7), 
    ('\u{0E08}', 0xA8), ('\u{0E09}', 0xA9), ('\u{0E0A}', 0xAA), ('\u{0E0B}', 0xAB), 
    ('\u{0E0C}', 0xAC), ('\u{0E0D}', 0xAD), ('\u{0E0E}', 0xAE), ('\u{0E0F}', 0xAF), 
    ('\u{0E10}', 0xB0), ('\u{0E11}', 0xB1), ('\u{0E12}', 0xB2), ('\u{0E13}', 0xB3), 
    ('\u{0E14}', 0xB4), ('\u{0E15}', 0xB5), ('\u{0E16}', 0xB6), ('\u{0E17}', 0xB7), 
    ('\u{0E18}', 0xB8), ('\u{0E19}', 0xB9), ('\u{0E1A}', 0xBA), ('\u{0E1B}', 0xBB), 
    ('\u{0E1C}', 0xBC), ('\u{0E1D}', 0xBD), ('\u{0E1E}', 0xBE), ('\u{0E1F}', 0xBF), 
    ('\u{0E20}', 0xC0), ('\u{0E21}', 0xC1), ('\u{0E22}', 0xC2), ('\u{0E23}', 0xC3), 
    ('\u{0E24}', 0xC4), ('\u{0E25}', 0xC5), ('\u{0E26}', 0xC6), ('\u{0E27}', 0xC7), 
    ('\u{0E28}', 0xC8), ('\u{0E29}', 0xC9), ('\u{0E2A}', 0xCA), ('\u{0E2B}', 0xCB), 
    ('\u{0E2C}', 0xCC), ('\u{0E2D}', 0xCD), ('\u{0E2E}', 0xCE), ('\u{0E2F}', 0xCF), 
    ('\u{0E30}', 0xD0), ('\u{0E31}', 0xD1), ('\u{0E32}', 0xD2), ('\u{0E33}', 0xD3), 
    ('\u{0E34}', 0xD4), ('\u{0E35}', 0xD5), ('\u{0E36}', 0xD6), ('\u{0E37}', 0xD7), 
    ('\u{0E38}', 0xD8), ('\u{0E39}', 0xD9), ('\u{0E3A}', 0xDA), ('\u{0E3F}', 0xDF), 
    ('\u{0E40}', 0xE0), ('\u{0E41}', 0xE1), ('\u{0E42}', 0xE2), ('\u{0E43}', 0xE3), 
    ('\u{0E44}', 0xE4), ('\u{0E45}', 0xE5), ('\u{0E46}', 0xE6), ('\u{0E47}', 0xE7), 
    ('\u{0E48}', 0xE8), ('\u{0E49}', 0xE9), ('\u{0E4A}', 0xEA), ('\u{0E4B}', 0xEB), 
    ('\u{0E4C}', 0xEC), ('\u{0E4D}', 0xED), ('\u{0E4E}', 0xEE), ('\u{0E4F}', 0xEF), 
    ('\u{0E50}', 0xF0), ('\u{0E51}', 0xF1), ('\u{0E52}', 0xF2), ('\u{0E53}', 0xF3), 
    ('\u{0E54}', 0xF4), ('\u{0E55}', 0xF5), ('\u{0E56}', 0xF6), ('\u{0E57}', 0xF7), 
    ('\u{0E58}', 0xF8), ('\u{0E59}', 0xF9), ('\u{0E5A}', 0xFA), ('\u{0E5B}', 0xFB), 
    ('\u{2013}', 0x96), ('\u{2014}', 0x97), ('\u{2018}', 0x91), ('\u{2019}', 0x92), 
    ('\u{201C}', 0x93), ('\u{201D}', 0x94), ('\u{2022}', 0x95), ('\u{2026}', 0x85), 
    ('\u{20AC}', 0x80), 
];
