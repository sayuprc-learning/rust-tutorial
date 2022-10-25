fn main() {
    // if 式
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 7;

    // 暗黙的に論理値への変換はされない
    // if number {
    //     println!("number was seven");
    // }

    // if に渡すのは必ず理論値
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let と組み合わせることができる
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // 型が異なるものは代入できない
    // let number = if condition { 5 } else { "six" };

    // ループ

    // loop
    // loop {
    //     // これだと無限ループ
    //     println!("again!");
    // }

    let mut count = 0;

    // ラベルを付けられる
    'counting_up: loop {
        println!("count = {}", count);

        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up; // ラベルを付けたループを抜ける
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    // while

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        // for なら全部舐められる
        println!("The value is: {}", element);
    }

    // おまけ
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LEFTOFF!!!");

    // まとめ
    // 今はやらん

    // 温度を華氏と摂氏で変換するプログラム

    // フィボナッチ数列のn番目を生成する

    // クリスマスキャロルの定番、"The Twelve Days of Christmas"の歌詞を、 曲の反復性を利用して出力する。
}
