trait UIComponent {
    fn render(&self) {
        println!("Rendering UIComponent");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

fn main() {
    // stored in stack
    let button_a = Button {
        text: "Click me".to_string(),
    };

    // stored in heap
    let button_b = Box::new(Button {
        text: "Click me".to_string(),
    }

    button_a.render();
}
