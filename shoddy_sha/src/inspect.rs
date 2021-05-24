use std::fmt;
use std::fmt::Formatter;

/// Struct representing the decomposed
/// version of a datatype
pub struct Bytefield<T>(pub T);

impl Bytefield<f64> {
    fn decompose(&self) -> (u64, u64, u64) {
        let bits = self.0.to_bits();
        let sign = (bits >> 63) & 0x1_u64;
        let exponent = (bits >> 52) & 0x07ff_u64;
        let mantissa = bits & 0xf_ffff_ffff_ffff_u64;

        (sign, exponent, mantissa)
    }
}

impl fmt::LowerHex for Bytefield<f64> {
    fn fmt(&self,f: &mut Formatter<'_>) -> fmt::Result {
        let (sign, exponent, mantissa) = self.decompose();
        f.write_fmt(format_args!("{:x} {:03x} {:013x}", sign, exponent, mantissa))
    }
}

impl fmt::Binary for Bytefield<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (sign, exponent, mantissa) = self.decompose();
        f.write_fmt(format_args!("{:b} {:011b} {:052b}", sign, exponent, mantissa))
    }
}
