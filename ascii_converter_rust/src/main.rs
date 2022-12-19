use std::fs::File;
use std::io::{BufWriter, Write};
use image::{GenericImageView};

fn get_str_ascii(intent :u8) -> &'static str {
    let index = intent / 32;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
}

fn get_image(dir: &str, scale: u32, output_file: &str) {
    let img = image::open(dir).unwrap();
    println!("{:?}", img.dimensions());
    let (width, height) = img.dimensions();

    let file = File::create(output_file).unwrap();
    let mut writer = BufWriter::new(file);

    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let mut intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                if pix[3] == 0 {
                    intent = 0;
                }
                writer.write(get_str_ascii(intent).as_bytes()).unwrap();
                print!("{}",get_str_ascii(intent));
            }
        }
        if y % (scale * 2) == 0 {
            writer.write(b"\n").unwrap();
            println!("");
        }
    }
}

fn main() {
    get_image("pug.png", 4, "ascii_art.txt");
}
