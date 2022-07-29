# ukiyoe

<a href="https://docs.rs/ukiyoe"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

Render images to the terminal using truecolor and unicode.


<p align="center"><img width="754" alt="Screen Shot 2022-07-29 at 5 30 53 AM" src="https://user-images.githubusercontent.com/294042/181758877-732028d1-d1eb-442f-a595-ed4fa21b815c.png"></p>

This package was meant to be used by it's sister package [kimono](https://github.com/richardanaya/kimono).

```terminal
cargo add ukiyoe
```

# Examples

*Render to terminal with width and height.*

```rust
use ukiyoe::Image;

fn main() {
    let mut image = Image::new("examples/test.png");
    image.render(100, 40);
}
```

*Render at a specific location.*

```rust
use ukiyoe::Image;

fn main() {
    let mut image = Image::new("examples/test.png");
    image.render_at_position(0, 0, 100, 40);
}
```

# Art

![ukiyoe_portland](https://user-images.githubusercontent.com/294042/181436102-fdef0292-2170-4b5f-9779-de3c4a22ce4c.png)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `ukiyoe` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
