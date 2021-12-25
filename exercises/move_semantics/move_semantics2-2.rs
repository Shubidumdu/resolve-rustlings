// move_semantics2-2.rs

// 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
//    and then copy the data within the function in order to return an owned
//    `Vec<i32>`

fn main() {
  let mut vec0 = Vec::new();

  // // answer 2
  let mut vec1 = fill_vec(&vec0);  

  // Do not change the following line!
  println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

  vec1.push(88);

  println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
  let mut vec = vec.clone();

  vec.push(22);
  vec.push(44);
  vec.push(66);

  vec
}