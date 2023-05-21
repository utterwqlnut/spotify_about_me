use crate::grab::*;
use std::fs;
use std::process::Command;
use minijinja::{Environment, context};
pub mod grab;

fn main () {
    let result = grab::get_user_data(true);
    let mut env = Environment::new();
    env.add_template("template",include_str!("template.html")).unwrap();
    let tmpl = env.get_template("template").unwrap();

    match result {
        Grabber::Long{name, pfp, followers, top3artists, top3songs, last_played} => {
            fs::write("./about_me.html", tmpl.render(context!(user => name.unwrap(), img => pfp.unwrap(), 
                                                recent => last_played.unwrap(), artists => top3artists.unwrap(),
                                                songs => top3songs.unwrap(), followers => followers.unwrap()))
                      .unwrap()).expect("issue writing to file");

        },
        Grabber::Short{..} => println!("nah"),
    }
    let _command = Command::new("zsh").arg("-c").arg("open about_me.html").spawn().expect("issue opening file");
}
