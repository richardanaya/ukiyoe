use ukiyoe::Image;

fn main() {
    let mut image = Image::new("examples/test.png");
    image.render_at_position(0, 0, 100, 40);
}
