// 変更の可否に関わらず、参照はもとの所有者のライフタイムよりも長く生きることはできない
// 元の所有者が値を破棄してしまったら、参照できる値もなくなってしまうため

fn main() {
    let y;

    {
        // ----xのライフタイム----
        let x = 5;
        // ----yのライフタイム----
        y = &x;
        dbg!(x);
        // ----xのライフタイム----
    }

    dbg!(y);
    // ----yのライフタイム----
}