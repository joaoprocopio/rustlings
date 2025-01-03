// Booleans (`bool`)

struct ToTestando {
    teste_1: i32,
    teste_2: i32,
}

impl ToTestando {
    fn new(teste_1: i32, teste_2: i32) -> Self {
        Self { teste_1, teste_2 }
    }
}

fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    // The value of the variable should be the negation (opposite) of `is_morning`.
    let is_evening = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}
