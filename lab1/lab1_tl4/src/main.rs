fn main() {
    let mut toan_input = String::new();
    let mut hoa_input = String::new();

    println!("Nhap diem toan: ");
    if let Err(_) = std::io::stdin().read_line(&mut toan_input) {
        println!("Loi nhap!");
    }

    println!("Nhap diem hoa: ");
    if let Err(_) = std::io::stdin().read_line(&mut hoa_input) {
        println!("Loi nhap!");
    }

    let toan: f64 = toan_input.trim().parse().unwrap();
    let hoa: f64 = hoa_input.trim().parse().unwrap();
    let tb = (toan + hoa) / 2.0;

    println!("Diem trung binh: {}", tb);
}
