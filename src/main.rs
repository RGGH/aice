use iced::widget::Theme;
use iced::window::Position;
use iced::{window, Size};
use iced::{Point, Task};
use serde::Deserialize;

#[derive(Deserialize)]
struct CoinGeckoResponse {
    bitcoin: Bitcoin,
}

#[derive(Deserialize, Clone, Debug)]
struct Bitcoin {
    usd: f64,
    gbp: f64,
}

#[derive(Debug, Clone)]
enum Message {
    Refetch,
    CurrentPrice((f64, f64)), // Use a single tuple here
}

#[derive(Default)]
struct App {
    price_usd: f64,
    price_gbp: f64,
}

impl App {
    fn new() -> Self {
        Self {
            price_usd: 0.0,
            price_gbp: 0.0,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let content = iced::widget::column![
            iced::widget::text(format!("{:.2} USD", self.price_usd)),
            iced::widget::button("Fetch Current Price").on_press(Message::Refetch),
            iced::widget::text(format!("{:.2} GBP", self.price_gbp))
        ]
        .width(iced::Fill)
        .spacing(10)
        .padding(20)
        .align_x(iced::Alignment::Center)
        .into();

        content
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Refetch => {
                // Use a closure to match the output tuple
                return Task::perform(fetch_btc(), |(usd, gbp)| Message::CurrentPrice((usd, gbp)))
            } 
            Message::CurrentPrice((usd, gbp)) => {
                self.price_usd = usd;
                self.price_gbp = gbp;
            }
        }
        Task::none()
    }
}

async fn fetch_btc() -> (f64, f64) {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd,gbp";
    let response = reqwest::get(url)
        .await
        .unwrap()
        .json::<CoinGeckoResponse>()
        .await
        .unwrap();

    (response.bitcoin.usd, response.bitcoin.gbp)
}

fn theme(_: &App) -> Theme {
    Theme::Nord
}

fn main() -> Result<(), iced::Error> {
    iced::application("Get Latest Bitcoin Price", App::update, App::view)
        .window(window::Settings {
            position: Position::Specific(Point::new(1400.0, 200.0)),
            resizable: false,
            size: Size::new(400.0, 400.0),
            ..Default::default()
        })
        .theme(theme)
        .run_with(|| (App::new(), iced::Task::none()))
}
