use std::{fs::File, io};

use npy::{npy, reshape};

fn main() -> io::Result<()> {
    let mut file = File::create("foo.npy")?;

    npy(
        &mut file,
        reshape(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0].as_slice(), vec![3, 2]),
    )
}
