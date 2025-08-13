use utils::trpl;
use utils::trpl::{Either, Html};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    trpl::run(async {
        let url1 = if args.len() < 2 {"https://www.google.com"} else {&args[1]};
        let url2 = if args.len() < 3 {"https://www.douyin.com"} else {&args[2]};
        let title_fut_1 = page_title(url1);
        let title_fut_2 = page_title(url2);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}