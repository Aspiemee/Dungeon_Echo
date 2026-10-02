mod location;
mod game;


fn main() {
    let descr = location::get_description(&game::Location::DarkCave);
    println!("{}", descr);
}
