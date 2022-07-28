use ansi_escapes::{CursorMove, CursorTo};
use colored::Colorize;
use image::DynamicImage;
use image::GenericImageView;
use image::Rgba;

pub struct Image {
    pub path: String,
    image: Option<DynamicImage>,
    cached_image: Option<(u16, u16, Vec<String>)>,
}

impl Image {
    pub fn new(path: &str) -> Image {
        Image {
            path: path.to_owned(),
            image: None,
            cached_image: None,
        }
    }

    fn generate_cached_image(
        &self,
        width: u16,
        height: u16,
        image: &DynamicImage,
    ) -> (u16, u16, Vec<String>) {
        let pixels: Vec<_> = image.pixels().collect();
        let (image_width, image_height) = image.dimensions();

        let mut lines = Vec::new();

        for y in 0..height {
            let mut line = String::new();
            for x in 0..width {
                let r_x = (x as f32 / width as f32 * image_width as f32) as u32;
                let r_y = (y as f32 / height as f32 * image_height as f32) as u32;
                let pixel = pixels[(r_y * image_width + r_x) as usize];
                let color: Rgba<u8> = pixel.2;
                let data = color.0;

                let r = data[0];
                let g = data[1];
                let b = data[2];

                line.push_str(&format!("{}", "▒".truecolor(r, g, b)));
            }
            lines.push(line);
        }

        (width, height, lines)
    }

    fn lazy_load(&mut self, width: u16, height: u16) {
        if let None = self.image {
            self.image = Some(image::open(&self.path).unwrap());
        }
        if let Some(im) = &self.image {
            if let None = self.cached_image {
                self.cached_image = Some(self.generate_cached_image(width, height, im));
            }
            if let Some(c) = self.cached_image.as_ref() {
                if c.0 != width && c.1 != height {
                    self.cached_image = Some(self.generate_cached_image(width, height, im));
                }
            }
        } else {
            panic!("No image");
        }
    }

    pub fn render_at_position(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.lazy_load(width, height);

        let image = self.cached_image.as_ref().expect("No cached image");
        print!("{}", CursorTo::AbsoluteXY(x, y));
        let char_rows = &image.2;
        for char_row in char_rows {
            print!("{}{}", char_row, CursorMove::XY(-(width as i16), 1))
        }
    }

    pub fn render(&mut self, width: u16, height: u16) {
        self.lazy_load(width, height);
        let image = self.cached_image.as_ref().expect("No cached image");
        let char_rows = &image.2;
        for char_row in char_rows {
            println!("{}", char_row);
        }
    }
}
