// Вам нужно реализовать программу обработки команд для дисплея.
// На вход пользователь подает:
// * 2 числа: размер дисплея
// * 1 число: цвет дисплея по-умолчанию (1 - красный, 2 - зеленый, 3 - синий)
// * Последовательность команд: набор чисел.
//
// Дисплей поддерживает следующие команды:
// * 1 x y - переместить курсор в позицию x y
// * 2 colour - перекрасить пиксель в цвет colour
//
// Пример входных данных:
// 4 4
// 1
// 1 2 2 2 3
// В результате пиксель по позиции (2,2) будет перекрашен в синий цвет

// Обновлять состояние дисплея нужно через метод matrix.set_colour(pos_x, pos_y, colour)

// Важно! Обязательна проверка на ошибки. Если пользователь просит переместиться на пиксель за пределами дисплея или ввел неправильный цвет, то вам нужно кинуть панику!

use std::io;
mod matrix;
use matrix::Matrix;

struct Display {
    // можете добавить сюда любые дополнительные поля
    matrix: Matrix,
    width: u64,
    height: u64,
}

fn create_display(max_width: u32, max_height: u32, default_colour: u8) -> Display {
    // ваш код сюда
    Display {
        matrix: Matrix::new(max_width, max_height, default_colour),
        width: max_width as u64,
        height: max_height as u64,
    }
}

fn process_commands(display: &mut Display, input: Vec<u64>) {
    // ваш код сюда
    let mut i = 0;
    let mut pos_x: Option<u64> = None;
    let mut pos_y: Option<u64> = None;
    while i < input.len() {
        match input[i] {
            1 => {
                // Команда перемещения курсора
                if i + 2 >= input.len() {
                    panic!("Недостаточно аргументов для команды перемещения");
                }
                let x = input[i + 1];
                let y = input[i + 2];
                if x >= display.width || y >= display.height {
                    panic!("Позиция вне дисплея: ({},{})", x, y);
                }
                pos_x = Some(x);
                pos_y = Some(y);
                i += 3;
            }
            2 => {
                // Команда перекраски пикселя
                if i + 1 >= input.len() {
                    panic!("Недостаточно аргументов для команды перекраски");
                }
                let colour = input[i + 1] as u8;
                if !(1..=3).contains(&colour) {
                    panic!("Недопустимый цвет: {}", colour);
                }
                let (x, y) = match (pos_x, pos_y) {
                    (Some(a), Some(b)) => (a, b),
                    _ => panic!("Курсор не установлен перед перекраской"),
                };
                display.matrix.set_colour(x, y, colour);
                i += 2;
            }
            _ => panic!("Неизвестная команда: {}", input[i]),
        }
    }
}

// код ниже трогать не нужно, можете просто посмотреть его

// тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_case() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 3]);
        let mut expected = Matrix::new(4, 4, 1);
        expected.set_colour(2, 2, 3);
        assert_eq!(display.matrix, expected);
    }

    #[test]
    #[should_panic]
    fn test_error() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 5, 5, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_error_invalid_colour() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 5]);
    }
}

fn main() {
    println!("Введите размеры дисплея (ширина высота):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (width, height) = parse_dimensions(&input);

    println!("Введите стандартный цвет дисплея (1 - красный, 2 - зеленый, 3 - синий):");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let default_colour = match input.trim() {
        "1" => 1, // Красный
        "2" => 2, // Зеленый
        "3" => 3, // Синий
        _ => panic!("Неверный ввод цвета. Ожидалось 1, 2 или 3."),
    };

    // Создаём дисплей и заполняем его стандартным цветом
    let mut display = create_display(width, height, default_colour);

    // Ввод действий
    println!("Введите строку с действиями:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let commands = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // Отображение дисплея
    process_commands(&mut display, commands);

    display.matrix.display();
}

fn parse_dimensions(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Неверный ввод размера"))
        .collect();
    if parts.len() != 2 {
        panic!("Ожидалось два числа для размеров дисплея.");
    }
    (parts[0], parts[1])
}
