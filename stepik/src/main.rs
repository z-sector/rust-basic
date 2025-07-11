// В данной задаче вам предстоит найти и исправить
// логическую ошибку в функции find_second_max,
// которая ищет второе максимальное значение
// в переданной коллекции.
//
// - Изучите предоставленный код
// - Запустите тесты и проанализируйте результаты
// - Найдите логическую ошибку в функции
// - Исправьте ошибку так, чтобы все тесты проходили успешно

pub fn find_second_max(numbers: &[i32]) -> Option<i32> {
    let mut max = None;
    let mut second = None;
    for &n in numbers {
        if Some(n) > max {
            second = max;
            max = Some(n);
        } else if Some(n) < max && Some(n) > second {
            second = Some(n);
        }
    }
    second
}


fn main() {
    // Read the total number of questions
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read total questions");
    let total_questions: u32 = input.trim().parse().expect("Invalid number");

    // Read the number of questions not studied
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read not studied questions");
    let not_studied: u32 = input.trim().parse().expect("Invalid number");

    // Calculate the probability of getting a studied question
    // Probability = (total questions - not studied questions) / total questions
    let probability = (total_questions - not_studied) as f64 / total_questions as f64;

    // Output the result with 3 decimal places
    println!("Вероятность попадания выученного вопроса: {:.3}", probability);
}
