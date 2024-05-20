extern crate yaml_rust;
mod models;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::Read;
use crate::models::DatabaseConfig;

fn main() {
    println!("Hello, world!");
    // print the database configuration
    let config = read_database_yml();
    println!("{}", config.to_string());
}

fn read_api_service_file() -> String {
    // Read .service file
    let contents = read_file_contents("/path/to_file".to_string());
    // check if the file contains the word "production"
    if contents.contains("production") {
        "production".to_string()
    } else {
        "development".to_string()
    }
}

fn read_database_yml() -> DatabaseConfig {
    // Read the YAML file
    let contents = read_file_contents("/path/to_file".to_string());
    let env = read_api_service_file();

    // Parse the YAML file
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0]; // get the first document

    // Access the development database configuration
    let host = doc["default"]["host"].as_str().unwrap();
    let database = doc[env.to_string()]["database"].as_str().unwrap();
    let username = doc["default"]["username"].as_str().unwrap();
    let password = doc["default"]["password"].as_str().unwrap();
    let port = doc["default"]["port"].as_i64().unwrap_or(3306) as u16;
    
    DatabaseConfig {
        host: host.to_string(),
        port: port,
        username: username.to_string(),
        password: password.to_string(),
        database: database.to_string(),
    }
}

fn read_file_contents(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

// function that will receive the database struct and execute a backup command
fn backup_database(config: DatabaseConfig, facility_name: String) -> String {
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let backup_file = format!("openmrs_{}_{}.sql", facility_name, current_date);
    // execute mysqldump command
    let command = format!(
        "mysqldump -h {} -P {} -u {} -p{} {} > {}.sql",
        config.host, config.port, config.username, config.password, config.database, backup_file
    );
    // execute the command
    std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("failed to execute process");
    backup_file
}

// function that will compress the backup file using gzip highest compression
fn compress_backup_file(backup_file: String) -> String {
    let compressed_file = format!("{}.gz", backup_file);
    // execute the command
    std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("gzip -9 {}", backup_file))
        .output()
        .expect("failed to execute process");
    compressed_file
}

// function that will log the backup size in json format hash
fn log_backup_size(compressed_file: String) {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("ls -lh {}", compressed_file))
        .output()
        .expect("failed to execute process");
    let size = String::from_utf8_lossy(&output.stdout).to_string();
    let size = size.split_whitespace().collect::<Vec<&str>>()[4];
    let log = format!("{{ \"{}\": \"{}\" }}",chrono::Local::now().format("%Y-%m-%d").to_string(), size);
    println!("{}", log);
}

// function that will verify the size of the backup file based on previous backups


// function to execute a query in the database and return the result
fn execute_query(config: DatabaseConfig, query: String) -> String {
    // execute the query
    let command = format!(
        "mysql -h {} -P {} -u {} -p{} {} -e \"{}\"",
        config.host, config.port, config.username, config.password, config.database, query
    );
    // execute the command
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("failed to execute process");
    // return the output
    String::from_utf8_lossy(&output.stdout).to_string()
}
