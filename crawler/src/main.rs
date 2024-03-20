use reqwest::Result;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {
    let target_url = "https://github.com/trending/rust";

    let body = reqwest::get(target_url)
    .await
    .expect("请求地址失败")
    .text()
    .await
    .expect("解析网页内容失败");


    let document = Html::parse_document(body.as_str());

    let rows_selector = Selector::parse(".Box-row").unwrap();
    let repo_link_selector = Selector::parse("h2 a").unwrap();
    let repo_today_star_selector: Selector = Selector::parse("span.d-inline-block.float-sm-right").unwrap();

    for row in document.select(&rows_selector)
    {
        if let Some(repo_link) = row.select(&repo_link_selector).nth(0)
        {
            if let Some(href) = repo_link.value().attr("href")
            {
                print!("仓库链接: {href}");
            }
        }
        if let Some(today_star) = row.select(&repo_today_star_selector).nth(0)
        {
            let texts: Vec<_> = today_star.text().collect();
            let text = texts.join("").split_whitespace().nth(0).expect("获取今日star数失败").to_string();
            let text = text.replace(",", "");
            print!("今日star数: {text}"); // 这里好像不加分号也行

        }

        if let Some(total_star) = row.select(&repo_today_star_selector).nth(0)
        {
            let texts: Vec<_> = total_star.text().collect();
            let text = texts.join("").split_whitespace().nth(0).expect("获取总star数失败").to_string();
            let text = text.replace(",", "");
            print!("总star数: {text}")// 这里好像不加分号也行
        }
        println!("")

    }
    
    Ok(()) //什么作用？


}