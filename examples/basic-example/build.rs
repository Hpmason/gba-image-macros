use gba_image_macros::prelude::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");

    let consts = ConstWriter::from_path(&Path::new("src/assets.rs")).unwrap();
    let mut consts = consts.finish_dependencies();
    
    load_sprite!(consts, "diamond", "DIAMOND");
}