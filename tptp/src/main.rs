use tptp::str_to_visit;

fn main() {
    println!("Start");
    let input = r#"fof(propositional,axiom,
        ( ( p0
          & ~ q0 )
       => ( r0
          | ~ s0 ) ))."#;

    str_to_visit(input);
}
