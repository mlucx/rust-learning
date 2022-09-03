use std::io;

fn main() {
    println!("Enter your rgb value, seperated by spaces: ");

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<&str> = input.split(' ').collect();
    
    println!("Converted your input to hex: {}", rgb_to_hex(nums[0].trim().parse().unwrap(), nums[1].trim().parse().unwrap(), nums[2].trim().parse().unwrap()));
}


fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!
    ( 
        "#{:02X}{:02X}{:02X}", 
        r, 
        g, 
        b
    ) 
}