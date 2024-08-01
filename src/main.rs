use num_traits::{Float, ToPrimitive};

//f32 and f64 both implement 'Float' traits
fn solve<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    // let a: f32 = 3.0;
    // let b: f32 = 4.0;

    // let a: f64 = 3.0;
    // let b: f64 = 4.0;

    let a: f32 = 3.0;
    let b: f64 = 4.0;

    //println!("{}", solve::<f64>(a, b));

    println!("{}", solve(a, b));
}
