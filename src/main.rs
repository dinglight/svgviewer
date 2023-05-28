use std::{
    fs,
    path::{Path, PathBuf},
};

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Column, Svg},
    Element, Length, Sandbox, Settings,
};

fn main() -> iced::Result {
    SvgViewer::run(Settings::default())
}

#[derive(Debug)]
struct SvgViewer {
    folder_path: String,
    file_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TextInputChanged(String),
    ButtonClicked,
}

impl Sandbox for SvgViewer {
    type Message = Message;

    fn new() -> Self {
        SvgViewer::new()
    }

    fn title(&self) -> String {
        String::from("SVG Viewer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonClicked => {
                self.collect_svg_files();
            }
            Message::TextInputChanged(text) => {
                self.folder_path = text;
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content;
        if self.file_paths.is_empty() {
            content = container(text("no svg files").size(50)).width(Length::Fill).center_x();
        } else {
            let svgs: Vec<Element<_>> = self
                .file_paths
                .iter()
                .map(|path| Svg::from_path(path).into())
                .collect();
            content = container(
                Column::with_children(svgs)
                    .spacing(20)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .width(Length::Fill)
            .center_x();
        }

        container(column![
            row![
                text_input("placeholder", &self.folder_path).on_input(Message::TextInputChanged),
                button(text("refresh")).on_press(Message::ButtonClicked)
            ]
            .spacing(10),
            scrollable(content),
        ])
        .height(Length::Fill)
        .padding(10)
        .into()
    }
}

impl SvgViewer {
    fn new() -> Self {
        SvgViewer {
            folder_path: String::from(""),
            file_paths: vec![],
        }
    }

    fn collect_svg_files(&mut self) {
        let path = Path::new(&self.folder_path);
        self.file_paths.clear();
        if path.is_dir() {
            let files = fs::read_dir(path).unwrap();
            files
                .filter_map(Result::ok)
                .filter(|d| {
                    if let Some(e) = d.path().extension() {
                        e == "svg"
                    } else {
                        false
                    }
                })
                .for_each(|f| self.file_paths.push(f.path()));
        } else {
            println!("{} not dir", &self.folder_path);
        }
    }
}
