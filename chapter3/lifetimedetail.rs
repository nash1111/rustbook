fn main() {
    let mut x = 5;
    
    let y = &x;

    let z = &mut x;

    dbg!(z);

    dbg!(x);
}

// xのライフタイム: 2~10行目
// yのライフタイム: 4行目
// zのライフタイム: 6~8行目

// ライフタイム決定の流れ
// 1. 参照のライフタイム
//    参照が利用されている期間に対応したライフタイム
// 2. 値のライフタイム
//    値が開放される直前までの期間に対応したライフタイム