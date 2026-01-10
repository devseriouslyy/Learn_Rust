fn main() {
    let (x, y);
    
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    
    assert_eq!([x,y], [3, 2]);
    println!("Success!");
   
}

//This is an example of "destructing assignments".
//The square brackets are Array Datastructure
