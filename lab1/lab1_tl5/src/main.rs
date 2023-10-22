fn main() {
    let mut so1_input = String::new();
    let mut so2_input = String::new();

    println!("Nhap so 1: ");
    if let Err(_) = std::io::stdin().read_line(&mut so1_input) {
        println!("Loi");
    }

    println!("Nhap so 2: ");
    if let Err(_) = std::io::stdin().read_line(&mut so2_input) {
        println!("Loi");
    }

    let so1: f64 = so1_input.trim().parse().unwrap();
    let so2: f64 = so2_input.trim().parse().unwrap();

    println!("- Tong 2 so: {}", so1 + so2);
    println!("- Hieu 2 so: {}", so1 - so2);
    println!("- Tich 2 so: {}", so1 * so2);
    println!("- Thuong 2 so: {}", so1 / so2);
}
