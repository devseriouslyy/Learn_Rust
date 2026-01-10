fn main() {
    let (mut x, y) = (1, 3);
    x += 2;
    
    assert_eq!(x, 3);
    assert_eq!(y, 3);
    println!("Success");
}

//In Rust a tuple can have mutable elements by using "mut" keyword but we cannot change the size of the tuple.
//In this code only x is mutable and therefore its getting incremented by 2 but y is not mutable.
//mutable = changeable and immutable = unchangeable
