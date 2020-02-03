#[macro_use]
extern crate bitflags;
extern crate nom;

use nom::{
    IResult,
    bytes::complete::tag,
    bytes::complete::take,
    bytes::complete::take_while,
    number::complete::le_f32,
    number::complete::le_i32,
    number::complete::le_u8,
    number::complete::le_u16,
    number::complete::le_u32,
};

bitflags! {
    #[derive(Default)]
    pub struct TES4Flags: u32 {
        const MASTER    = 0x00000001;
        const LOCALIZED = 0x00000080;
        const LIGHT     = 0x00000200;
    }
}

pub fn parse_header_flags(input: &[u8]) -> IResult<&[u8], TES4Flags> {
    let (input, val) = le_u32(input)?;
    let flags = TES4Flags::from_bits(val).expect("Cannot parse TES4 header flags.");

    Ok((input, flags))
}

#[derive(Debug, PartialEq, Default)]
pub struct VersionControl {
    pub day: u8,
    pub month: u8,
    pub last_user: u8,
    pub curr_user: u8,
}

pub fn parse_version_control(input: &[u8]) -> IResult<&[u8], VersionControl> {
    let (input, day) = le_u8(input)?;
    let (input, month) = le_u8(input)?;
    let (input, last_user) = le_u8(input)?;
    let (input, curr_user) = le_u8(input)?;

    Ok((input, VersionControl{
        day: day,
        month: month,
        last_user: last_user,
        curr_user: curr_user,
    }))
}

#[derive(Debug, PartialEq, Default)]
pub struct HEDR {
    pub version: f32,
    pub num_records: i32,
    pub next_obj_id: u32,
}

pub fn parse_hedr(input: &[u8]) -> IResult<&[u8], HEDR> {
    let (input, _) = tag("HEDR")(input)?;
    let (input, _) = take(2u8)(input)?;

    let (input, version) = le_f32(input)?;
    let (input, num_records) = le_i32(input)?;
    let (input, next_obj_id) = le_u32(input)?;

    Ok((input, HEDR{
        version: version,
        num_records: num_records,
        next_obj_id: next_obj_id,
    }))
}

#[derive(Debug, PartialEq, Default)]
pub struct CNAM {
    pub author: String,
}

pub fn parse_cnam(input: &[u8]) -> IResult<&[u8], CNAM> {
    let (input, _) = tag("CNAM")(input)?;
    let (input, _) = take(2u8)(input)?;
    let (input, cnam_data) = take_while(|c: u8| c != 0)(input)?;
    let (input, _) = tag([0])(input)?;

    Ok((input, CNAM{
        author: String::from_utf8(cnam_data.to_vec()).unwrap(),
    }))
}

#[derive(Debug, PartialEq, Default)]
pub struct SNAM {
    pub author: String,
}

pub fn parse_snam(input: &[u8]) -> IResult<&[u8], SNAM> {
    let (input, _) = tag("SNAM")(input)?;
    let (input, _) = take(2u8)(input)?;
    let (input, snam_data) = take_while(|c: u8| c != 0)(input)?;
    let (input, _) = tag([0])(input)?;

    Ok((input, SNAM{
        author: String::from_utf8(snam_data.to_vec()).unwrap(),
    }))
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
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], TES4> {
    let (input, _) = tag("TES4")(input)?;

    let (input, size) = le_u32(input)?;
    let (input, flags) = parse_header_flags(input)?;
    let (input, _) = take(4u8)(input)?;
    let (input, vc) = parse_version_control(input)?;
    let (input, version) = le_u16(input)?;
    let (input, unknown) = le_u16(input)?;

    let (input, hedr) = parse_hedr(input)?;
    let cnam_opt = if let Ok((_input, cnam)) = parse_cnam(input) {
        Some(cnam)
    } else {
        None
    };
    let snam_opt = if let Ok((_input, snam)) = parse_snam(input) {
        Some(snam)
    } else {
        None
    };

    Ok((input, TES4{
        size: size,
        flags: flags,
        vc: vc,
        version: version,
        unknown: unknown,

        hedr: hedr,
        cnam: cnam_opt,
        snam: snam_opt,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_test() {
        let data = include_bytes!("../data/Skyrim.esm");

        if let Ok((_, tes4)) = parse_header(data) {
            println!("{:?}", tes4);
        } else {
            println!("Failure parsing TES4 header!");
        }
    }
}