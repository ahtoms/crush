fn reverse(array: &mut [i32]) {
    let length = array.len();
    for i in 0..(length/2) {
        let t = array[i];
        array[i] = array[length-i-1];
        array[length-i-1] = t;
    }
}

fn main() {
    let mut array = [1, 2, 3, 4, 5, 6];
    reverse(&mut array);
    
    for i in array.iter() {
        println!("{}", i);
    }
    
    
}
