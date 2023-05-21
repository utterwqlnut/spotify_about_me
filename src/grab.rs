use rspotify::{
    prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth,model::enums::misc::TimeRange::LongTerm,
};
#[derive(Debug)]
pub enum Grabber {
    Long{
        name: Option<String>,
        pfp: Option<String>,
        followers: Option<u32>,
        top3artists: Option<Vec<String>>,
        top3songs: Option<Vec<String>>,
        last_played: Option<String>
    },
    Short{
        name: Option<String>,
        pfp: Option<String>,
        followers: Option<u32>,
    },
}
#[tokio::main]
pub async fn get_user_data(long:bool) -> Grabber {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-currently-playing","user-top-read","user-read-recently-played")).unwrap();
    let spotify = AuthCodeSpotify::new(creds, oauth);
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).unwrap();
    match spotify.me() {
        Ok(info) => {
            //get display name
            let name = info.display_name;

            //get the pfp 
            let pfp = match info.images {

                Some(list) => {
                    if list.len() == 0 {
                        Some(String::from("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS3Prp8mclhZXHX1yhnjQybmT7d0HnSFv7zSijfKjtpvQ&usqp=CAU&ec=48665701"))
                    } else {
                        Some(list[0].url.clone())
                    }   
                },
                None => {
                    Some(String::from("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS3Prp8mclhZXHX1yhnjQybmT7d0HnSFv7zSijfKjtpvQ&usqp=CAU&ec=48665701"))
                }
            };

            //get follower count
            let followers: Option<u32> = if let Some(total) = info.followers { 
                Some(total.total)
            } else {
                None
            };

            if long {
                let result = spotify.current_user_top_artists(Some(LongTerm));
                
                let mut ctr=0;
                let mut top3artists: Vec<String> = Vec::new();
                for item in result {
                    if ctr == 3 {
                        break;
                    }
                    top3artists.push(item.unwrap().name);
                    ctr+=1;
                }

                let result = spotify.current_user_top_tracks(Some(LongTerm));
                let mut ctr=0;
                let mut top3songs: Vec<String> = Vec::new();
                for item in result {
                    if ctr == 3 {
                        break;
                    }
                    top3songs.push(item.unwrap().name);
                    ctr+=1;
                }

                let result = spotify.current_user_recently_played(Some(1),None);

                Grabber::Long {
                    name: name,
                    pfp: pfp, 
                    followers: followers,
                    top3artists: Some(top3artists),
                    top3songs: Some(top3songs), 
                    last_played: Some(result.unwrap().items[0].track.name.clone()), 

                }
            } else {
                Grabber::Short{name: name, pfp: pfp, followers: followers}
            } 
        },
        Err(_) => {
            panic!("could not get data from spotify");
        }
    }
}
