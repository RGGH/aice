/// main.rs
/// Demonstration using async function (reqwest)
/// with a Task
///
/// Learning Rust
///
use iced::Settings;
use iced::Task;
use iced::Theme;
use serde::Deserialize;

#[derive(Deserialize)]
struct CoinGeckoResponse {
    bitcoin: Bitcoin,
}

#[derive(Deserialize)]
struct Bitcoin {
    usd: f64,
}

#[derive(Debug, Clone)]
enum Message {
    Refetch,
    CurrentIp(String),
}

#[derive(Default)]
struct App {
    ip: String,
}

impl App {
    fn view(&self) -> iced::Element<Message> {
        let content = iced::widget::column![
            iced::widget::text(&self.ip),
            iced::widget::button("Fetch Current Price in USD").on_press(Message::Refetch)
        ]
        .width(iced::Fill)
        .spacing(10)
        .padding(20)
        .align_x(iced::Alignment::Center)
        .into();

        content
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        println!("update");
        match message {
            Message::Refetch => return Task::perform(fetch_ip(), Message::CurrentIp),
            Message::CurrentIp(text) => {
                self.ip = text;
            }
        }
        Task::none()
    }
}

async fn fetch_ip() -> String {
    println!("fetch_ip");
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
    let response = reqwest::get(url)
        .await
        .unwrap()
        .json::<CoinGeckoResponse>() // Deserialize JSON into struct
        .await
        .unwrap();

    let res = response;
    format!("Price {:.2}", res.bitcoin.usd)
}

fn theme(_: &App) -> Theme {
    Theme::Nightfly
}

fn main() {
    let _ = iced::application("Get latest Bitcoin price", App::update, App::view)
        .theme(theme)
        .settings(Settings::default())
        .run();
}
