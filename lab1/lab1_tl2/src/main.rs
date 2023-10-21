use std::io::Write;

fn main() {
    let mut input = String::new();

    print!("Nhap ban kinh hinh tron = ");
    let _ = std::io::stdout().flush();

    if let Err(_) = std::io::stdin().read_line(&mut input) {
        println!("Khong doc duoc");
    }

    let r: f64 = input.trim().parse().expect("Khong phai so nguyen");

    let d = 3.14 * (r * r);
    println!("Dien tich hinh tron = {}", d);

    let c = 3.14 * (r * 2.0);
    println!("Chu vi hinh tron = {}", c);
}
