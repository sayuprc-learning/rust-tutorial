fn main() {
    // データ型

    // 複数の型が推論できる場合、型注釈が必要
    // u32 という型注釈
    // let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar data type

    // 整数型

    //           8bit 16bit 32bit 64bit arch(64arch -> 64bit/32arch -> 32bit)
    // 符号付き: i8   i16   i32   i64   isize
    // 符号なし: u8   u16   u32   u64   usize

    // 数値リテラル
    98_222; // 10進数
    0xff; // 16進数
    0o77; // 8進数
    0b1111_0000; // 2進数
    b'A'; // バイト(u8のみ)

    // 基本的に i32 を使っておけばいいらしい

    // 浮動小数点型(IEEE754準拠)
    let a = 2.0; // f64 倍精度
    let b: f32 = 3.0; //f32 単精度
    println!("{} {}", a, b);

    // 数値演算
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 0
    let reminder = 43 % 5;
    println!(
        "{} {} {} {} {} {}",
        sum, difference, product, quotient, floored, reminder
    );

    // 理論値型
    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);

    // 文字列型

    // char ユニコードのスカラー値
    let c = 'z';
    let d = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, d, heart_eyed_cat);

    // 複合型

    // タプル型 要素の型は必ずしも同じじゃなくてよい
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 分配
    println!("x: {} y: {} z: {}", x, y, z);
    // .数値 で直接その要素にアクセスできる
    println!("First value of tup: {}", tup.0);

    // 配列型 要素の型はすべて同じ
    // スタックにメモリ確保したいときは配列のほうがいいらしい
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let a = [3; 5]; // 3 が 5 個ある配列になる
    println!("{:?}", a);

    // 各要素へのアクセスはほかの言語と同じ感じ
    let first = a[0];
    let second = a[1];
    println!("{} {}", first, second);

    // 固定長のものを使うときは配列がよさげ
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{:?}", months);
}
