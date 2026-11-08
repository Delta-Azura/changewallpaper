use wallpaper; 
use iced::widget::pick_list;
use iced::widget::{button, container, row, scrollable, text, text_input};
use std:fs;
use std::path::Path;



struct App {
    path: String,
    hourone: chrono::NaiveTime; 
    hourtwo: chrono::NaiveTime;
}


#[derive(Debug, Clone)]
pub enum Message {
    ChooseWallpaper,
    WallpaperPath(String),
    SetHourOne(chrono::NaiveTime),
    SetHourTwo(chrono::NaiveTime),
}


impl App {
    pub fn new() -> (Self, Task<Message>) {
        let app = Self {
            path: String::new(),
            hourone: NaiveTime::default(),
            hourtwo: NaiveTime::default(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChooseWallpaper => {
                let Some(path) = rfs::FileDialog::new()
                    .pick_file()
            }
            Message::SetHourOne(v) => {
                self.hourone = v,
            }

            Message::SetHourTwo(v) => {
                self.hourtwo = v,
            }
        }
    }

}


fn main() {
    println!("Hello, world!");
    wallpaper::set_from_path(wallpaper);
}


