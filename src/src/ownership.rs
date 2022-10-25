fn main() {
    {
        // s はこの時点では未定義
        // let s = "hello"; // s が有効になる
    } // スコープが終わり、s は無効となる

    // 文字列リテラルから String 型にする
    // let s = String::from("hello");

    // String は可変にできる
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    {
        // s はこの時点では未定義
        // let s = String::from("hello"); // s が有効になる
    } // スコープが終わり、s は無効となる
      // 閉じ波括弧で特別な関数 drop を自動的に呼び出して、メモリ(ヒープ)を開放してくれている

    // 値はどちらも 5 だが、スタックには 2 つ積まれている
    // let x = 5;
    // let y = x;

    // スタックには 2 つ積まれるが、ヒープにある値はそのまま
    // ポインタが同じところを指している
    // let s1 = String::from("hello");
    // let s2 = s1;

    // s2 の生成後、s1 は使えなくなる
    // Rust では s1 はもう不要のものと判断する => s1 は s2 にムーブされた // shallow copy みたいな感じ
    // println!("{}, world", s1);

    // ヒープの deep copy が必要なら clone をする
    let s1 = String::from("hello");
    let s2 = s1.clone(); // ヒープもコピーされている(性能は落ちちゃう)

    println!("{} {}", s1, s2);

    // ムーブされるなら下記コードも動かないはず
    let x = 5;
    let y = x; // clone しても同じ
    println!("x = {} y = {}", x, y); // 動くんだなぁそれが
                                     // スタック上に保持されるものはヒープを使った時のような不安もなければ、性能の劣化も少ないので、shallow copy と deep copy が同じことを示している
                                     // よって shallow copy 以上のことはしないからムーブする必要もない

    // これらは Copy の型の一部
    // 整数型
    // 論理値型
    // 浮動小数点型
    // char 型
    // タプル(Copy の型のみ)

    let s = String::from("hello"); // s がスコープに入る

    takes_ownership(s); // s の値が関数にムーブされる
                        // この時点で s はもう有効ではない

    let x = 5; // x がスコープに入る
    makes_copy(x); // x も関数にムーブされる
                   // x は i32 型 であり、Copy なのでこの後も x は利用可能
    println!("x = {}", x);

    let s1 = gives_ownership(); // gives_ownership は戻り値を s1 にムーブ

    let s2 = String::from("hello"); // s2 がスコープに入る

    let s3 = takes_and_gives_back(s2); // s2 は takes_and_gives_back にムーブされる
                                       // また、戻り値は s3 にムーブされる

    println!("{} {}", s1, s3);

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4); // s4 は 関数にムーブされる
    println!("'{}' is {}", s5, len);
} // s1 と s3 はスコープを抜け、drop が呼ばれることによりメモリ解放される
  // s と s2 はすでにムーブされているので何も起こらない
  // x はスコープを抜けるだけ

fn takes_ownership(some_string: String) {
    // some_string がスコープに入る
    println!("{}", some_string);
} // some_string はスコープを抜け、drop が呼ばれてメモリが解放される

fn makes_copy(some_integer: i32) {
    // some_integer がスコープにはいる
    println!("{}", some_integer);
} // some_integer はスコープを抜ける。ヒープは使ってないので、特に何も起こらない

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string // 呼び出し元に some_string がムーブされる
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string がスコープに入る
    a_string // s_string は呼び出し元にムーブされる
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // String の長さを返す

    (s, length)
}
