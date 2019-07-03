
fn reverse_array(array: &mut [i32; 6]) {
    for i in 0..array.len()/2 {
        let t = array[i];
        array[i] = array[6-i-1];
        array[6-i-1] = t;
    }
}


fn main() {
    let mut array = [1, 2, 3, 4, 5, 6];
    reverse_array(&mut array);
    
    for i in array.iter() {
        println!("{}", i);
    }
    
}
