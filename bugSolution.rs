fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Solution 1: Using an iterator
    numbers.retain(|&x| x != 3);
    println!("Solution 1: {:?}", numbers);

    //Solution 2: Iterating backward
    let mut numbers2 = vec![1,2,3,4,5];
    for i in (0..numbers2.len()).rev() {
        if numbers2[i] == 3 {
            numbers2.remove(i);
        }
    }
    println!("Solution 2: {:?}", numbers2);

    // Solution 3: Using a while loop and checking length before accessing index
    let mut numbers3 = vec![1, 2, 3, 4, 5];
    let mut i = 0;
    while i < numbers3.len() {
        if numbers3[i] == 3 {
            numbers3.remove(i);
        } else {
            i += 1;
        }
    }
    println!("Solution 3: {:?}", numbers3);
} 