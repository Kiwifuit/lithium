mod db;
mod err;
mod path;

fn main() {
    let iter = match path::get_all_profiles() {
        Ok(i) => i,
        Err(e) => panic!("An error occurred while getting profiles: {}", e),
    };

    for i in iter {
        println!("Path {}", i.display())
    }
}
