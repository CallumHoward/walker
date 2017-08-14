// main.rs
// Callum Howard 2017

extern crate ignore;


fn main() {
    use ignore::{WalkBuilder, WalkState};

    WalkBuilder::new("./").build_parallel().run(|| {
        Box::new(move |result| {
            let dent = match result {
                Err(_) => return WalkState::Continue,
                Ok(dent) => dent,
            };

            let ft = match dent.file_type() {
                None => return WalkState::Continue,
                Some(ft) => ft,
            };

            if ft.is_dir() {
                let path = match dent.path().strip_prefix("./") {
                    Err(_) => dent.path(),
                    Ok(path) => path,
                };
                println!("{}", path.display());
            }

            WalkState::Continue
        })
    });
}
