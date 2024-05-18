fn main() {
    let arr = vec![200, 5, 2, 100, 50, 8, 3,1,6,4,7];
    let sorted_arr = merge_sort(arr);
    println!("{:?}", sorted_arr);
}

fn merge_sort(mut arr: Vec<i32>) -> Vec<i32>{
    //check of len of arr is less than one element  
    if arr.len() <= 1 {
        return arr;
    }

    let mid = arr.len() / 2;
    let left = merge_sort(arr.drain(0..mid).collect());
    let right = merge_sort(arr.into());

    merger(left, right)
}

fn merger(mut left: Vec<i32>, mut right: Vec<i32>)-> Vec<i32> {
    let mut result = Vec::new();
    
    while !left.is_empty() && !right.is_empty(){
        if left[0] <= right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }

    result.append(&mut left);
    result.append(&mut right);

    result
}