use std::io::{self, Read};
use std::io::prelude::*;
use std::vec;



fn main() {
    if let Err(err) = try_main() {
            if err.kind() == std::io::ErrorKind::BrokenPipe {
                return;
            }
            // Ignore any error that may occur while writing to stderr.
            let _ = writeln!(std::io::stderr(), "{}", err);
        }
}


fn try_main() -> Result<(), std::io::Error>{
    let stdin = io::stdin();
    let mut stdin = stdin.lock(); // locking is optional

    let mut lines = String::new();

    // Could also `match` on the `Result` if you wanted to handle `Err`
    // initialisation of flag
    let mut previous_id: String = "INITIALISATION_UNASSIGNED".to_string();
    let mut current_id: String = "INITIALISATION_UNASSIGNED_CURRENT".to_string();
    let mut current_position: u32;
    let mut previous_position: u32 = 0 ;
    let mut previous_depth_value: u32 = 0;
    let mut current_depth_value: u32;
    let mut previous_line: Vec<String> = Vec::new();
    let mut owned_line: String;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut results : Vec<String>;

    // loop
    // where to perform first initialisation?
    // use flag
    let mut first_line = true;

    while let Ok(n_bytes) = stdin.read_to_string(&mut lines) {
        if n_bytes == 0{
            break }

        let splt_lines: Vec<&str> = lines.split("\n").collect();
        for line in splt_lines{
            owned_line = line.to_string();

            let spt: Vec<String> = Vec::from_iter(owned_line.split("\t")
                                        .map(String::from));




            if owned_line == "\n".to_string() || line.is_empty() || spt.len() < 7{
                    results = vec![previous_line[0].to_string(), previous_line[1].to_string(),
                     previous_line[2].to_string(), previous_line[3].to_string(),
                     previous_line[4].to_string(), previous_line[5].to_string()];//..6];
                     results.push(previous_position.to_string());
                     results.push((previous_line[6].parse::<u32>().unwrap()).to_string());
                     results.push((previous_depth_value).to_string());

                     writeln!(stdout, "{}", results.join("\t"));


                return Ok(())
            }

            current_id = spt[3].to_string();
            current_depth_value = spt[7].parse::<u32>().unwrap();
            current_position = spt[6].parse::<u32>().unwrap();

            if first_line{ // don't like that
                first_line = false;
                previous_line = spt.clone();
                previous_id = current_id.to_string();
                previous_depth_value = current_depth_value;
                previous_position = current_position;
            }
            else {

                 if current_id == previous_id && current_depth_value != previous_depth_value {
                     // write
                     //same ID Gene
                     results = vec![spt[0].to_string(), spt[1].to_string(),
                     spt[2].to_string(), spt[3].to_string(),
                     spt[4].to_string(), spt[5].to_string()];
                     results.push(previous_position.to_string());
                     results.push((current_position-1).to_string());
                     results.push((previous_depth_value).to_string());

                     writeln!(stdout, "{}", results.join("\t"));

                     previous_line = spt.clone();
                     previous_id = current_id.to_string();
                     previous_depth_value = current_depth_value;
                     previous_position = current_position;

                 }

                else if current_id != previous_id {

                     results = vec![previous_line[0].to_string(), previous_line[1].to_string(),
                     previous_line[2].to_string(), previous_line[3].to_string(),
                     previous_line[4].to_string(), previous_line[5].to_string()];

                     results.push((previous_position).to_string());
                     results.push((previous_line[6].parse::<u32>().unwrap()).to_string());
                     results.push((previous_depth_value).to_string());

                     writeln!(stdout, "{}", results.join("\t"));

                     previous_line = spt.clone();
                     previous_id = current_id.to_string();
                     previous_depth_value = current_depth_value;
                     previous_position = current_position;
                    }
                else{
                    previous_line = spt.clone();

                    }

                }

            }
                lines.clear();
        }
        Ok(())
    }



// Some easy looking algo yipee!


