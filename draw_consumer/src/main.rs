use features::color;

fn main() {
    features::draw_line(32, 32);

    let color = color::RGB {
        r: 255,
        g: 255,
        b: 255
    };

    color::draw_line(32, 32, &color);

    let square = features::shapes::Rectangle {
        width: 32,
        height: 32,
        color,
    };

    println!("{square:?}");
}
