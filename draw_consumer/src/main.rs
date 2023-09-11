use Draw::color;

fn main() {
    Draw::draw_line(32, 32);

    let color = color::RGB { r: 0, g: 0, b: 0 };

    color::draw_line(32, 32, &color);

    let square = Draw::shapes::Rectangle {
        width: 32,
        height: 32,
        color: color,
    };

    println!("{:?}", square);
}
