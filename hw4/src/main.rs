fn main() {
    let mut tuple = (1, 2);
    let result_tuple = lib::get_tuple_element(&mut tuple, true);
    *result_tuple += 1;
    println!("Updated tuple: {:?}", tuple);

    let mut slice = [1, 2, 3, 4, 5];
    let result_slice = lib::get_slice_element(&mut slice, 2);
    *result_slice += 10;
    println!("Updated slice: {:?}", slice);

    let mut slice_from_end = [1, 2, 3, 4, 5];
    let result_from_end = lib::get_slice_element_from_end(&mut slice_from_end, 2);
    *result_from_end += 10;
    println!("Updated slice from end: {:?}", slice_from_end);

    let slice_split = [1, 2, 3, 4, 5];
    let (left, right) = lib::split_slice(&slice_split, 2);
    println!("Left slice: {:?}, Right slice: {:?}", left, right);

    let slice_four_split = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let res = lib::split_into_four(&slice_four_split);
    println!("Result of split_into_four: {:?}", res);
}
