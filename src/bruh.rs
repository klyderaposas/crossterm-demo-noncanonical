use crossterm::terminal::{disable_raw_mode, enable_raw_mode};use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::*;
/* 
    ____             __         _                                                     ___             __         
   / __ )_______  __/ /_       (_)___ ___     ____ _   _________  ____ ___  ___  ____/ (_)____   ____/ /__ _   __
  / __  / ___/ / / / __ \     / / __ `__ \   / __ `/  / ___/ __ \/ __ `__ \/ _ \/ __  / / ___/  / __  / _ \ | / /
 / /_/ / /  / /_/ / / / /    / / / / / / /  / /_/ /  / /__/ /_/ / / / / / /  __/ /_/ / / /__   / /_/ /  __/ |/ / 
/_____/_/   \__,_/_/ /_( )  /_/_/ /_/ /_/   \__,_/   \___/\____/_/ /_/ /_/\___/\__,_/_/\___/   \__,_/\___/|___/  
                       |/    
*/

//idk what to say vro at this line
fn main() -> Result<()> {
print!("\x1b[?1049h\x1b[44m\x1b[H\x1b[2J");
print!("Click Ctrl+A to exit or SIGKILL. This is a demo. No SIGINT");
stdout.flush()?;
enable_raw_mode()?;
let mut buffer = [0u8; 1];

loop {
stdin.read_exact(&mut buffer)?;
if  buffer[0] == 3 {
    continue;
}
if buffer[0] == 1 {
break;
}
}
disable_raw_mode()?;
print!("\x1b[0m\x1b[?1049l");
stdout.flush()?;
Ok(())
}