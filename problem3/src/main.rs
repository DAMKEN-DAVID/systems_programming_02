#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    let mut result: i32 = 0;
    for num in low..=high {
        result += num;
    }
    println!("{}", result);
    *total = result;
}

fn main() {
    // create necessary variables and test your function for low 0 high 100
    // total should be 5050
    let low: i32 = 0;
    let high: i32 = 100;
    let mut result: i32 = 0;
    sum(&mut result, low, high);
    println!("{}", result);

}