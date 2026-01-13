fn main() {
    let v1: u8 = 241_u8 + 8;
    let v2: u8 = 160_u8 + 9;

    let sum: u16 = v1 as u16 + v2 as u16;

    println!("sum = {}", sum);
}

//Integer types in Rust (u= unsigned int and i = signed int)
// unsigned int can only store non-negtaive numbers
// signed int can store negtaive and positive numbers
//Type	Bits	Range
//u8	8	0 → 255
//i8	8	-128 → 127
//u16	16	0 → 65,535
//i16	16	-32,768 → 32,767
//u32	32	0 → 4,294,967,295
//i32	32	-2,147,483,648 → 2,147,483,647
//u64	64	0 → 18,446,744,073,709,551,615
//i64	64	≈ -9e18 → ≈ +9e18
