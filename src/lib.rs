use ansi_escapes::CursorTo;
use image::DynamicImage;

pub struct Image {
    pub path: String,
    image: Option<DynamicImage>,
    cached_image: Option<(u16, u16, Vec<char>)>,
}

impl Image {
    pub const fn new(path: String) -> Image {
        Image {
            path: path,
            image: None,
            cached_image: None,
        }
    }

    pub fn render_at_position(&mut self, x: u16, y: u16, width: u16, height: u16) {
        if let None = self.image {
            self.image = Some(image::open(&self.path).unwrap());
        }
        if let Some(_image) = &self.image {
            if let None = self.cached_image {
                self.cached_image = Some((width, height, vec![]));
            }
            if let Some(c) = self.cached_image.as_ref() {
                if c.0 != width && c.1 != height {
                    self.cached_image = Some((width, height, vec![]));
                }
            }
        } else {
            panic!("No image");
        }

        let image = self.cached_image.as_ref().expect("No cached image");
        print!("{}", CursorTo::AbsoluteXY(x, y));
        let characters = &image.2;
        let rows = characters.len() as u16 / width;
        for row in 0..rows {
            for col in 0..width {
                let index = row * width + col;
                let character = characters[index as usize];
                print!("{}", character);
            }
            println!("");
        }
    }
}
