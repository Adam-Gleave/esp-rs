use nom::{
    do_parse,
    named,
    tag,
    take_while,
    number::complete::le_u8,
    number::complete::le_u16,
};

named!(pub parse_version_control<&[u8], VersionControl>, 
    do_parse!(day: le_u8 >> month: le_u8 >> last_user: le_u8 >> curr_user: le_u8 >>
    (VersionControl {
        day: day,
        month: month,
        last_user: last_user,
        curr_user: curr_user,
    }))
);

named!(pub parse_edid<&[u8], EDID>,
    do_parse!(
        _code: tag!("EDID") >> _size: le_u16 >> 
        edid_str: take_while!(|c: u8| c != 0) >> _delim: tag!([0]) >>
    (EDID {
        id: String::from_utf8(edid_str.to_vec()).unwrap(),
    }))
);

#[derive(Debug, PartialEq, Default)]
pub struct VersionControl {
    pub day: u8,
    pub month: u8,
    pub last_user: u8,
    pub curr_user: u8,
}

pub struct EDID {
    pub id: String,
}
