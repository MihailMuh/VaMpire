use vampire_file::VaMpireFile;

mod vampire_file;

fn main() {
    let vampire_file = VaMpireFile::get("Vampirefile");

    println!("{}", vampire_file.to_string());
}
