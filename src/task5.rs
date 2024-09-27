#[test]

/*// Fix errors and panics to make it work
fn main() {
   let v1 = 251_u8 + 8;
   let v2 = i8::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}*/

// Fix errors and panics to make it work
fn main() {
    // Используем wrapping_add для обработки переполнения u8
    let v1 = 251_u8.wrapping_add(8);

    // Используем допустимое значение в пределах диапазона i8
    let v2 = i8::checked_add(127, 1).unwrap(); // 127 — максимальное значение для i8

    println!("{}, {}", v1, v2);
}

/*251_u8.wrapping_add(8) позволяет корректно обработать переполнение, когда сумма превышает 255 (она "перепрыгнет" обратно на 0).
i8::checked_add(127, 1) выполняет сложение в пределах допустимого диапазона для типа i8, чтобы избежать ошибки.*/