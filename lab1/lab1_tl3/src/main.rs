fn main() {
    let mut msv_input = String::new();
    let mut ten_input = String::new();
    let mut tuoi_input = String::new();
    let mut sdt_input = String::new();
    let mut email_input = String::new();
    let mut diachi_input = String::new();
    let mut gioitinh_input = String::new();

    println!("Nhap msv: ");
    if let Err(_) = std::io::stdin().read_line(&mut msv_input) {
        println!("Khong doc duoc");
    }

    println!("Nhap ten sv: ");
    if let Err(_) = std::io::stdin().read_line(&mut ten_input) {
        println!("Khong doc duoc");
    }

    println!("Nhap tuoi: ");
    if let Err(_) = std::io::stdin().read_line(&mut tuoi_input) {
        println!("Khong doc duoc");
    }

    println!("Nhap sdt: ");
    if let Err(_) = std::io::stdin().read_line(&mut sdt_input) {
        println!("Khong doc duoc");
    }

    println!("Nhap email: ");
    if let Err(_) = std::io::stdin().read_line(&mut email_input) {
        println!("Khong doc duoc");
    }

    println!("Nhap dia chi: ");
    if let Err(_) = std::io::stdin().read_line(&mut diachi_input) {
        println!("Khong doc duoc");
    }

    println!("Nhap gioi tinh: ");
    if let Err(_) = std::io::stdin().read_line(&mut gioitinh_input) {
        println!("Khong doc duoc");
    }

    println!("\n- Ma SV: {}", msv_input.trim());
    println!("- Ten SV: {}", ten_input.trim());
    println!("- Tuoi: {}", tuoi_input.trim());
    println!("- SDT: {}", sdt_input.trim());
    println!("- Email: {}", email_input.trim());
    println!("- Dia Chi: {}", diachi_input.trim());
    println!("- Gioi Tinh: {}", gioitinh_input.trim());
}
