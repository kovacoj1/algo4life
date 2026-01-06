mod sorting;

fn main() {
    let mut data = vec![5, 2, 9, 1, 3];
    sorting::merge_sort(&mut data);
    println!("{:?}", data);
}
