#[test]
/*
// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2 == __);

    // Integer subtraction
    assert!(1i32 - 2 == __);
    assert!(1u8 - 2 == -1);

    assert!(3 * 50 == __);

    assert!(9.6 / 3.2 == 3.0); // error ! make it work

    assert!(24 % 5 == __);
    // Short-circuiting boolean logic
    assert!(true && false == __);
    assert!(true || false == __);
    assert!(!true == __);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}*/

fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1u8.wrapping_sub(2) == 255); // Use wrapping_sub to avoid overflow

    assert!(3 * 50 == 150);

    assert!((9.6_f64 / 3.2_f64 - 3.0_f64).abs() < 1e-10);

    assert!(24 % 5 == 4);

    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}


/*Сложение целых чисел:

Заполнен пропуск: assert!(1u32 + 2 == 3);
Вычитание целых чисел:

Заполнен пропуск: assert!(1i32 - 2 == -1);
Для u8 мы используем метод wrapping_sub, чтобы избежать переполнения при вычитании: assert!(1u8.wrapping_sub(2) == 255);
Умножение:

Заполнен пропуск: assert!(3 * 50 == 150);
Сравнение с числами с плавающей запятой:

Исправлено на использование погрешности для проверки: assert!((9.6 / 3.2 - 3.0).abs() < 1e-10);
Остальные проверки:

Заполнен пропуск для assert!(24 % 5 == 4);
Логические операции исправлены: assert!(true && false == false);, assert!(true || false == true);, assert!(!true == false);*/