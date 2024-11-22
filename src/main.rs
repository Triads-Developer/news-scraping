use chrono::{NaiveDate};

//struct is required to hold the object written to the csv
struct ListItem {
    url: Option<String>,
    title: Option<String>,
    author: Option<String>,
    date: Option<String>,
}

//I believe I wrote this for either debugging or maybe it's require to write to the csv
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
    //let base_url: &str = "https://www.kshb.com/search?q=abortion%20AND%20ballot%20AND%20Kansas&p=";
    //let base_url: &str = "https://www.wxyz.com/search?q=abortion%20AND%20ballot%20AND%20Michigan&p=";
    let base_url: &str = "https://www.lex18.com/search?q=abortion%20AND%20ballot%20AND%20Kentucky&p=";

    // current iteration - to turn this into a terminal argument, maybe
    let mut i = 1;

    // max number of iterations allowed
    let max_iterations = 11849;

    //Rust is really particular about who owns a variable
    //so we format the string, clone it, and then send the pointer to path and writer
    let binding = &format!("lex18-{}.csv",i).clone();
    let path = std::path::Path::new(binding);
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

        let html_content = news_response.unwrap().text().unwrap();

        let document = scraper::Html::parse_document(&html_content);

        let html_list_item_selector = scraper::Selector::parse(".ListItem ").unwrap();
        let list_items = document.select(&html_list_item_selector);

        //loop to get the items in the page
        for list_item in list_items {
            let url = list_item.value().attr("href").map(str::to_owned);

            let Ok((raw_date, _reminder)) = NaiveDate::parse_and_remainder(
                list_item
                    .select(&scraper::Selector::parse(".ListItem-date").unwrap())
                    .next()
                    .unwrap()
                    .attr("data-timestamp")
                    .unwrap(),
                "%Y-%m-%d"
            ) else { todo!() } ;

            let date: Option<String> = Some(raw_date.to_string());

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
