fn array() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3]; // defining array type with array length pattern [T; {length}]
    let blank1 = [0; 3]; // defining array of length 3 and filling all values with 0
    let blank2: [u8; 3] = [0; 3]; // same as above

    let arrays = [one, two, blank1, blank2];
    for a in &arrays {
        print!("{:?}", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i]
        }
        println!("\tsumof({:?}) = {}", a, sum);
    }
}

fn vector() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if i >= lower_bound && i <= upper_bound {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            println!("{}: {}", i + 1, line);
        }
    }
}

fn main() {
    println!("Array [] example");
    array();
    println!("Vec<T> example");
    vector();
}
