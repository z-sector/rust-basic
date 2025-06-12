fn main() {
    println!("double_int32(5) = {}", double_int32(5));
    println!("double_int64(6) = {}", double_int64(6));
    println!("double_float32(2.5) = {}", double_float32(2.5));
    println!("double_float64(2.5) = {}", double_float64(2.5));
    println!(
        "int_plus_float_to_float(3, 2.5) = {}",
        int_plus_float_to_float(3, 2.5)
    );
    println!(
        "int_plus_float_to_int(3, 2.5) = {}",
        int_plus_float_to_int(3, 2.5)
    );
    println!("tuple_sum((10, 20)) = {}", tuple_sum((10, 20)));
    println!("array_sum([1, 2, 3]) = {}", array_sum([1, 2, 3]));
}

fn double_int32(input: i32) -> i32 {
    input * 2
}

fn double_int64(input: i32) -> i64 {
    input as i64 * 2
}

fn double_float32(input: f32) -> f32 {
    input * 2.0
}

fn double_float64(input: f32) -> f64 {
    input as f64 * 2.0
}

fn int_plus_float_to_float(input_int: i32, input_float: f32) -> f64 {
    input_int as f64 + input_float as f64
}

fn int_plus_float_to_int(input_int: i32, input_float: f32) -> i64 {
    input_int as i64 + input_float as i64
}

fn tuple_sum(input: (i64, i64)) -> i64 {
    input.0 + input.1
}

fn array_sum(input: [i64; 3]) -> i64 {
    input.iter().sum()
}
