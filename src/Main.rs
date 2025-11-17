// A new snake project begins...

// Define the struct to hold the screen data. (AI)
// We derive the Debug trait so we can print the struct easily for debugging if needed. (AI)
#[derive(Debug)]
struct Screen8x8 {
    // The screen is an 8x8 array of u8 (0 or 1 for pixel on/off). (AI)
    data: [[u8; 8]; 8],
}

// Implement methods for the Screen8x8 struct (AI)
impl Screen8x8 {
    // A constructor-like function to create a new, empty screen instance. (AI)
    fn new() -> Self {
        Screen8x8 {
            data: [[0; 8]; 8],
        }
    }

    // Method to set a point on the screen. (AI)
    // &mut self means this method borrows the struct mutably, allowing modification. (AI)
    fn set_point(&mut self, x: usize, y: usize, value: u8) {
        if x < 8 && y < 8 {
            // Access the internal data using self.data (AI)
            self.data[y][x] = value;
            println!("Set point ({}, {}) to value {}.", x, y, value);
        } else {
            eprintln!("Error: Coordinates ({}, {}) are out of bounds (0-7).", x, y);
        }
    }

    // Method to display the current screen state. (AI)
    // &self means this method borrows the struct immutably (read-only). (AI)
    fn show(&self) {
        println!("--- Current Screen State ---");
        const ON: char = '█';
        const OFF: char = ' ';
        
        // Access the internal data using self.data (AI)
        for row in 0..8 {
            for col in 0..8 {
                let pixel = if self.data[row][col] == 1 { ON } else { OFF };
                print!("{} ", pixel);
            }
            println!();
        }
        println!("----------------------------");
    }
}

// Main function
fn main() {
    // Create a new screen instance (I AI)
    let mut screen = Screen8x8::new();

    // Set some points on the screen (I AI)
    screen.set_point(0, 0, 1);
    screen.set_point(1, 1, 1);
    screen.set_point(2, 2, 1);
    screen.set_point(3, 3, 1);
    screen.set_point(4, 4, 1);
    screen.set_point(5, 5, 1);
    screen.set_point(6, 6, 1);
    screen.set_point(7, 7, 1);

    // Display the current state of the screen (I AI)
    screen.show();
    sleep::sleep(std::time::Duration::from_secs(2));
}