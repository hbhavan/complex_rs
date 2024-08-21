mod complex;
mod lateral;
mod operators;

use complex::Complex;

use crate::lateral::Lateral;

fn init_lateral_test() {
    println!("---INIT LATERAL TEST---");
    let w1 = Lateral::new(1.0);
    let w2 = Lateral::new(2.0);
    let w3 = Lateral::new(3.5);

    println!("w1 = {}", w1);
    println!("w2 = {}", w2);
    println!("w3 = {}", w3);
}

fn add_lateral_test() {
    println!("---ADD LATERAL TEST---");
    let w1 = Lateral::new(1.0);
    let w2 = Lateral::new(2.5);

    let w3 = &w1 + &w2;

    println!("{} + {} = {}", w1, w2, w3);
}

fn mul_lateral_test() {
    println!("---MULTIPLY LATERAL TEST---");
    let w1 = Lateral::new(1.0);
    let w2 = Lateral::new(2.5);

    let w4 = Lateral::new(2.4);
    let x = 2;

    let w3 = &w1 * &w2;
    let w5 = &w4 * &x;

    println!("{} * {} = {}", w1, w2, w3);
    println!("{} * {} = {}", x, w4, w5);
}

fn init_complex_test() {
    println!("---INIT COMPLEX TEST---");
    let z1 = Complex::new(2.0, Lateral::new(3.2));
    let z2 = Complex::new(2.2, Lateral::new(0.0));
    let z3 = Complex::new(3.2, Lateral::new(1.0));

    println!("z1 = {}", z1);
    println!("z2 = {}", z2);
    println!("z3 = {}", z3);
}

fn add_complex_test() {
    println!("---ADD COMPLEX TEST---");
    let z1 = Complex::new(2.1, Lateral::new(1.2));
    let z2 = Complex::new(2.0, Lateral::new(5.5));

    let x = 3.8;
    let w = Lateral::new(6.2);

    let z3 = &z1 + &x;
    let z4 = &z1 + &w;
    let z5 = &z1 + &z2;

    println!("{} + {} = {}", z1, x, z3);
    println!("{} + {} = {}", z1, w, z4);
    println!("{} + {} = {}", z1, z2, z5);
}

fn mul_complex_test() {
    println!("---MULTIPLY COMPLEX TEST---");
    let z1 = Complex::new(2.1, Lateral::new(1.2));
    let z2 = Complex::new(2.0, Lateral::new(5.5));

    let x = 3.8;
    let w = Lateral::new(6.2);

    let z3 = &z1 * &x;
    let z4 = &z1 * &w;
    let y = &z1 * &z2;

    println!("{} * {} = {}", z1, x, z3);
    println!("{} * {} = {}", z1, w, z4);
    println!("{} * {} = {}", z1, z2, y);
}

fn main() {
    init_lateral_test();
    add_lateral_test();
    mul_lateral_test();

    init_complex_test();
    add_complex_test();
    mul_complex_test();
}
