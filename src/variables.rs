pub fn run() { //shadowing
    /*shadowing_example();
    type_change_example();
    shadowing_type_change();
    integer_example();
    float_example();
    char_example();
    boolean_example();
    parse_practice();*/
    tuple_example();
    array_example();
}
fn mutability_example() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing_example() {
   let x=5;
   let x=x+1;
   let x=x*2;
   println!("The value of x is: {}", x);
}

fn type_change_example() {
    let spaces="   ";
    let spaces= spaces.len();
    println!("The number of spaces is: {spaces}");
}

fn shadowing_type_change() {
    let username= "Neha";
    let username= username.len();
    println!("{username}");

}

fn integer_example() 
{
    let x= 5;
    let y= -10;
    println!("x: {}, y: {}", x, y);
}

fn float_example() {
    let pi = 3.14;

    println!("{pi}");
}

fn char_example() {
    let grade = 'A';
    let heart = '❤';

    println!("{grade}");
    println!("{heart}");
}
fn boolean_example() {
    let is_raining = false;
    let rust_is_fun = true;

    println!("{is_raining}");
    println!("{rust_is_fun}");
}

fn parse_practice()  {

   let x= "42";
    let x= x.parse::<u32>().unwrap();
    let x = x as f64;
    println!("{:.1}", x/2.0);

}

//tuples
fn tuple_example() {
let tup = (500, 6.4, 'A');
println!("The value of tup is: {:?}", tup);
println!("The first element of tup is: {}", tup.0);
//unpacking the tuple
let (x,y,z)= tup;
println!("The value of y is: {}", y);
//intentionally ignoring values
let (a, _, c) = tup;
println!("The value of a is: {}", a);
println!("The value of c is: {}", c);
}

//arrays
fn array_example() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of arr is: {:?}", arr); 
    //println!("{}", arr[10]); //this will cause a panic at runtime
    let arr2 = [0; 5]; //array of 5 elements, all initialized to 0
    println!("The value of arr2 is: {:?}", arr2);
}
