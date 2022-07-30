use ukiyoe::Image;

fn main() {
    Image::render_pixels(
        20,
        20,
        &[
            [255, 0, 0, 255],
            [0, 255, 0, 255],
            [0, 0, 255, 255],
            [0, 0, 255, 255],
        ],
        2,
    );
}
