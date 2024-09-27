use std::ops::{Range, RangeInclusive};
#[test]


/*// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..__), Range{ start: 1, end: 5 });
    assert_eq!((1..__), RangeInclusive::new(1, 5));

    println!("Success!");
}*/



fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });  // Для Range диапазон от 1 до 4, 5 не включается
    assert_eq!((1..=5), RangeInclusive::new(1, 5));  // Для RangeInclusive диапазон от 1 до 5, включая 5

    println!("Success!");
}

/*1..5: Создает диапазон от 1 до 4 (5 не включается).
1..=5: Создает включающий диапазон от 1 до 5 (5 включается).*/