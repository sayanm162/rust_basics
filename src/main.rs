//main function runs first
/* fn calculate_weight_on_earth(mass: f32) -> f32 {
    let mut gravity: f32 = 10.0;
    gravity = 9.81;
    (mass * gravity) as f32
        }
        
fn main() {
    println!("Hello, world! {}", calculate_weight_on_earth(5.0));
}
*/
pub mod utils;
use utils::calc::*;

fn main() {
    println!("Hello, world! {}", add(5, 2));
    println!("Hello, world! {}", sub(5, 2));
}