use sha256::digest;
use std::env;

struct Arguments<'a> {
    flag: &'a str,
    operation_type: &'a str,
    data: &'a [String],
    data_modifyied: String
}

// flag = --help / -t
// operation_type = 
// data = Any message

impl Arguments<'_> {
    fn new(args: &Vec<String>) -> Option<Arguments> {
        
        if args.len() < 2 {
            println!("Use --help to see how to manage");
            
            return None
        }
        
        let arguments = Arguments {
            flag: &args[1],
            operation_type: &args[2],
            data: &args[3..],
            data_modifyied: "".to_string()
        };

        if arguments.flag == "--help" {
            println!("Usage: -t to select the type of codification\n\rOperations types: sha256");
            
            return None
        }
        
        Some(arguments)

    }

    fn decode(&mut self) {
        match self.operation_type {
            "sha256" => {
                let formatted_data = self.data.join(" ");

                let digested = digest(formatted_data);

                self.data_modifyied = digested;
            },
            _ => println!("Invalid Operation!")
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut arguments = match Arguments::new(&args) {
        Some(args) => args,
        None => return
    };

    arguments.decode();

    println!("{}", arguments.data_modifyied)
    
}
