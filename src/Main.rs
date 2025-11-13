// A new snake project begins...

// Screen fuctions
fn showcreen() {
    // code to show an 8 by 8 screen
    for y in 0..8 {
        for x in 0..8 {
            print!("[ ]");
        }
        println!();
    }
}

fn setpoint(x: usize, y: usize, icon: char) {
    // code to set a point on the screen
    for y in 0..8 {
        for x in 0..8 {
            if x == x && y == y {
                print!("{}", icon);
            } else {
                print!("[ ]");
            }
        }
        println!();
    }

}

// Main function
fn main() {
    println!("Loading...");
    showcreen();
    setpoint(3, 4, 'X');
    println!("Screen displayed.");
}