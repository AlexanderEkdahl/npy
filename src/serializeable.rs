use std::io;
use std::io::Write;

use crate::Serializeable;

pub struct Reshaped<T: Serializeable> {
    data: T,
    shape: Vec<usize>,
}

pub fn reshape<T: Serializeable>(data: T, shape: Vec<usize>) -> Reshaped<T> {
    Reshaped { data, shape }
}

impl<T> Serializeable for Reshaped<T>
where
    T: Serializeable,
{
    fn data_type(&self) -> &'static [u8] {
        self.data.data_type()
    }

    fn write<O: Write>(&self, output: &mut O) -> Result<(), io::Error> {
        self.data.write(output)
    }

    fn data_shape(&self) -> Vec<usize> {
        self.shape.clone()
    }
}

macro_rules! impl_serializeable_single {
    ($prim:ty, $data_type:expr) => {
        impl Serializeable for $prim {
            fn data_type(&self) -> &'static [u8] {
                $data_type
            }

            fn write<O: Write>(&self, output: &mut O) -> Result<(), io::Error> {
                output.write_all(&self.to_ne_bytes())
            }

            fn data_shape(&self) -> Vec<usize> {
                Vec::new()
            }
        }
    };
}

impl_serializeable_single!(f32, b"f4");
impl_serializeable_single!(f64, b"f8");
impl_serializeable_single!(u8, b"u1");
impl_serializeable_single!(u16, b"u2");
impl_serializeable_single!(u32, b"u4");
impl_serializeable_single!(u64, b"u8");
impl_serializeable_single!(i8, b"i1");
impl_serializeable_single!(i16, b"i2");
impl_serializeable_single!(i32, b"i4");
impl_serializeable_single!(i64, b"i8");

macro_rules! impl_serializeable_1d_array {
    ($type:ty, $data_type:expr) => {
        impl Serializeable for $type {
            fn data_type(&self) -> &'static [u8] {
                $data_type
            }

            fn write<O: Write>(&self, output: &mut O) -> Result<(), io::Error> {
                for x in self.iter() {
                    output.write_all(&x.to_ne_bytes())?;
                }
                Ok(())
            }

            fn data_shape(&self) -> Vec<usize> {
                vec![self.len()]
            }
        }
    };
}

impl_serializeable_1d_array!(&[f32], b"f4");
impl_serializeable_1d_array!(&[f64], b"f8");
impl_serializeable_1d_array!(&[u8], b"u1");
impl_serializeable_1d_array!(&[u16], b"u2");
impl_serializeable_1d_array!(&[u32], b"u4");
impl_serializeable_1d_array!(&[u64], b"u8");
impl_serializeable_1d_array!(&[i8], b"i1");
impl_serializeable_1d_array!(&[i16], b"i2");
impl_serializeable_1d_array!(&[i32], b"i4");
impl_serializeable_1d_array!(&[i64], b"i8");

macro_rules! impl_serializeable_2d_array {
    ($type:ty, $data_type:expr) => {
        impl<const N: usize> Serializeable for $type {
            fn data_type(&self) -> &'static [u8] {
                $data_type
            }

            fn write<O: Write>(&self, output: &mut O) -> Result<(), io::Error> {
                for x in self.iter() {
                    for y in x.iter() {
                        output.write_all(&y.to_ne_bytes())?;
                    }
                }
                Ok(())
            }

            fn data_shape(&self) -> Vec<usize> {
                vec![self.len(), N]
            }
        }
    };
}

impl_serializeable_2d_array!(&[[f32; N]], b"f4");
impl_serializeable_2d_array!(&[[f64; N]], b"f8");
impl_serializeable_2d_array!(&[[u8; N]], b"u1");
impl_serializeable_2d_array!(&[[u16; N]], b"u2");
impl_serializeable_2d_array!(&[[u32; N]], b"u4");
impl_serializeable_2d_array!(&[[u64; N]], b"u8");
impl_serializeable_2d_array!(&[[i8; N]], b"i1");
impl_serializeable_2d_array!(&[[i16; N]], b"i2");
impl_serializeable_2d_array!(&[[i32; N]], b"i4");
impl_serializeable_2d_array!(&[[i64; N]], b"i8");

macro_rules! impl_serializeable_2d_tuple {
    ($type:ty, $data_type:expr) => {
        impl Serializeable for $type {
            fn data_type(&self) -> &'static [u8] {
                $data_type
            }

            fn write<O: Write>(&self, output: &mut O) -> Result<(), io::Error> {
                for x in self.iter() {
                    output.write_all(&x.0.to_ne_bytes())?;
                    output.write_all(&x.1.to_ne_bytes())?;
                }
                Ok(())
            }

            fn data_shape(&self) -> Vec<usize> {
                vec![self.len(), 2]
            }
        }
    };
}

impl_serializeable_2d_tuple!(Vec<(f32, f32)>, b"f4");
impl_serializeable_2d_tuple!(Vec<(f64, f64)>, b"f8");
impl_serializeable_2d_tuple!(Vec<(u8, u8)>, b"u1");
impl_serializeable_2d_tuple!(Vec<(u16, u16)>, b"u2");
impl_serializeable_2d_tuple!(Vec<(u32, u32)>, b"u4");
impl_serializeable_2d_tuple!(Vec<(u64, u64)>, b"u8");
impl_serializeable_2d_tuple!(Vec<(i8, i8)>, b"i1");
impl_serializeable_2d_tuple!(Vec<(i16, i16)>, b"i2");
impl_serializeable_2d_tuple!(Vec<(i32, i32)>, b"i4");
impl_serializeable_2d_tuple!(Vec<(i64, i64)>, b"i8");
