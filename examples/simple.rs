use std::{fs::File, io};

fn main() -> io::Result<()> {
    let mut file = File::create("1x3.npy")?;
    npy::save(&mut file, &vec![1, 2, 3], &[1, 3])?;

    let mut file = File::create("singleton.npy")?;
    npy::save(&mut file, &vec![42u64], &[])?;

    let mut file = File::create("3x3.npy")?;
    npy::save(&mut file, &vec![1.0, 2.0, 3.0, 4.0], &[2, 2])?;

    let mut file = File::create("2x2.npy")?;
    let x = vec![1i8, 2, 3, 4];
    npy::save(&mut file, &x, &[2, 2])?;

    Ok(())
}
