use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("数の予想");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("生成した値: {}", secret_number);

    loop {
        println!("予想した値を入力してください。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました。");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("次のように予測しています: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さい"),
            Ordering::Greater => println!("大きい"),
            Ordering::Equal => {
                println!("正解");
                break;
            }
        }
    }
}
