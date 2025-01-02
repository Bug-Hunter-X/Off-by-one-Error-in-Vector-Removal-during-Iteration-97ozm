fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut index = 0;

    loop {
        if index >= numbers.len() {
            break;
        }

        println!("The value at index {} is: {}", index, numbers[index]);

        if numbers[index] == 3 {
            numbers.remove(index);
        } else {
            index += 1; 
        }
    }
}