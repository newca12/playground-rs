#[derive(Debug)]
enum Expr<'a> {
    Num(i32),
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Mul(&'a Expr<'a>, &'a Expr<'a>),
}

fn fold_expr<T, Unary: Fn(i32) -> T, Add: Fn(T, T) -> T, Mul: Fn(T, T) -> T>(
    unary: &Unary,
    add: &Add,
    mul: &Mul,
    e: &Expr,
) -> T {
    let rec = |e: &Expr| fold_expr(unary, add, mul, e);
    match e {
        &Expr::Num(i) => unary(i),
        &Expr::Add(l, r) => add(rec(l), rec(r)),
        &Expr::Mul(l, r) => mul(rec(l), rec(r)),
    }
}

fn main() {
    // full traversel
    let has_add = |e: &Expr| fold_expr(&|_| false, &|_, _| true, &|_, _| false, e);
    let eval = |e: &Expr| fold_expr(&|a| a, &|a, b| a + b, &|a, b| a * b, e);
    let e = Expr::Add(&Expr::Num(5), &Expr::Mul(&Expr::Num(10), &Expr::Num(5)));
    let e1 = Expr::Mul(&Expr::Num(4), &Expr::Num(2));
    println!("{:?}", eval(&e));
    println!("{:?}", has_add(&e));
    println!("{:?}", has_add(&e1));
}
