use std::io;
// use std::io::Write;
use regex::Regex;

fn main() -> io::Result<()> {
    let re = Regex::new(r"\[[^\]]+]: .+\s*$").unwrap();
    let more = "<!--more-->";
    let mut vec = Vec::new();

    // Iterate over our input from stdin
    let lines = io::stdin().lines();
    for line in lines {
        let s = line.unwrap();
        // println!("got a line: {}", s);

        if re.is_match(&s) {
            // If we find a link, save it for later.
            eprintln!("got a link {}", s);
            vec.push(s);
        } else {
            // If we find a header or <!--more-->, print
            // the found links before the line.
            let mut found = false;
            if s.trim() == more {
                found = true;
                eprintln!("line was more");
            } else if s.starts_with("#") {
                found = true;
                eprintln!("line was header");
            }
            if found && vec.len() > 0 {
                for link in &vec {
                    let _ = println!("{}", link);
                    eprintln!("output a link {}", link)
                }
                vec.clear();
                println!("")
            }

            let _ = println!("{}", s);
        }
    }

    // Print any remaining links on exit
    println!("");
    for link in &vec {
        let _ = println!("{}", link);
        eprintln!("output a link {}", link)
    }
    vec.clear();

    Ok(())
}
