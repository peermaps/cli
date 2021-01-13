use peermaps_ingest;

fn main() -> std::result::Result<bool, Error> {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    if cmd == "ingest" {
        let pbf = &args[2];
        let output = &args[3];
        let db = &args[4];

        match peermaps_ingest::denormalize(pbf, output) {
            Ok(_) -> {
                peermaps_ingest::write_to_db(output, db)?;
            },
            Err(e) -> {
                eprintln!("Error during pbf denormalization {}", e);
            }
        };
            
        return Ok(());
    } else {
        println!("{} command not known.", cmd);
        return Ok(());
    }
}
