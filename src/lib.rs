#[macro_use]
extern crate bitflags;
extern crate nom;

#[allow(non_snake_case)]
mod common;
mod TES4;

use nom::{
    IResult,
    bytes::complete::tag,
    bytes::complete::take,
    number::complete::le_u16,
    number::complete::le_u32,
};

macro_rules! optional_subrecord {
    ($subrecord:ty, $record:ident, $parser_fn:ident, $input:expr) => {{
        let (input, opt) = if let Ok((_input, data)) = $record::$parser_fn($input) {
            (_input, Some(data))
        } else {
            ($input, None)
        };

        (input, opt)
    }};
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], TES4::TES4> {
    let (input, _) = tag("TES4")(input)?;

    let (input, size) = le_u32(input)?;
    let (input, flags) = TES4::parse_header_flags(input)?;
    let (input, _) = take(4u8)(input)?;
    let (input, vc) = common::parse_version_control(input)?;
    let (input, version) = le_u16(input)?;
    let (input, unknown) = le_u16(input)?;

    let (input, hedr) = TES4::parse_hedr(input)?;
    let (input, cnam_opt) = optional_subrecord!(CNAM, TES4, parse_cnam, input);
    let (input, snam_opt) = optional_subrecord!(SNAM, TES4, parse_snam, input);
    let (input, mast_opt) = optional_subrecord!(MAST, TES4, parse_mast, input);
    let (input, onam_opt) = optional_subrecord!(ONAM, TES4, parse_onam, input);
    let (input, intv_opt) = optional_subrecord!(INTV, TES4, parse_intv, input);
    let (input, incc_opt) = optional_subrecord!(INCC, TES4, parse_incc, input);

    Ok((input, TES4::TES4{
        size: size,
        flags: flags,
        vc: vc,
        version: version,
        unknown: unknown,

        hedr: hedr,
        cnam: cnam_opt,
        snam: snam_opt,
        mast: mast_opt,
        onam: onam_opt,
        intv: intv_opt,
        incc: incc_opt,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_test() {
        let data = include_bytes!("../data/Update.esm");

        if let Ok((_, tes4)) = parse_header(data) {
            println!("{:?}", tes4);
        } else {
            println!("Failure parsing TES4 header!");
        }
    }
}
