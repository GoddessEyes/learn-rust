const MAXS: u32 = 1000;

fn main() {
    // PART:
    // // Const from global scope:
    println!("The value of x is: {}", MAXS);
    {
        // Const from local scope:
        const MAXS: u32 = 20000;
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
        println!("The value of x is: {}", MAXS);
    }
    // PART:
    // // First scope:
    // let x = 5;
    // // Another scope:
    // {
    //     let x = x + 1;
    //     println!("{}", x);
    // }
    // // First scope:
    // let x = x * 2;
    // println!("{}", x)

    // PART:
    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!("{}", spaces);

    // PART
    // let mut x = "_";
    // println!("The value of x is: {}", x);
    // spaces = x.len();
    // println!("The value of x is: {}", x);
}
