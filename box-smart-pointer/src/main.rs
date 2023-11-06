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

    // example 2
    let stack_var = 5;
    let heap_var = Box::new(4);
    let res = stack_var + *heap_var;
    assert_eq!(res, 9);

    // example 3
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(1, Box::new(List::Nil))))
}
