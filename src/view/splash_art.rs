

pub struct SplashArt {
    pub splash_art: String,
}

impl SplashArt {
    pub fn default()->SplashArt{
        let splash_art =  r#"
                   ,__
                   |  `'.
__           |`-._/_.:---`-.._
\='.       _/..--'`__         `'-._
 \- '-.--"`      ===        /   o  `',
  )= (                 .--_ |       _.'
 /_=.'-._             {=_-_ |   .--`-.
/_.'    `\`'-._        '-=   \    _.'
         )  _.-'`'-..       _..-'`
        /_.'        `/";';`|
                     \` .'/
                      '--'
        "#.to_string();
        Self{
            splash_art
        }
    }
}