mod io;

fn main() {
    let side = io::ask("Black (b) or white (w)?", &["b", "w"]);

    println!("You have choosen {:?} side", side);
}
