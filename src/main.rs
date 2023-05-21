use crate::grab::*;
use std::fs;
use std::io;
use webbrowser;
use minijinja::{Environment, context};
pub mod grab;

fn main () {
    println!("Would you like a long or short about me (1 for long 2 for short)");

    let mut which = String::new();

    io::stdin().read_line(&mut which).expect("Failed to read line");

    let which: u32 = which.trim().parse().expect("Please type a number");
    
    let result = if which == 1 as u32 {
        grab::get_user_data(true)
    } else {
        grab::get_user_data(false)
    };

    let mut env = Environment::new();

    match result {
        Grabber::Long{name, pfp, followers, top3artists, top3songs, last_played} => {
        env.add_template("template",include_str!("template.html")).unwrap();

        let tmpl = env.get_template("template").unwrap();
            fs::write("./about_me.html", tmpl.render(context!(user => name.unwrap(), img => pfp.unwrap(), 
                                                recent => last_played.unwrap(), artists => top3artists.unwrap(),
                                                songs => top3songs.unwrap(), followers => followers.unwrap()))
                      .unwrap()).expect("issue writing to file");

        },
        Grabber::Short{name, pfp, followers} => {
            env.add_template("template",include_str!("template2.html")).unwrap();
            let tmpl = env.get_template("template").unwrap();

            fs::write("./about_me.html", tmpl.render(context!(user => name.unwrap(), img => pfp.unwrap(), 
                                                              followers => followers.unwrap()))
                      .unwrap()).expect("issues writing to file");
        },
    }

    if !webbrowser::open("about_me.html").is_ok() {
        panic!("could not open html file");
    }

}
