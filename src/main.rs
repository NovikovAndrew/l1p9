fn set_bit(num: &mut i64, bit_index: usize, value: bool) {
    if bit_index >= 64 {
        panic!("Bit index out of range. Valid range is 0 to 63.");
    }

    let bit_mask = 1 << bit_index; // Создаем маску для i-го бита

    if value {
        // Устанавливаем бит в 1
        *num |= bit_mask;
    } else {
        // Устанавливаем бит в 0
        *num &= !bit_mask;
    }
}

fn main() {
    let mut number: i64 = 0b101010; // Пример начального значения в двоичном формате

    println!("Original number: {:064b}", number); // Выводим число в бинарном формате

    let bit_index = 3;
    let new_value = true; // Установить 3-й бит в 1

    set_bit(&mut number, bit_index, new_value);

    println!("After setting bit {} to {}: {:064b}", bit_index, new_value, number);

    let bit_index = 1;
    let new_value = false; // Установить 1-й бит в 0

    set_bit(&mut number, bit_index, new_value);

    println!("After setting bit {} to {}: {:064b}", bit_index, new_value, number);
}
