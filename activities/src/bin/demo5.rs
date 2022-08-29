struct Book{
    pages: i32,
    rating: i32,
    price: i32,
}
fn display_page_count(book:&Book){
    println!("pages={:?}",book.pages);
}
fn display_rating(book:&Book){
    println!("rating={:?}",book.rating);
}
fn display_rating(book:&Book){
    println!("price={:?}",book.price);
}
fn main(){
    let book=Book{
        pages: 5,
        rating: 9,
        price:2000,
    };
    display_page_count(&book);
    display_rating(&book);
    display_price(&book);
}