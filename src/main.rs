fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = hoarder::Command::new(args);
    hoarder::run(command)
}
