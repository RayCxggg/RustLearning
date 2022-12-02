fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    let x = five();

    println!("The value of y is: {y} {x}");
}

fn five() -> i32 {
    5
}
