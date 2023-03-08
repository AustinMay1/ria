fn main() {
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
        for(j, tag) in tags.iter().enumerate() {
            let lower = tag.saturating_sub(ctx_lines);
            let upper = tag + ctx_lines;

            if (i >= lower) && (i <= upper) {
                let line_as_str = String::from(line);
                let local = (i, line_as_str);
                ctx[j].push(local);
            }
        }
    }

    for local in ctx.iter() {
        for &(i, ref line) in local.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}