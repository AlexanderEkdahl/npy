use std::io;
use std::io::Write;

pub trait Serializeable<const N: usize> {
    fn data_type() -> &'static [u8];
    fn to_bytes(&self) -> [u8; N];
}

pub fn save<T, O: Write, const N: usize>(output: &mut O, data: T, shape: &[usize]) -> Result<(), io::Error>
where
    T: IntoIterator,
    T::Item: Serializeable<N>,
{
    let header = build_header(shape, T::Item::data_type())?;
    let header_len = u16::try_from(header.len()).expect("header exceeds max length");

    output.write(b"\x93NUMPY\x01\x00")?;
    output.write(&header_len.to_le_bytes())?;
    output.write(&header)?;

    let mut n = 0;
    for x in data.into_iter() {
        output.write(&x.to_bytes())?;
        n += 1;
    }

    match shape.iter().product::<usize>().cmp(&n) {
        std::cmp::Ordering::Less => todo!(),
        std::cmp::Ordering::Equal => Ok(()),
        std::cmp::Ordering::Greater => todo!(),
    }
}

fn build_header(shape: &[usize], data_type: &'static [u8]) -> Result<Vec<u8>, io::Error> {
    let mut header = Vec::new();
    header.write(b"{'descr': '")?;
    if cfg!(target_endian = "big") {
        header.write(b">")?;
    } else {
        header.write(b"<")?;
    }
    header.write(data_type)?;
    header.write(b"', 'fortran_order': False, 'shape': (")?;
    for dim in shape.iter() {
        header.write(dim.to_string().as_bytes())?;
        header.write(b",")?;
    }
    header.write(b"), }")?;
    Ok(header)
}

macro_rules! impl_serializeable {
    ($prim:ty, $data_type:expr, $size:expr) => {
        impl Serializeable<$size> for $prim {
            fn data_type() -> &'static [u8] {
                $data_type
            }

            fn to_bytes(&self) -> [u8; $size] {
                self.to_ne_bytes()
            }
        }
    };
}

// TOOD: f2 (half)
impl_serializeable!(&f32, b"f4", 4);
impl_serializeable!(&f64, b"f8", 8);
impl_serializeable!(&u8, b"u1", 1);
impl_serializeable!(&u16, b"u2", 2);
impl_serializeable!(&u32, b"u4", 4);
impl_serializeable!(&u64, b"u8", 8);
impl_serializeable!(&i8, b"i1", 1);
impl_serializeable!(&i16, b"i2", 2);
impl_serializeable!(&i32, b"i4", 4);
impl_serializeable!(&i64, b"i8", 8);
