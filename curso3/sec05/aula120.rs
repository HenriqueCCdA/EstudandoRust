// bubble sort algorithm

fn swap_array(lista:&mut [i32;7], i: usize, j:usize) {
    let temp: i32 = lista[i];
    lista[i] = lista[j];
    lista[j] = temp;
}


fn main () {

    let mut array:[i32;7] = [10, 23, 4, 5, 66, 7, -3];

    for i in 0..array.len() {
        for j in ((i+1)..array.len()).rev() {
            println!("j -1 = {}, j = {}", array[j - 1], array[j]);

            if array[j-1] > array[j] {
                swap_array(&mut array, j-1, j);
            }
        }
        println!("{:?}", array);
    }

}
