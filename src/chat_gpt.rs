struct Filename{
    file_name : String,
    ids : Vec<String>
}

impl Filename {
    fn new(input : String ) -> Filename{
        let re = Regex::new(r"\{([^}]*)\}").unwrap();
        let mut result = VecDeque::new();
    
        for cap in re.captures_iter(input) {
            result.push_back(String::from(&cap[1]));
        }
    
        result
    }
    
    fn get_file_name( &self, json : &JsonValue) -> String {
        let mut return_str = self.file_name.clone();
        for ele in self.ids.iter(){
            let temp = json[ele].as_str().expect("Something wrong with the string given with the --filename or -f or the var not avaliable");
            return_str = return_str.replace(&format!("{{{}}}", ele), temp);
        }
        return_str
    }
}