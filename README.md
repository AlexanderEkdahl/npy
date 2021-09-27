# npy

Simple library to **serialize** numerical Rust types into the [NPY format](https://numpy.org/doc/stable/reference/generated/numpy.lib.format.html).

``` rust
fn main() {
    let mut file = File::create("foo.npy").unwrap();

    npy(
        &mut file,
        reshape(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0].as_slice(), vec![3, 2]),
    )
    .unwrap();
}
```

This library automatically infers the shape of single, 1-dimensional array, and 2-dimensional arrays. The `reshape` function is used to explicitly set the shape of the output N-dimensional array.

Pull dependency directly from GitHub:

``` toml
[dependencies]
npy = { git = "https://github.com/AlexanderEkdahl/npy" }
```
