


fn solve(num:i32, ans: &str) -> &str{
    if num > 0 {
        println!("Number is positive: {}", ans);
    } else if num < 0 {
        println!("Number is negative");
    } else{
        println!("Equal zero")
    }
    // for loop
    for x in 1..11{
        println!("x is {}", x);
    }
    // or while
    let mut x = 0;
    while x < 10 {
        println!("while x is {}", x);
        x+=1;
    }

    // tuple 
    // let tuple:(i32, f64, u8) = (1, 4.5, 22);
    let tuple = (1, 4.5, 22);
    println!("tuple {} {} {}", tuple.0, tuple.1, tuple.2);
    // array 
    let arr:[i32;4] = [1, 2, 3, 4];
    let arr = [1, 2, 3, 4];
    println!("array {} {} {}", arr[0], arr[1], arr[2]);

    // Ownership
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("ownership {:?}", v2);
    return ans;
}

// This is single line comments
/*
 * This is multi-line comments
 */
fn main(){
    solve(5, "Ne");
    let mut title_string:&str = "Learn Rust";
    let rating_float = 4.5;
    let is_confirm = true;
    title_string = "Learn Rust Lang";
    const PI:f32 = 3.14;
    println!("{} {} {} {}",title_string, rating_float, is_confirm, PI);
    // Ownership
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("ownership {:?}", v2);

}
