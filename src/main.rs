
fn main() {
    // println!("Hello, world!");
    display();
}


fn display(){
//64 x 32 pixels
    const DISPLAY_WIDTH: usize = 64;
    const DISPLAY_HEIGHT: usize = 32;

    
    let mut display_arr: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];


   for i in 0..display_arr.len(){
    for j in 0..display_arr[i].len(){
        let mut output:char = ' ';
        if(display_arr[i][j] == true){
            output = '█'; 
        }
        print!("{}",output);
    }
    println!("");
   }

   
}

//https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
/*
1. Memory
2. Display
    How to go about creating a display? Need a popout console?
        Research.
3. Program Counter
4. Index Register
5. Stack
6. Delay Timer
7. Sound Timer
8. Variable Register


*/