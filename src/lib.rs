pub mod prelude;

/// load_sprite!(writer, file_name, var_name)
#[macro_export]
macro_rules! load_sprite {
    ($writer:ident, $file_name:literal, $name:literal) => {
        {
            $writer.add_raw("#[allow(dead_code)]");
            load_data!($writer, concat!("assets/", $file_name, ".pal.bin"), concat!($name, "_PAL"), u16);
            $writer.add_raw("#[allow(dead_code)]");
            load_data!($writer, concat!("assets/", $file_name, ".img.bin"), concat!($name, "_IMG"), u32);
        }
    }
}
/// load_data!(writer, file, name, arr_type)
#[macro_export]
macro_rules! load_data {
    ($writer:ident, $file:expr, $name:expr, $byte_type:ty) => {
        {
            const BYTES: &[u8] = include_bytes!($file);
            let mut current = &BYTES[..];
            const size: usize = size_of::<$byte_type>();
            let mut buf: [$byte_type; BYTES.len()/size]= [0; BYTES.len()/size];
            
            let mut i = 0;
            while let Ok(uint) = current.read_uint::<LittleEndian>(size) {
                buf[i] = uint as $byte_type;
                i += 1;
            }
            let raw = buf.iter().map(|b| 
                match size {
                    1 => format!("{:#04X?}", b),
                    2 => format!("{:#06X?}", b),
                    4 => format!("{:#010X?}", b),
                    _ => format!("{:#010X?}", b),
                }
            ).collect::<Vec<String>>();
            $writer.add_array_raw($name, stringify!($byte_type), &raw);
        }
    };
}

/// load_data!(writer, load_as, file, var_name)
#[macro_export]
macro_rules! load_pal {
    ($writer:ident, $load_as:ident, $file:literal, $name:literal) => {
        {
            const BYTES: &[u8] = include_bytes!($file);
            let mut current = &BYTES[..];
            let mut buf: [u16; BYTES.len()/2] = [0; BYTES.len()/2];

            current.read_u16_into::<LittleEndian>(&mut buf).unwrap();
            $writer.add_array($name, "u16", &buf);
        }
    }
}
