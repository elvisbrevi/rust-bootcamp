trait UIComponent {
    fn render(&self) {
        println!("Rendering UIComponent");
    }
}

struct Button {
    text: String,
}

// case 2: recursive property
struct Container {
    name: String,
    child: Box<Container>,
}

impl UIComponent for Button {}
impl UIComponent for Container {}

fn main() {
    // stored in stack
    let button_a = Button {
        text: "Click me".to_owned(),
    };

    // stored in heap
    let button_b = Box::new(Button {
        text: "Click me".to_owned(),
    });

    let button_c = button_a;
    let button_d = button_b;

    // case 1: specify type
    let components: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}
