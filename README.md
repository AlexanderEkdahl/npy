# npy

Simple library to **serialize** numerical Rust types into the [NPY format](https://numpy.org/doc/stable/reference/generated/numpy.lib.format.html).

``` rust
fn main() -> io::Result<()> {
    let mut file = File::create("3x3.npy")?;
    npy::save(&mut file, &vec![1.0, 2.0, 3.0, 4.0], &[2, 2])?;
}
```

``` python
>>> import numpy as np
>>> np.load("3x3.npy")
array([[1., 2.],
       [3., 4.]])
```
