fn main() {
    let search = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house and dark square is a picture feverishly turned--in search of what?
    It is the same with books.
    What do we seek through millions of pages?";
    
    // instead of using a counter iterator can be chained with the enumerate() function to return (index, value) tuple.
    let mut line_counter: usize = 1;

    for line in quote.lines() {
        if line.contains(search) {
            println!("{}: {}", line_counter, line);
        }
        line_counter += 1;
    }
}
