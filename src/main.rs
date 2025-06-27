use trpl::{Either, Html};

fn main() {
    let args : Vec<String> = std::env::args().collect();
    trpl::run(async {
    let titel_text1 = page_title(&args[1]);
    let title_text2 = page_title(&args[2]);

    let (url , maybe_tirle) = 
      match  trpl::race(titel_text1 , title_text2).await {
            Either::Left(left) =>left,
            Either::Right(right)=>right,
      };
      println!("{url} return first ");
      match maybe_tirle {
           Some(title)=> println!("Its page title is : '{title}' "),

           None =>println!("Its title could not be parsed")
      }
    });
    println!("Hello, world!");
}
async fn page_title(url : &str)-> (&str , Option<String>){
     let text = trpl::get(url).await.text().await;
     let title = Html::parse(&text)
         .select_first("title")
         .map(|title | title.inner_html());
        (url , title)
}