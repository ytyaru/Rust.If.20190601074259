/*
 * Rustのif式。
 * CreatedAt: 2019-06-01
 */
fn main() {
    if true { println!("if");}

    if false { println!("if");}
    else { println!("else");}

    if false { println!("if");}
    else if true { println!("else if 1");}
    else if true { println!("else if 2");}
    else if true { println!("else if 3");}
    else { println!("else");}

//    if 1 { println!("if 1");} // error[E0308]: mismatched types

    let v = if true { 5 } else { 6 };
    println!("v = {}", v);
//    let v = if true { 5 } else { "six" }; // error[E0308]: if and else have incompatible types
}

