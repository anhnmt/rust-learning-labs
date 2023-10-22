fn main() {
    let mut n = String::new();

    println!("Nhap so nguyen co 3 chu so: ");
    if let Err(_) = std::io::stdin().read_line(&mut n) {
        println!("Khong nhap duoc");
        return;
    }

    let n = n.trim();
    if n.len() != 3 {
        println!("Yeu cau 3 chu so");
        return;
    }


}
