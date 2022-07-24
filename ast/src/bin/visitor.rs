extern crate rayon;

#[derive(Debug)]
pub enum Expr<'a> {
    Num(i32),
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Mul(&'a Expr<'a>, &'a Expr<'a>),
}

pub trait Visitor<'a>: Sized + Sync {
    type R: Send;
    fn visit_expr(&self, e: &'a Expr<'a>) -> Self::R {
        walk_expr(self, e)
    }
    fn visit_mul(&self, l: &'a Expr<'a>, r: &'a Expr<'a>) -> Self::R;
    fn visit_add(&self, l: &'a Expr<'a>, r: &'a Expr<'a>) -> Self::R;
    fn visit_num(&self, i: i32) -> Self::R;
}

pub fn walk_expr<'a, V: Visitor<'a>>(v: &V, e: &'a Expr<'a>) -> V::R {
    // Maybe use join here?
    let mut r: Option<V::R> = None;
    rayon::scope(|scope| {
        scope.spawn(|_| {
            r = Some(match e {
                &Expr::Add(l, r) => v.visit_add(l, r),
                &Expr::Mul(l, r) => v.visit_mul(l, r),
                &Expr::Num(i) => v.visit_num(i),
            })
        });
    });
    r.unwrap()
}

pub fn walk_add<'a, V: Visitor<'a>>(v: &V, l: &'a Expr<'a>, r: &'a Expr<'a>) -> (V::R, V::R) {
    rayon::join(|| v.visit_expr(l), || v.visit_expr(r))
}

pub fn walk_mul<'a, V: Visitor<'a>>(v: &V, l: &'a Expr<'a>, r: &'a Expr<'a>) -> (V::R, V::R) {
    rayon::join(|| v.visit_expr(l), || v.visit_expr(r))
}

pub struct FoldVisitor;

pub fn fold_expr<'a, 'b>(e: &'a Expr<'b>) -> i32 {
    let f = FoldVisitor {};
    FoldVisitor::visit_expr(&f, e)
}

impl<'a> Visitor<'a> for FoldVisitor {
    type R = i32;
    fn visit_mul(&self, l: &'a Expr<'a>, r: &'a Expr<'a>) -> Self::R {
        let (l, r) = walk_add(self, l, r);
        l * r
    }

    fn visit_add(&self, l: &'a Expr<'a>, r: &'a Expr<'a>) -> Self::R {
        let (l, r) = walk_add(self, l, r);
        l + r
    }

    fn visit_num(&self, i: i32) -> Self::R {
        i
    }
}

fn main() {
    let e = Expr::Mul(&Expr::Num(5), &Expr::Add(&Expr::Num(10), &Expr::Num(5)));
    println!("{:?}", fold_expr(&e));
}
