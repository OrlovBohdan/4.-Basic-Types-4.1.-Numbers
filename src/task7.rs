#[test]
/*
// Fill the blank to make it work
fn main() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "__".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}*/


// Fill the blank to make it work
fn main() {
    let x = 1_000.000_1; // f64 по умолчанию
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

/*В данном коде нужно заполнить строку в функции assert_eq! правильным типом для переменной x.
Посмотрим на её значение:
let x = 1_000.000_1;
По умолчанию литералы с плавающей точкой в Rust интерпретируются как тип f64, если явно не указано,
что это f32. Следовательно, тип переменной x будет f64.

Чтобы исправить код, нужно вставить f64 в строку в assert_eq!:*/