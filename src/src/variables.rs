fn main() {
    // イミュータブルな変数
    let x = 5;
    println!("The value of x is: {}", x);

    // x = 6; // x は immuable なので再代入不可
    // println!("The value of x is: {}", x);

    // ミュータブルな変数
    let mut y = 5;
    println!("The value of y is: {}", y);

    y = 6; // y は mutable なので再代入可能
    println!("The value of y is: {}", y);

    // 定数
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINT is: {}", MAX_POINTS);

    // シャドーイング
    let z = 5;
    let z = z + 1; // let を使うと、該当スコープでの最後に宣言された値が使われる(再代入とは異なる)

    {
        let z = z * 2;
        println!("The value of x in the inner scope is: {}", z);
    }

    println!("The value of z is: {}", z);

    // // mut にしないことで型の異なるものでも使える
    // // 意味合い的には変数をもう一つ作るのと同じ
    let space = "   "; // string
    let space = space.len(); // uzise
    println!("{}", space);

    // let mut space = "  ";
    // space = space.len(); // これは型が違うので動かない
}
