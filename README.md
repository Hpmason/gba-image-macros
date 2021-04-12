# gba-image-macros
This crate is for macros that convert a `.bin` file to an array of a specific byte size (u8, u16, u32) at compile time. This avoids needing convert from bytes to the specified datatype at runtime when only using `include_bytes!()`.

These macros are not in a very useful state and will be changed and added to. This is mostly for personal use.

## Macros
### load_sprite!
`load_sprite!(writer, file_name, var_name)` takes a writer (of type `ConstWriter` from the build_consts crate), a file name (the name of the `pal.bin` and `img.bin` files in the assets folder), and a var_name (the output const name $var_name`_PAL` and $var_name`_IMG`)

### load_data!
`load_data!(writer, file, name, arr_type)` takes a writer (of type `ConstWriter` from the build_consts crate), a full file name path, a name for the resulting constant, and a type for the array (u8, u16, u32). 

## Usage
These macros are meant to be used in a `build.rs` file and with this file structure:
```
.
|-- assets/
|   |-- image.bmp
|   |-- image.pal.bin
|   |-- image.img.bin
|-- src/
|   |-- main.rs
|   `-- assets.rs
`-- build.rs
```

```rust
// build.rs
use gba_image_macros::prelude::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");
    // Create a ConstWriter for the macros to use
    let consts = ConstWriter::from_path(&Path::new("src/assets.rs")).unwrap();
    let mut consts = consts.finish_dependencies();
    // add a load_sprite macro for each sprite
    load_sprite!(consts, "image", "IMAGE");
}
```
```rust
// resulting assets.rs
#[allow(dead_code)]
pub const IMAGE_PAL: [u16; 5] = [
    ...
];
#[allow(dead_code)]
pub const IMAGE_IMG: [u32; 32] = [
    ...
];
```