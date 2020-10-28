use std::{fs, env};

pub fn delete(mut targets: Vec<String>) -> Result<(), ()> {
    if targets.len() > 1 {
        targets.remove(0);
    } else {
        println!("\nOops... it appears that you forgot to supply any arguments.\n\nThe proper format for calling this utility is 'rmdear <directory(s)>'.");
        return Err(()) // I've already done an error message. Also, for some reason Rust won't allow me to return anything else :)
    }
    
    let absolute_path: String = match std::env::current_dir() {
        Ok(result) => result.to_str().unwrap().to_string(),
        Err(_) => {
            println!("For some reason, this program cannot access the absolute path.");
            return Err(())
        },
    };

    println!("{:?}", absolute_path);
    println!("{:?}", targets);

    for mut raw_target in targets {

        #[allow(unused_variables)]
        let target: String = match String::from(raw_target.chars().nth(0).unwrap()).as_str() {
            "." => {
                raw_target.remove(0);
                raw_target + absolute_path.as_str()
            },
            "/" | "~" => raw_target, // User has specified exact location
            _ => {
                absolute_path.to_string() + "/" + raw_target.as_str() // TODO: absolute_path + / + raw_target
            }
        };

        println!("{:?}", target);

        
        match fs::remove_dir_all(&target) {
            Ok(()) => {},
            Err(_error) => println!("Unfortunately, it appears that the directory {} does not exist.", target),
        }
    }

    Ok(())
}