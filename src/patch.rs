use regex::{Captures, Regex};
use std::fs;
use std::path::PathBuf;

pub struct Patch {
    pub regex: Regex,
    pub handler: Box<dyn Fn(&Captures) -> Option<(String, String)>>,
}

mod tdp_slider;

pub async fn patch(chunk: PathBuf) {
    let patch_item = tdp_slider::get_patch();

    let mut content = fs::read_to_string(&chunk).unwrap();

    if let Some(captures) = patch_item.regex.captures(&content) {
        let patch = (patch_item.handler)(&captures);

        if let Some((from, to)) = patch {
            content = content.replace(&from, &to);
        } else {
            println!("Couldn't patch - text entry not found");
        }

        fs::write(chunk, content).unwrap();
    } else {
        println!("Couldn't patch - text entry not found");
    }
}
