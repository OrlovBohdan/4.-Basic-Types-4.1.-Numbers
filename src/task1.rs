#[test]
/*// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x;

    let z = 10; // Type of z ?

    println!("Success!");
}*/
// Remove something to make it work

fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10; // Type of z ?

    println!("Success!");
}

/*В коде происходит ошибка при попытке присвоить значение переменной с типом i32 (знаковое число) переменной с типом u32 (беззнаковое число). Эти типы несовместимы напрямую.

Чтобы исправить код, вам нужно удалить тип u32 у переменной y, чтобы она соответствовала типу переменной x (i32).*/