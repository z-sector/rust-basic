// Слайсы. (мы спрашиваем эту задачку на собеседования на уровено Junior Engineer)
// Ring Buffer (кольцевой буффер) - структура данных, которая позволяет очень удобно реализовывать очередь на массиве фиксированного размера.
// https://ru.wikipedia.org/wiki/%D0%9A%D0%BE%D0%BB%D1%8C%D1%86%D0%B5%D0%B2%D0%BE%D0%B9_%D0%B1%D1%83%D1%84%D0%B5%D1%80
// Ключевая идея в том, что заполняя буффер до конца мы переходим в начало
// Пример API, вызовов и как меняется состояние буффера:
// [ _ _ _ ] create(3)
// [ a b _ ] write "ab" -> return 2
// [ a b c ] write "cd" -> return 1
// [ _ b c ] read(1) -> return "a"
// [ e b c ] write "e" -> return 1
// [ e _ _ ] read(2) -> return "bc"
// Ваша задача написать такой буффер и добавить тесты

#[derive(Debug)]
struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
    count: usize,
}

fn create(size: usize) -> RingBuffer {
    RingBuffer {
        read_idx: 0,
        write_idx: 0,
        data: vec![0; size],
        count: 0,
    }
}

fn write(rb: &mut RingBuffer, input: &[u8]) -> usize {
    let mut written = 0;
    for &byte in input {
        if rb.count == rb.data.len() {
            break; // buffer full
        }
        rb.data[rb.write_idx] = byte;
        rb.write_idx = (rb.write_idx + 1) % rb.data.len();
        written += 1;
        rb.count += 1;
    }
    written
}

fn read(rb: &mut RingBuffer, count: usize) -> Vec<u8> {
    let mut result = Vec::new();
    for _ in 0..count {
        if rb.count == 0 {
            break; // buffer empty
        }
        result.push(rb.data[rb.read_idx]);
        rb.data[rb.read_idx] = 0; // clear after read
        rb.read_idx = (rb.read_idx + 1) % rb.data.len();
        rb.count -= 1;
    }
    result
}

fn main() {
    let mut rb = create(3);
    println!("{:?}", rb);

    write(&mut rb, b"ab");
    println!("{:?}", rb);

    write(&mut rb, b"cd");
    println!("{:?}", rb);

    let read_data = read(&mut rb, 1);
    println!("Read: {:?}", String::from_utf8(read_data).unwrap());

    write(&mut rb, b"e");
    println!("{:?}", rb);

    let read_data = read(&mut rb, 2);
    println!("Read: {:?}", String::from_utf8(read_data).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ring_buffer_basic() {
        let mut rb = create(3);
        assert_eq!(write(&mut rb, b"ab"), 2);
        assert_eq!(rb.data, vec![b'a', b'b', 0]);
        assert_eq!(write(&mut rb, b"cd"), 1);
        assert_eq!(rb.data, vec![b'a', b'b', b'c']);
        assert_eq!(read(&mut rb, 1), vec![b'a']);
        assert_eq!(rb.data, vec![0, b'b', b'c']);
        assert_eq!(write(&mut rb, b"e"), 1);
        assert_eq!(rb.data, vec![b'e', b'b', b'c']);
        assert_eq!(read(&mut rb, 2), vec![b'b', b'c']);
        assert_eq!(rb.data, vec![b'e', 0, 0]);
    }

    #[test]
    fn test_ring_buffer_overflow() {
        let mut rb = create(2);
        assert_eq!(write(&mut rb, b"abc"), 2);
        assert_eq!(rb.data, vec![b'a', b'b']);
        assert_eq!(rb.count, 2);
    }

    #[test]
    fn test_ring_buffer_empty_read() {
        let mut rb = create(2);
        assert_eq!(read(&mut rb, 1), vec![]);
        assert_eq!(rb.count, 0);
    }

    #[test]
    fn test_ring_buffer_full_cycle() {
        let mut rb = create(2);
        assert_eq!(write(&mut rb, b"ab"), 2);
        assert_eq!(read(&mut rb, 1), vec![b'a']);
        assert_eq!(write(&mut rb, b"c"), 1);
        assert_eq!(read(&mut rb, 2), vec![b'b', b'c']);
        assert_eq!(rb.count, 0);
    }
}
