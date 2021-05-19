use std::io;
use std::io::Write;
use std::convert::TryFrom;

mod serializeable;

pub use serializeable::{Reshaped, reshape};

pub trait Serializeable {
    fn data_type(&self) -> &'static [u8];
    fn write<O: Write>(&self, output: &mut O) -> Result<(), io::Error>;
    fn data_shape(&self) -> Vec<usize>;
}

pub fn npy<T: Serializeable, O: Write>(
    output: &mut O,
    data: T,
) -> Result<(), io::Error> {
    let header = build_header(&data.data_shape(), data.data_type());
    let header_len = u16::try_from(header.len()).expect("header exceeds max length");

    output.write_all(b"\x93NUMPY\x01\x00")?;
    output.write_all(&header_len.to_le_bytes())?;
    output.write_all(&header)?;

    data.write(output)
}

fn build_header(shape: &[usize], data_type: &'static [u8]) -> Vec<u8> {
    let mut header = Vec::new();
    header.extend_from_slice(b"{'descr': '");
    if cfg!(target_endian = "big") {
        header.extend_from_slice(b">")
    } else {
        header.extend_from_slice(b"<")
    }
    header.extend_from_slice(data_type);
    header.extend_from_slice(b"', 'fortran_order': False, 'shape': (");
    for dim in shape.iter() {
        header.extend_from_slice(dim.to_string().as_bytes());
        header.extend_from_slice(b",");
    }
    header.extend_from_slice(b"), }");
    header
}
