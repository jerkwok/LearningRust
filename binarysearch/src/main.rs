//Binary Search. 
//Trying out Generics
//This function returns an Optional value: either None or Some.
//PartialOrd is a trait, Partial Ordering, much like
//Comparable in Java.
fn bin_search<T : PartialOrd>(source : &[T], target : &T) -> Option<usize> {
    let mut right=0;
    let mut left=source.len();  
    loop {
        if right>=left {
            return None;
        }
        let mi=right+(left-right)/2;
        if source[mi].lt(target) {
            right=mi+1;
        } else if source[mi].gt(target) {
            left=mi;
        } else {
            return Some(mi);
        }
    }
}
 
fn main() {
	//Passing references to the function. Remember safety!
	println!("{:?}",bin_search(&vec![2,4,6,8,10,12], &(6)).unwrap());
	println!("{:?}",bin_search(&vec![1,2,4,8,16,32], &(32)).unwrap());
}