use rekoto::read;

fn main() {
    match read::file::read_file() {
        Ok(()) => {}

        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
