use ukiyoe::Image;

fn main() {
    let mut image = Image::new("examples/test.png");
    image.render(100, 40);
}
