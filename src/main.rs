use chrono::NaiveDate;

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
    let base_url: &str =
        "https://www.lex18.com/search?q=abortion%20AND%20ballot%20AND%20Kentucky&p=";

    const num_of_sites: usize = 11;

    let urls: [&str; num_of_sites] = [
        "https://www.fox17online.com/search?q=abortion&p=",
        "https://www.fox47news.com/search?q=abortion&p=",
        "https://www.ktvq.com/search?q=abortion&p=",
        "https://www.kpax.com/search?q=abortion&p=",
        "https://www.krtv.com/search?q=abortion&p=",
        "https://www.kbzk.com/search?q=abortion&p=",
        "https://www.kxlf.com/search?q=abortion&p=",
        "https://www.wcpo.com/search?q=abortion&p=",
        "https://www.news5cleveland.com/search?q=abortion&p=",
        "https://www.wxyz.com/search?q=abortion&p=",
        "https://www.lex18.com/search?q=abortion&p="
    ];

    let filenames: [&str; num_of_sites] = [
        "fox17online", 
    "fox47", 
    "ktvq", 
    "kpax", 
    "krtv", 
    "kbzk", 
    "kxlf", 
    "wcpo", 
    "news5cleveland",
    "wxyz",
    "lex18"];

    let max_values: [i32; num_of_sites] = [
        198, 
        196, 
        176, 
        148, 
        139, 
        185, 
        147, 
        248, 
        236,
        222,
        175];

    let mut site_index = 9;

    while site_index < num_of_sites {
        println!("{}", urls[site_index]);
        println!("{}", filenames[site_index]);
        println!("{}", max_values[site_index]);

        let mut binding = &format!("{}.csv", filenames[site_index]).clone();
        let mut path = std::path::Path::new(binding);
        let mut writer = csv::Writer::from_path(path).unwrap();

        // append the header to the CSV
        writer
            .write_record(&["url", "title", "author", "date"])
            .unwrap();

        // current iteration - to turn this into a terminal argument, maybe
        let mut i = 1;

        // max number of iterations allowed
        let mut max_iterations = max_values[site_index];

        while i <= max_iterations {
            // download the target HTML document
            let mut url = &format!("{}", urls[site_index]).clone();
            println!("{}", format!("{}{}", url, i));

            let news_response = reqwest::blocking::get(format!("{}{}", url, i));
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
                    "%Y-%m-%d",
                ) else {
                    todo!()
                };

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

        site_index += 1;
    }
}
