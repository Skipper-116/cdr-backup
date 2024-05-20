extern crate yaml_rust;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    // print the database configuration
    let config = read_database_yml();
    println!("{:?}", config);
}

// We want a function that will read a database.yml file and return a struct
// representing the configuration.

#[derive(Debug)]

#[warn(dead_code)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

fn read_database_yml() -> DatabaseConfig {
    // Read the YAML file
    let mut file = File::open("/Users/roychanunkha/Code/Mthandizi-Monitoring-Tool-API/config/database.yml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the YAML file
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0]; // get the first document

    // Access the development database configuration
    let host = doc["default"]["host"].as_str().unwrap();
    let port = doc["default"]["port"].as_i64().unwrap() as u16;
    let development = &doc["development"]["primary"];
    let database = development["database"].as_str().unwrap();
    let username = development["username"].as_str().unwrap();
    let password = development["password"].as_str().unwrap();

    DatabaseConfig {
        host: host.to_string(),
        port: port,
        username: username.to_string(),
        password: password.to_string(),
        database: database.to_string(),
    }
}

