use tera::Tera;


pub fn get_engine() -> Tera {
    println!("init Tera engine");
    let tera = match Tera::new("./**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Tera parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera
}