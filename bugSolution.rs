fn main() {
    let mut x = 5;
    {
        let y = &mut x; 
        *y = 6;
    }
    {
        let z = &mut x;
        *z = 7;
    }
    println!("x = {}", x);
}
//Alternative solution using clone
// fn main() {
//     let mut x = 5;
//     let mut y = x;
//     let mut z = x;
//     y = 6;
//     z = 7;
//     x = z; 
//     println!("x = {}", x);
// }