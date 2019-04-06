extern crate ansi_term;
extern crate image;

use std::io::Write;

use ansi_term::Colour::Fixed;
use ansi_term::ANSIStrings;
use image::{imageops, FilterType, Pixel};

pub fn print_image<W: Write>(img: image::DynamicImage, true_colour: bool, width: u32, height: u32, w: &mut W) {
    let img = imageops::resize(&img, width, height, FilterType::Nearest);

    if !true_colour {
        for y in 0..height {
            //TODO: inc by 2 instead
            if y%2 == 1 || y + 1 == height {
                continue;
            }

            let row: Vec<_> = (0..width).map(|x| {
                let mut top = img[(x,y)];
                let mut bottom = img[(x,y+1)];
                blend_alpha(&mut top);
                blend_alpha(&mut bottom);
                let top_colour = find_colour_index(top.to_rgb().channels());
                let bottom_colour = find_colour_index(bottom.to_rgb().channels());
                Fixed(bottom_colour).on(Fixed(top_colour)).paint("▄")
            }).collect();

            write!(w, "{}\n", ANSIStrings(&row)).ok();
        }    
    } else {
        let mut row = Vec::new();
        for y in 0..height {
            //TODO: inc by 2 instead
            if y%2 == 1 || y + 1 == height {
                continue;
            }

            for x in 0..width {
                let mut top = img[(x,y)];
                let mut bottom = img[(x,y+1)];
                blend_alpha(&mut top);
                blend_alpha(&mut bottom);
                write!(row, "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄",
                       top[0], top[1], top[2],
                       bottom[0], bottom[1], bottom[2]).unwrap();
            }

            write!(row, "\x1b[m\n").unwrap();
            w.write(&row).unwrap();
            row.clear();
        }
    }
}

fn find_colour_index(pixel: &[u8]) -> u8 {
    let mut best = 0;
    let mut best_distance = 255 * 255 * 3 + 1;
    for i in 16..255 {
        let ansi_colour = ANSI_COLOURS[i];
        let dr = ansi_colour[0] - pixel[0] as i32;
        let dg = ansi_colour[1] - pixel[1] as i32;
        let db = ansi_colour[2] - pixel[2] as i32;
        let distance = dr * dr + dg * dg + db * db;

        if distance < best_distance {
            best_distance = distance;
            best = i as u8;
        }
    }

    return best;
}

fn blend_alpha(pixel: &mut image::Rgba<u8>) {
    let alpha = pixel[3] as i32 as f32/255.0;
    pixel[0] = (alpha*(pixel[0] as i32 as f32) + (1.0 - alpha)*38.0) as u8;
    pixel[1] = (alpha*(pixel[1] as i32 as f32) + (1.0 - alpha)*38.0) as u8;
    pixel[2] = (alpha*(pixel[2] as i32 as f32) + (1.0 - alpha)*38.0) as u8;
}

static ANSI_COLOURS: [[i32; 3]; 256] = [
[ 0x00, 0x00, 0x00 ],[ 0x80, 0x00, 0x00 ],[ 0x00, 0x80, 0x00 ],[ 0x80, 0x80, 0x00 ],[ 0x00, 0x00, 0x80 ],
[ 0x80, 0x00, 0x80 ],[ 0x00, 0x80, 0x80 ],[ 0xc0, 0xc0, 0xc0 ],[ 0x80, 0x80, 0x80 ],[ 0xff, 0x00, 0x00 ],
[ 0x00, 0xff, 0x00 ],[ 0xff, 0xff, 0x00 ],[ 0x00, 0x00, 0xff ],[ 0xff, 0x00, 0xff ],[ 0x00, 0xff, 0xff ],
[ 0xff, 0xff, 0xff ],[ 0x00, 0x00, 0x00 ],[ 0x00, 0x00, 0x5f ],[ 0x00, 0x00, 0x87 ],[ 0x00, 0x00, 0xaf ],
[ 0x00, 0x00, 0xd7 ],[ 0x00, 0x00, 0xff ],[ 0x00, 0x5f, 0x00 ],[ 0x00, 0x5f, 0x5f ],[ 0x00, 0x5f, 0x87 ],
[ 0x00, 0x5f, 0xaf ],[ 0x00, 0x5f, 0xd7 ],[ 0x00, 0x5f, 0xff ],[ 0x00, 0x87, 0x00 ],[ 0x00, 0x87, 0x5f ],
[ 0x00, 0x87, 0x87 ],[ 0x00, 0x87, 0xaf ],[ 0x00, 0x87, 0xd7 ],[ 0x00, 0x87, 0xff ],[ 0x00, 0xaf, 0x00 ],
[ 0x00, 0xaf, 0x5f ],[ 0x00, 0xaf, 0x87 ],[ 0x00, 0xaf, 0xaf ],[ 0x00, 0xaf, 0xd7 ],[ 0x00, 0xaf, 0xff ],
[ 0x00, 0xd7, 0x00 ],[ 0x00, 0xd7, 0x5f ],[ 0x00, 0xd7, 0x87 ],[ 0x00, 0xd7, 0xaf ],[ 0x00, 0xd7, 0xd7 ],
[ 0x00, 0xd7, 0xff ],[ 0x00, 0xff, 0x00 ],[ 0x00, 0xff, 0x5f ],[ 0x00, 0xff, 0x87 ],[ 0x00, 0xff, 0xaf ],
[ 0x00, 0xff, 0xd7 ],[ 0x00, 0xff, 0xff ],[ 0x5f, 0x00, 0x00 ],[ 0x5f, 0x00, 0x5f ],[ 0x5f, 0x00, 0x87 ],
[ 0x5f, 0x00, 0xaf ],[ 0x5f, 0x00, 0xd7 ],[ 0x5f, 0x00, 0xff ],[ 0x5f, 0x5f, 0x00 ],[ 0x5f, 0x5f, 0x5f ],
[ 0x5f, 0x5f, 0x87 ],[ 0x5f, 0x5f, 0xaf ],[ 0x5f, 0x5f, 0xd7 ],[ 0x5f, 0x5f, 0xff ],[ 0x5f, 0x87, 0x00 ],
[ 0x5f, 0x87, 0x5f ],[ 0x5f, 0x87, 0x87 ],[ 0x5f, 0x87, 0xaf ],[ 0x5f, 0x87, 0xd7 ],[ 0x5f, 0x87, 0xff ],
[ 0x5f, 0xaf, 0x00 ],[ 0x5f, 0xaf, 0x5f ],[ 0x5f, 0xaf, 0x87 ],[ 0x5f, 0xaf, 0xaf ],[ 0x5f, 0xaf, 0xd7 ],
[ 0x5f, 0xaf, 0xff ],[ 0x5f, 0xd7, 0x00 ],[ 0x5f, 0xd7, 0x5f ],[ 0x5f, 0xd7, 0x87 ],[ 0x5f, 0xd7, 0xaf ],
[ 0x5f, 0xd7, 0xd7 ],[ 0x5f, 0xd7, 0xff ],[ 0x5f, 0xff, 0x00 ],[ 0x5f, 0xff, 0x5f ],[ 0x5f, 0xff, 0x87 ],
[ 0x5f, 0xff, 0xaf ],[ 0x5f, 0xff, 0xd7 ],[ 0x5f, 0xff, 0xff ],[ 0x87, 0x00, 0x00 ],[ 0x87, 0x00, 0x5f ],
[ 0x87, 0x00, 0x87 ],[ 0x87, 0x00, 0xaf ],[ 0x87, 0x00, 0xd7 ],[ 0x87, 0x00, 0xff ],[ 0x87, 0x5f, 0x00 ],
[ 0x87, 0x5f, 0x5f ],[ 0x87, 0x5f, 0x87 ],[ 0x87, 0x5f, 0xaf ],[ 0x87, 0x5f, 0xd7 ],[ 0x87, 0x5f, 0xff ],
[ 0x87, 0x87, 0x00 ],[ 0x87, 0x87, 0x5f ],[ 0x87, 0x87, 0x87 ],[ 0x87, 0x87, 0xaf ],[ 0x87, 0x87, 0xd7 ],
[ 0x87, 0x87, 0xff ],[ 0x87, 0xaf, 0x00 ],[ 0x87, 0xaf, 0x5f ],[ 0x87, 0xaf, 0x87 ],[ 0x87, 0xaf, 0xaf ],
[ 0x87, 0xaf, 0xd7 ],[ 0x87, 0xaf, 0xff ],[ 0x87, 0xd7, 0x00 ],[ 0x87, 0xd7, 0x5f ],[ 0x87, 0xd7, 0x87 ],
[ 0x87, 0xd7, 0xaf ],[ 0x87, 0xd7, 0xd7 ],[ 0x87, 0xd7, 0xff ],[ 0x87, 0xff, 0x00 ],[ 0x87, 0xff, 0x5f ],
[ 0x87, 0xff, 0x87 ],[ 0x87, 0xff, 0xaf ],[ 0x87, 0xff, 0xd7 ],[ 0x87, 0xff, 0xff ],[ 0xaf, 0x00, 0x00 ],
[ 0xaf, 0x00, 0x5f ],[ 0xaf, 0x00, 0x87 ],[ 0xaf, 0x00, 0xaf ],[ 0xaf, 0x00, 0xd7 ],[ 0xaf, 0x00, 0xff ],
[ 0xaf, 0x5f, 0x00 ],[ 0xaf, 0x5f, 0x5f ],[ 0xaf, 0x5f, 0x87 ],[ 0xaf, 0x5f, 0xaf ],[ 0xaf, 0x5f, 0xd7 ],
[ 0xaf, 0x5f, 0xff ],[ 0xaf, 0x87, 0x00 ],[ 0xaf, 0x87, 0x5f ],[ 0xaf, 0x87, 0x87 ],[ 0xaf, 0x87, 0xaf ],
[ 0xaf, 0x87, 0xd7 ],[ 0xaf, 0x87, 0xff ],[ 0xaf, 0xaf, 0x00 ],[ 0xaf, 0xaf, 0x5f ],[ 0xaf, 0xaf, 0x87 ],
[ 0xaf, 0xaf, 0xaf ],[ 0xaf, 0xaf, 0xd7 ],[ 0xaf, 0xaf, 0xff ],[ 0xaf, 0xd7, 0x00 ],[ 0xaf, 0xd7, 0x5f ],
[ 0xaf, 0xd7, 0x87 ],[ 0xaf, 0xd7, 0xaf ],[ 0xaf, 0xd7, 0xd7 ],[ 0xaf, 0xd7, 0xff ],[ 0xaf, 0xff, 0x00 ],
[ 0xaf, 0xff, 0x5f ],[ 0xaf, 0xff, 0x87 ],[ 0xaf, 0xff, 0xaf ],[ 0xaf, 0xff, 0xd7 ],[ 0xaf, 0xff, 0xff ],
[ 0xd7, 0x00, 0x00 ],[ 0xd7, 0x00, 0x5f ],[ 0xd7, 0x00, 0x87 ],[ 0xd7, 0x00, 0xaf ],[ 0xd7, 0x00, 0xd7 ],
[ 0xd7, 0x00, 0xff ],[ 0xd7, 0x5f, 0x00 ],[ 0xd7, 0x5f, 0x5f ],[ 0xd7, 0x5f, 0x87 ],[ 0xd7, 0x5f, 0xaf ],
[ 0xd7, 0x5f, 0xd7 ],[ 0xd7, 0x5f, 0xff ],[ 0xd7, 0x87, 0x00 ],[ 0xd7, 0x87, 0x5f ],[ 0xd7, 0x87, 0x87 ],
[ 0xd7, 0x87, 0xaf ],[ 0xd7, 0x87, 0xd7 ],[ 0xd7, 0x87, 0xff ],[ 0xd7, 0xaf, 0x00 ],[ 0xd7, 0xaf, 0x5f ],
[ 0xd7, 0xaf, 0x87 ],[ 0xd7, 0xaf, 0xaf ],[ 0xd7, 0xaf, 0xd7 ],[ 0xd7, 0xaf, 0xff ],[ 0xd7, 0xd7, 0x00 ],
[ 0xd7, 0xd7, 0x5f ],[ 0xd7, 0xd7, 0x87 ],[ 0xd7, 0xd7, 0xaf ],[ 0xd7, 0xd7, 0xd7 ],[ 0xd7, 0xd7, 0xff ],
[ 0xd7, 0xff, 0x00 ],[ 0xd7, 0xff, 0x5f ],[ 0xd7, 0xff, 0x87 ],[ 0xd7, 0xff, 0xaf ],[ 0xd7, 0xff, 0xd7 ],
[ 0xd7, 0xff, 0xff ],[ 0xff, 0x00, 0x00 ],[ 0xff, 0x00, 0x5f ],[ 0xff, 0x00, 0x87 ],[ 0xff, 0x00, 0xaf ],
[ 0xff, 0x00, 0xd7 ],[ 0xff, 0x00, 0xff ],[ 0xff, 0x5f, 0x00 ],[ 0xff, 0x5f, 0x5f ],[ 0xff, 0x5f, 0x87 ],
[ 0xff, 0x5f, 0xaf ],[ 0xff, 0x5f, 0xd7 ],[ 0xff, 0x5f, 0xff ],[ 0xff, 0x87, 0x00 ],[ 0xff, 0x87, 0x5f ],
[ 0xff, 0x87, 0x87 ],[ 0xff, 0x87, 0xaf ],[ 0xff, 0x87, 0xd7 ],[ 0xff, 0x87, 0xff ],[ 0xff, 0xaf, 0x00 ],
[ 0xff, 0xaf, 0x5f ],[ 0xff, 0xaf, 0x87 ],[ 0xff, 0xaf, 0xaf ],[ 0xff, 0xaf, 0xd7 ],[ 0xff, 0xaf, 0xff ],
[ 0xff, 0xd7, 0x00 ],[ 0xff, 0xd7, 0x5f ],[ 0xff, 0xd7, 0x87 ],[ 0xff, 0xd7, 0xaf ],[ 0xff, 0xd7, 0xd7 ],
[ 0xff, 0xd7, 0xff ],[ 0xff, 0xff, 0x00 ],[ 0xff, 0xff, 0x5f ],[ 0xff, 0xff, 0x87 ],[ 0xff, 0xff, 0xaf ],
[ 0xff, 0xff, 0xd7 ],[ 0xff, 0xff, 0xff ],[ 0x08, 0x08, 0x08 ],[ 0x12, 0x12, 0x12 ],[ 0x1c, 0x1c, 0x1c ],
[ 0x26, 0x26, 0x26 ],[ 0x30, 0x30, 0x30 ],[ 0x3a, 0x3a, 0x3a ],[ 0x44, 0x44, 0x44 ],[ 0x4e, 0x4e, 0x4e ],
[ 0x58, 0x58, 0x58 ],[ 0x60, 0x60, 0x60 ],[ 0x66, 0x66, 0x66 ],[ 0x76, 0x76, 0x76 ],[ 0x80, 0x80, 0x80 ],
[ 0x8a, 0x8a, 0x8a ],[ 0x94, 0x94, 0x94 ],[ 0x9e, 0x9e, 0x9e ],[ 0xa8, 0xa8, 0xa8 ],[ 0xb2, 0xb2, 0xb2 ],
[ 0xbc, 0xbc, 0xbc ],[ 0xc6, 0xc6, 0xc6 ],[ 0xd0, 0xd0, 0xd0 ],[ 0xda, 0xda, 0xda ],[ 0xe4, 0xe4, 0xe4 ],
[ 0xee, 0xee, 0xee ]];


