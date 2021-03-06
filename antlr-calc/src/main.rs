use antlr_calc::str_to_calc;
use antlr_calc::str_to_calc_v3;

fn main() {
    let data = r#"
    193
    a = 5
    b = 6
    c = b
    a+b
    a-b
    a+b*2
    (1+2)*3
    "#;

    let result = str_to_calc(data);
    println!("{:?}", result);
    let result = str_to_calc_v3(data);
    println!("{:?}", result);
}
