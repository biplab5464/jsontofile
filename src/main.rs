use clap::Parser;
use json::{stringify, stringify_pretty, JsonValue};
use std::fs::{self, read_to_string, File};
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "The tool is used to convert a JSON array of objects to individual JSON files"
)]
struct Command {
    /// The file must be a json array of josn object
    file: String,
    ///print in formated form in json, if not provided it print in compact from
    #[arg(long)]
    pretty: bool,
    ///no print to the stdout
    #[arg(long)]
    quite: bool,
    ///Output directory for the generated JSON files (defaults to ./temp directory)
    #[arg(long, short)]
    output: Option<String>,
}

fn main() {
    let args = Command::parse();

    //println!("got it  = {:?}", args.file);
    let file = read_to_string(args.file).expect("Unable to Read the file");

    let json: JsonValue =
        json::parse(&file).expect("Unable to read json,\n Please check th json file");

    let output_path = match args.output {
        None => {
            if !Path::new("./temp").exists() {
                fs::create_dir_all("./temp").expect("unable to create temp dir");
            }
            "./temp".to_string()
        }
        Some(tmp_path) => tmp_path,
    };

    for obj in json.members() {
        let id = obj["id"].as_str().unwrap();
        let template_name = obj["name"].as_str().unwrap();

        let path = format!("{output_path}/{id}_{template_name}.json");
        let name = Path::new(&path);

        let obj_string = match args.pretty {
            false => stringify(obj.clone()),
            true => stringify_pretty(obj.clone(), 2),
        };

        let mut write_file =
            File::create(name).expect("problem with opening the file maybe present before");
        write_file
            .write_all(obj_string.as_bytes())
            .expect("Probem with writeing the file");

        if !args.quite {
            println!("saved {}", name.display());
        }
    }
}
