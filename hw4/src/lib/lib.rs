// Принимает мутабельную ссылку на кортеж и bool значение.
// Если false, возвращает мутабельную ссылку на первый элемент кортежа.
// Если true, возвращает мутабельную ссылку на второй элемент кортежа.
pub fn get_tuple_element(tuple: &mut (i32, i32), is_right: bool) -> &mut i32 {
    if is_right { &mut tuple.1 } else { &mut tuple.0 }
}

// Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
pub fn get_slice_element(slice: &mut [i32], index: usize) -> &mut i32 {
    if index < slice.len() {
        &mut slice[index]
    } else {
        panic!("Index out of bounds");
    }
}

// Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
pub fn get_slice_element_from_end(slice: &mut [i32], index: usize) -> &mut i32 {
    if index < slice.len() {
        &mut slice[slice.len() - 1 - index]
    } else {
        panic!("Index out of bounds");
    }
}

// Принимает слайс и число N. Возвращает два слайса с элементами: с нулевого по N-1; с N-го по последний;
pub fn split_slice(slice: &[i32], index: usize) -> (&[i32], &[i32]) {
    if index > slice.len() {
        panic!("Index out of bounds");
    }
    (&slice[..index], &slice[index..])
}

// Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
pub fn split_into_four(slice: &[i32]) -> [&[i32]; 4] {
    let len = slice.len();
    let part_size = len / 4;
    let remainder = len % 4;
    let mut parts = [&slice[0..0]; 4];

    let mut end = 0;
    for (index, item) in parts.iter_mut().enumerate() {
        let extra = if index < remainder { 1 } else { 0 };
        let start = end;
        end = start + part_size + extra;
        *item = &slice[start..end];
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tuple_element() {
        let mut tuple = (1, 11);

        let result = get_tuple_element(&mut tuple, true);
        *result += 1;

        assert_eq!(*result, 12);
        assert_eq!(tuple.1, 12);

        let mut tuple = (1, 11);

        let result = get_tuple_element(&mut tuple, false);
        *result += 1;

        assert_eq!(*result, 2);
        assert_eq!(tuple.0, 2);
    }

    #[test]
    fn test_get_slice_element() {
        let mut slice = [1, 2, 3, 4, 5];

        let result = get_slice_element(&mut slice, 2);
        *result += 10;

        assert_eq!(*result, 13);
        assert_eq!(slice[2], 13);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_get_slice_element_panic() {
        let mut slice = [1, 2, 3, 4, 5];

        get_slice_element(&mut slice, 10);
    }

    #[test]
    fn test_get_slice_element_from_end() {
        let mut slice = [1, 2, 3, 4, 5];

        // [5, 4, 3, 2, 1]
        let result = get_slice_element_from_end(&mut slice, 2);
        *result += 10;

        assert_eq!(*result, 13);
        assert_eq!(slice[2], 13);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_get_slice_element_from_end_panic() {
        let mut slice = [1, 2, 3, 4, 5];

        get_slice_element_from_end(&mut slice, 10);
    }

    #[test]
    fn test_split_slice() {
        let slice = [1, 2, 3, 4, 5];

        let (first, second) = split_slice(&slice, 2);

        assert_eq!(first, &[1, 2]);
        assert_eq!(second, &[3, 4, 5]);

        let slice: [i32; 0] = [];

        let (first, second) = split_slice(&slice, 0);

        assert_eq!(first, &[]);
        assert_eq!(second, &[]);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_split_slice_panic() {
        let slice = [1, 2, 3, 4, 5];

        split_slice(&slice, 10);
    }

    #[test]
    fn test_split_into_four() {
        let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        let parts = split_into_four(&slice);

        assert_eq!(parts[0], &[1, 2, 3]);
        assert_eq!(parts[1], &[4, 5]);
        assert_eq!(parts[2], &[6, 7]);
        assert_eq!(parts[3], &[8, 9]);

        let slice: [i32; 0] = [];
        let parts = split_into_four(&slice);

        assert_eq!(parts[0], &[]);
        assert_eq!(parts[1], &[]);
        assert_eq!(parts[2], &[]);
        assert_eq!(parts[3], &[]);
    }
}
