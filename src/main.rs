/// Originally "jsontofile" by biplab5464

use clap::Parser;
use json::{stringify, stringify_pretty, JsonValue};
use std::fs::{self, read_to_string, File};
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "This command line tool is used to convert a single JSON file containing array of objects to seperate JSON files, each containing one array object. All output files are stored in one folder."
)]

struct Command {
    /// The file must be a JSON array
    file: String,
    /// Custom filename for output JSON files
    #[arg(long,short)]
    filename: Option<String>,
    /// Output directory for the generated JSON files (defaults to ./temp directory)
    #[arg(long, short)]
    output: Option<String>,
    /// If provided print in compacted form, else print in pretty from
    #[arg(long)]
    compact: bool,
    /// Don't print to stdout
    #[arg(long)]
    quite: bool,
    /// Number of spaces for JSON in pretty form (only works in pretty form)
    #[arg(long="space",short='s')]
    json_space : Option<u16>,
    //remove _id from JSON
    #[arg(long = "rmdashid")]
    remove_id : bool
}

struct Filename {
    file_name : String,
    ids : Vec<String>
}

impl Filename {
    fn new(file_name : String ) -> Filename{
        let mut ids = Vec::new();
        let mut start = 0;
        let mut end = 0;
        let mut inside = false;

        for (i, c) in file_name.chars().enumerate() {
            if c == '{' {
                start = i + 1;
                inside = true;
            } else if c == '}' {
                end = i;
                if inside {
                    ids.push(file_name[start..end].to_string());
                    inside = false;
                }
            }
        }
    
        Filename {
            ids,
            file_name
        }
    }
    
    fn get_file_name( &self, json : &JsonValue) -> String {
        let mut return_str = self.file_name.clone();
        for ele in self.ids.iter() {
            let temp = json[ele].as_str().expect("Element not present in specified JSON file");
            return_str = return_str.replace(&format!("{{{}}}", ele), temp);
        }
        return_str
    }
}

fn main() {
    let args = Command::parse();

    //println!("got it  = {:?}", args.file);
    let file = read_to_string(args.file).expect("Unable to read the specified file. Please input the correct filename.");

    let mut json: JsonValue =
        json::parse(&file).expect("Unable to read JSON, please check the JSON file.");

    let output_path = match args.output {
        None => {
            if !Path::new("./temp").exists() {
                fs::create_dir_all("./temp").expect("Unable to create temp directory.");
            }
            "./temp".to_string()
        }
        Some(tmp_path) => tmp_path,
    };

    let file_name = match args.filename {
        None => Filename::new("{id}_{name}.json".to_string()),
        Some(tmp_str) => Filename::new(tmp_str)
    };

    let spaces = match args.json_space {
        None => 2,
        Some(tmp ) => tmp
    };

    for obj in json.members_mut() {
        let path = format!("{output_path}/{}",file_name.get_file_name(obj));
        let name = Path::new(&path);

        if args.remove_id {
            obj.remove("_id");
        }

        let obj_string = match args.compact {
            true => stringify(obj.clone()),
            false => stringify_pretty(obj.clone(), spaces),
        };

        let mut write_file =
            File::create(name).expect("Error in creating the new file. Permission may not be available.");
        write_file
            .write_all(obj_string.as_bytes())
            .expect("Problem with writing the file");

        if !args.quite {
            println!("saved {}", name.display());
        }
    }
}
