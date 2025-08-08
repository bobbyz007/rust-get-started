use std::future::Future;
use utils::trpl;
use utils::trpl::Html;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    trpl::run(async {
        let url = if args.len() < 2 {"https://www.baidu.com"} else {&args[1]};
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("There was no title for {url}"),
        }
    })
}

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text).select_first("title")
            .map(|title| title.inner_html())
    }
}