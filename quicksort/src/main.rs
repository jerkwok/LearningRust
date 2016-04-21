type OrderFunc<T> = Fn(&T, &T) -> bool;
 
fn lessthan<T: Ord>(x: &T, y: &T) -> bool {
    x < y
}
 
fn quicksort<T>(data: &mut [T], comparator: &OrderFunc<T>) {
 
    let len = data.len();
    if len < 2 {
        return;
    }
 
    let pivot = partition(data, comparator);
 
    // left side
    quicksort(&mut data[0..pivot], comparator);
 
    // right side
    quicksort(&mut data[pivot + 1..len], comparator);
}
 
fn partition<T>(data: &mut [T], comparator: &OrderFunc<T>) -> usize {
    let len = data.len();
    let pivot = len / 2;
 
    data.swap(pivot, len - 1);
 
    let mut store_index = 0;
    for i in 0..len - 1 {
        if comparator(&data[i], &data[len - 1]) {
            data.swap(i, store_index);
            store_index += 1;
        }
    }
 
    data.swap(store_index, len - 1);
    store_index
}

 
fn main() {
    let mut numbers = [12,5,13,4,8,14,17,21,1,25];
    quicksort(&mut numbers, &lessthan);
    println!("{:?}", numbers);
 
}