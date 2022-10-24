fn main() {
    println!("Hello world!");

    another_functino(5);

    print_labeled_measurement(5, 'h');

    // let y = 6; // 文 statement

    // これはエラーになる
    // let x = (let y = 6); // let は文のため

    // ブロック {} は式
    let y = {
        let x = 3;
        // 式は文末にセミコロンをつけない
        // つけると文になってしまう
        x + 1
    };
    println!("The vlaue of y is: {}", y);

    let x = five();
    println!("The vlaue of x is: {}", x);

    let x = plus_one(x);
    println!("The value of x is: {}", x);
}

fn another_functino(x: i32) {
    println!("The value of x is: {}", x)
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// 関数の返り値の型は -> type で表す
// return は早期リターンの時ぐらいしか使わないらしい
// 返る値は関数内部の最後の式を暗黙的に返す
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
