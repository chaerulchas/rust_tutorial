

use futures::StreamExt;

use chromiumoxide::browser::{Browser, BrowserConfig};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
   // create a `Browser` that spawns a `chromium` process running with UI (`with_head()`, headless is default) 
   // and the handler that drives the websocket etc.
    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;
    
   // spawn a new task that continuously polls the handler
    let handle = async_std::task::spawn(async move {
        loop {
           let _ = handler.next().await.unwrap();
        }
    });
    
   // create a new browser page and navigate to the url
    let page = browser.new_page("https://en.wikipedia.org").await?;
    
   // find the search bar type into the search field and hit `Enter`,
   // this triggers a new navigation to the search result page
   page.find_element("input#searchInput")
           .await?
           .click()
           .await?
           .type_str("Rust programming language")
           .await?
           .press_key("Enter")
           .await?;

   let html = page.wait_for_navigation().await?.content().await?;
   
    handle.await;
    Ok(())
}