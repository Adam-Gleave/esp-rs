use nom::{
    count,
    do_parse,
    many0,
    tag,
    take_while,
    named,
    number::complete::le_f32,
    number::complete::le_i32,
    number::complete::le_u8,
    number::complete::le_u16,
    number::complete::le_u32,
    number::complete::le_u64,
};

bitflags! {
    #[derive(Default)]
    pub struct TES4Flags: u32 {
        const MASTER    = 0x00000001;
        const LOCALIZED = 0x00000080;
        const LIGHT     = 0x00000200;
    }
}

named!(pub parse_header_flags<&[u8], TES4Flags>,
    do_parse!(val: le_u32 >> (TES4Flags::from_bits(val).unwrap()))
);

named!(pub parse_version_control<&[u8], VersionControl>, 
    do_parse!(day: le_u8 >> month: le_u8 >> last_user: le_u8 >> curr_user: le_u8 >>
    (VersionControl {
        day: day,
        month: month,
        last_user: last_user,
        curr_user: curr_user,
    }))
);

named!(pub parse_hedr<&[u8], HEDR>,
    do_parse!(
        _code: tag!("HEDR") >> _size: le_u16 >> version: le_f32 >> 
        num_records: le_i32 >> next_obj_id: le_u32 >> 
    (HEDR {
        version: version,
        num_records: num_records,
        next_obj_id: next_obj_id,
    }))
);

named!(pub parse_cnam<&[u8], CNAM>,
    do_parse!(
        _code: tag!("CNAM") >> _size: le_u16 >> 
        cnam_str: take_while!(|c: u8| c != 0) >> _delim: tag!([0]) >>
    (CNAM {
        author: String::from_utf8(cnam_str.to_vec()).unwrap(),
    }))
);

named!(pub parse_snam<&[u8], SNAM>,
    do_parse!(
        _code: tag!("SNAM") >> _size: le_u16 >> 
        snam_str: take_while!(|c: u8| c != 0) >> _delim: tag!([0]) >>
    (SNAM {
        desc: String::from_utf8(snam_str.to_vec()).unwrap(),
    }))
);

named!(pub parse_mast<&[u8], Vec<MAST>>,
    many0!(do_parse!(
        _code: tag!("MAST") >> _size: le_u16 >>
        mast_str: take_while!(|c: u8| c != 0) >> _delim: tag!([0]) >>
        _data: tag!("DATA") >> _size: le_u16 >> _zero: le_u64 >>
    (MAST {
        master: String::from_utf8(mast_str.to_vec()).unwrap(),
    })))
);

named!(pub parse_onam<&[u8], ONAM>,
    do_parse!(
        _code: tag!("ONAM") >> size: le_u16 >> 
        overrides: count!(le_u32, (size/4) as usize) >>
    (ONAM {
        overrides: overrides,
    }))
);

named!(pub parse_intv<&[u8], INTV>,
    do_parse!(
        _code: tag!("INTV") >> _size: le_u16 >> internal_version: le_u32 >>
    (INTV {
        internal_version: internal_version,
    }))
);

named!(pub parse_incc<&[u8], INCC>,
    do_parse!(
        _code: tag!("INCC") >> _size: le_u16 >> incc: le_u32 >>
    (INCC {
        incc: incc,
    }))
);

#[derive(Debug, PartialEq, Default)]
pub struct VersionControl {
    pub day: u8,
    pub month: u8,
    pub last_user: u8,
    pub curr_user: u8,
}

#[derive(Debug, PartialEq, Default)]
pub struct HEDR {
    pub version: f32,
    pub num_records: i32,
    pub next_obj_id: u32,
}

#[derive(Debug, PartialEq, Default)]
pub struct CNAM {
    pub author: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct SNAM {
    pub desc: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct MAST {
    pub master: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct ONAM {
    pub overrides: Vec<u32>,
}

#[derive(Debug, PartialEq, Default)]
pub struct INTV {
    pub internal_version: u32,
}

#[derive(Debug, PartialEq, Default)]
pub struct INCC {
    pub incc: u32,
}

#[derive(Debug, PartialEq, Default)]
pub struct TES4 {
    pub size: u32,
    pub flags: TES4Flags,
    pub vc: VersionControl,
    pub version: u16,
    pub unknown: u16,

    pub hedr: HEDR,
    pub cnam: Option<CNAM>,
    pub snam: Option<SNAM>,
    pub mast: Option<Vec<MAST>>,
    pub onam: Option<ONAM>,
    pub intv: Option<INTV>,
    pub incc: Option<INCC>,
}
