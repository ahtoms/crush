
fn reverse_array(array: [i32; 6]) -> [i32; 6] {
    let mut n_array = [0; 6];
    for i in 0..array.len() {
        n_array[i] = array[array.len()-i-1]
    }
    n_array
}


fn main() {
    let array = [1, 2, 3, 4, 5, 6];
    let array = reverse_array(array); // Different variable
    
    for i in array.iter() {
        println!("{}", i);
    }
    
}
