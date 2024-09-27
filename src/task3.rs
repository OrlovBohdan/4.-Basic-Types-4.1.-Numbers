#[test]

/*
// Modify `assert_eq!` to make it work
fn main() {
    let x = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}*/


fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));  // Исправлено на "i32"

    println!("Success!");
}

// Получить тип переменной и вернуть его строковое представление, например, "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


/*Функция type_of возвращает строковое представление типа переменной.
В данном случае, переменная x имеет тип i32 по умолчанию, так как это целое число без явного указания типа.
Следовательно, выражение type_of(&x) вернёт строку "i32", а не "u32", как указано в assert_eq!.
Чтобы исправить код, нужно изменить тип ожидаемого значения в assert_eq! на "i32".*/