use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let _f = File::open("hello.txt")?;

    Ok(())
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     //SHORTEST WAY
//     fs::read_to_string("hello.txt")

//     // SHORTER WAY
//     // let mut s = String::new();
//     // File::open("helloo.txt")?.read_to_string(&mut s)?;
//     // Ok(s)

//     // LONGEST WAY
//     // let f = File::open("hello.txt");

//     // let mut f = match f {
//     //     Ok(file) => file,
//     //     Err(e) => return Err(e),
//     // };

//     // let mut s = String::new();

//     // match f.read_to_string(&mut s) {
//     //     Ok(_) => Ok(s),
//     //     Err(e) => Err(e),
//     // }
// }
