struct ListItem {
    url: Option<String>,
    title: Option<String>,
    author: Option<String>,
    date: Option<String>,
}

impl std::fmt::Display for ListItem {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Title - {:?}, date - {:?} url - {:?} author - {:?}",
            self.title, self.date, self.url, self.author
        )
    }
}

fn main() {
    let base_url: &str = "https://www.kshb.com/search?q=abortion%20AND%20ballot%20AND%20Kansas&p=";

    // current iteration
    let mut i = 1;
    // max number of iterations allowed
    let max_iterations = 18058; //is the total

    let path = std::path::Path::new("kshb.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // append the header to the CSV
    writer
        .write_record(&["url", "title", "author", "date"])
        .unwrap();

    while i <= max_iterations {
        // download the target HTML document
        println!("{}", format!("{base_url}{i}"));

        let news_response = reqwest::blocking::get(format!("{base_url}{i}"));
        //get the HTML content from the request response
        //and print it
        //
        let html_content = news_response.unwrap().text().unwrap();

        let document = scraper::Html::parse_document(&html_content);

        let html_list_item_selector = scraper::Selector::parse(".ListItem ").unwrap();
        let list_items = document.select(&html_list_item_selector);

        for list_item in list_items {
            let url = list_item.value().attr("href").map(str::to_owned);

            let date = list_item
                .select(&scraper::Selector::parse(".ListItem-date").unwrap())
                .next()
                .map(|date| date.text().collect::<String>());

            let author_raw = list_item
                .select(&scraper::Selector::parse(".ListItem-author").unwrap())
                .next()
                .map(|author| author.text().collect::<String>());

            let author_str = author_raw
                .as_ref()
                .map_or("default string", |x| &**x)
                .trim();

            let author: Option<String> = Some(author_str.to_string());

            let title_raw = list_item
                .select(&scraper::Selector::parse(".ListItem-title").unwrap())
                .next()
                .map(|title| title.text().collect::<String>());

            let title_str = title_raw.as_ref().map_or("title", |x| &**x);

            let title: Option<String> = Some(title_str.to_string());

            let item = ListItem {
                url,
                title,
                author,
                date,
            };

            // create the CSV output file
            // populate the output file
            let url = item.url.unwrap();
            let title = item.title.unwrap();
            let author = item.author.unwrap();
            let date = item.date.unwrap();

            writer.write_record(&[url, title, author, date]).unwrap();
        }
        i += 1;

        // free up the resources
        writer.flush().unwrap();
    }
}
