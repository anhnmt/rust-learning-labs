use std::io;
use std::io::Write;

fn main() {
    let mut num = String::new();

    print!("Nhap n = ");

    // https://stackoverflow.com/questions/34993744/why-does-this-read-input-before-printing
    let _ = io::stdout().flush();

    if let Err(err) = io::stdin().read_line(&mut num) {
        println!("Khong the nhan duoc du lieu: {}", err);
    }

    let n: i64 = num.trim().parse().expect("Khong phai so nguyen");

    let result: i64 = n * n;
    println!("Binh phuong n = {}", result);
}
