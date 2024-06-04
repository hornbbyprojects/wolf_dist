use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use fixed;

pub trait WolfSerialise: Sized {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()>;
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self>;
}

impl WolfSerialise for bool {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        let as_u8: u8 = if *self { 1 } else { 0 };
        as_u8.wolf_serialise(out_stream)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        let as_u8 = u8::wolf_deserialise(in_stream)?;
        Ok(if as_u8 == 0 { false } else { true })
    }
}
impl WolfSerialise for f32 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_f32::<BigEndian>(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_f32::<BigEndian>()
    }
}
impl WolfSerialise for f64 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_f64::<BigEndian>(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_f64::<BigEndian>()
    }
}
impl WolfSerialise for i64 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_i64::<BigEndian>(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_i64::<BigEndian>()
    }
}
impl WolfSerialise for i32 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_i32::<BigEndian>(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_i32::<BigEndian>()
    }
}
impl WolfSerialise for i16 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_i16::<BigEndian>(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_i16::<BigEndian>()
    }
}
impl WolfSerialise for i8 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_i8(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_i8()
    }
}
impl WolfSerialise for u64 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_u64::<BigEndian>(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_u64::<BigEndian>()
    }
}
impl WolfSerialise for u32 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_u32::<BigEndian>(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_u32::<BigEndian>()
    }
}
impl WolfSerialise for u16 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_u16::<BigEndian>(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_u16::<BigEndian>()
    }
}
impl WolfSerialise for u8 {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_u8(*self)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        in_stream.read_u8()
    }
}

impl<T: WolfSerialise> WolfSerialise for Vec<T> {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_u32::<BigEndian>(self.len() as u32)?;
        for item in self.iter() {
            item.wolf_serialise(out_stream)?;
        }
        Ok(())
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        let mut ret = Vec::new();
        let length = in_stream.read_u32::<BigEndian>()?;
        for _ in 0..length {
            let member = T::wolf_deserialise(in_stream)?;
            ret.push(member);
        }
        Ok(ret)
    }
}

impl<T: WolfSerialise + Sized, const L: usize> WolfSerialise for [T; L] {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        for member in self.iter() {
            member.wolf_serialise(out_stream)?;
        }
        Ok(())
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        let mut ret: [std::mem::MaybeUninit<T>; L] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..L {
            let next_obj = T::wolf_deserialise(in_stream);
            match next_obj {
                Ok(next_obj) => {
                    ret[i] = std::mem::MaybeUninit::new(next_obj);
                }
                Err(err) => unsafe {
                    //clean up memory
                    for j in 0..i {
                        let mut obj = std::mem::MaybeUninit::uninit();
                        std::mem::swap(&mut obj, &mut ret[j]);
                        let _to_drop = obj.assume_init();
                    }
                    return Err(err);
                },
            }
        }
        let initialised = unsafe {
            let initialised = std::mem::transmute_copy(&ret);
            std::mem::forget(ret);
            initialised
        };
        Ok(initialised)
    }
}

impl<Frac> WolfSerialise for fixed::FixedI64<Frac> {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        let bits: i64 = self.to_bits();
        bits.wolf_serialise(out_stream)
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        let bits = i64::wolf_deserialise(in_stream)?;
        Ok(Self::from_bits(bits))
    }
}

macro_rules! tuple_serialise {
    ($($x:ident),+ : $($num:tt),+) => {
        impl <$($x: WolfSerialise),+> WolfSerialise for ($($x),+) {
            fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
                $(self.$num.wolf_serialise(out_stream)?);+;
                Ok(())
            }
            fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
                Ok((
                    $($x::wolf_deserialise(in_stream)?),+
                ))
            }
        }
    };
}
tuple_serialise!(T1, T2 : 0, 1);
tuple_serialise!(T1, T2, T3 : 0, 1, 2);

impl<T: WolfSerialise> WolfSerialise for Option<T> {
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        let is_none = bool::wolf_deserialise(in_stream)?;
        if is_none {
            Ok(None)
        } else {
            let inner = T::wolf_deserialise(in_stream)?;
            Ok(Some(inner))
        }
    }
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        self.is_none().wolf_serialise(out_stream)?;
        if let Some(inner) = self {
            inner.wolf_serialise(out_stream)?;
        }
        Ok(())
    }
}

impl WolfSerialise for String {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        let bytes = self.clone().into_bytes();
        bytes.wolf_serialise(out_stream)?;
        Ok(())
    }

    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        Self::from_utf8(WolfSerialise::wolf_deserialise(in_stream)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
#[cfg(test)]
mod tests {
    use crate::WolfSerialise;
    fn test_cycle<T: WolfSerialise + Eq + std::fmt::Debug>(x: T) {
        let buffer_1: Vec<u8> = Vec::new();
        let mut cursor_1 = std::io::Cursor::new(buffer_1);
        x.wolf_serialise(&mut cursor_1).unwrap();
        let buffer_2 = cursor_1.into_inner();
        let mut cursor_2 = std::io::Cursor::new(buffer_2);
        let result = T::wolf_deserialise(&mut cursor_2).unwrap();
        assert_eq!(x, result);
        let remaining_bytes = std::io::Read::read_to_end(&mut cursor_2, &mut Vec::new()).unwrap();
        assert_eq!(remaining_bytes, 0);
    }
    #[test]
    fn test_serialise_option() {
        let x1: Option<i32> = Some(5);
        let x2: Option<i32> = None;
        test_cycle(x1);
        test_cycle(x2);
    }
    #[test]
    fn test_serialise_tuple() {
        let x1: (i32, i32) = (5, 6);
        let x2: (i32, i32, u32) = (5, 6, 7);
        test_cycle(x1);
        test_cycle(x2);
    }
    #[test]
    fn test_serialise_u8() {
        test_cycle(5u8);
    }
    #[test]
    fn test_serialise_u16() {
        test_cycle(5u16);
    }
    #[test]
    fn test_serialise_u32() {
        test_cycle(5u32);
    }
    #[test]
    fn test_serialise_string() {
        test_cycle("Hello, world!".to_string());
    }
}
