use nom::{
    do_parse,
    named,
    tag,
    number::complete::le_u8,
    number::complete::le_u16,
};

use super::common::EDID;

named!(pub parse_cnam<&[u8], CNAM>, 
    do_parse!(
        _code: tag!("CNAM") >> _size: le_u16 >>
        r: le_u8 >> g: le_u8 >> b: le_u8 >> a: le_u8 >>
    (CNAM { r: r, g: g, b: b, a: a, }))
);

pub struct CNAM {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct KYWD {
    pub editorID: common::EDID,
    pub color: CNAM,
}
