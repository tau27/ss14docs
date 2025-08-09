use std::io;

pub fn choose(title: String, chooses: Vec<(String, String)>) -> String {
    let mut guess = String::new();
    let mut ct = 0;
    println!("{}", title);
    for i in &chooses {
        println!("{}. {}", ct, i.1);
        ct += 1;
    }
    let _ = io::stdin().read_line(&mut guess)
        .expect("Failed to read_line");
    let getter: usize = guess
        .trim()
        .parse()
        .unwrap();
    let result = String::from(chooses[getter].0.clone());
    result
}

pub fn y_or_n(title: &str) -> bool {
    println!("{} (y, n)", title);
    let mut guess = String::new();
    let _ = io::stdin().read_line(&mut guess)
        .expect("Failed to read_line");

    guess == "y\n".to_string()
}

pub fn asker(title: &str) -> String {
    println!("{}", title);
    let mut aska = String::new();
    let _ = io::stdin().read_line(&mut aska)
        .expect("Failed to read_line");
    aska.pop();

    aska
}
