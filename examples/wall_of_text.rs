use navni::prelude::*;

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer pulvinar ligula nec lorem rutrum placerat id nec ante. In tincidunt tincidunt nisi sed efficitur. Ut mi nisi, pellentesque quis ex eu, venenatis sagittis nisl. In commodo a neque vel commodo. Curabitur maximus pulvinar turpis, suscipit pulvinar nibh sodales non. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum pellentesque ligula metus, eu ultricies leo luctus non. Maecenas justo augue, scelerisque et varius eu, viverra eu dolor. Pellentesque vehicula iaculis augue. Nulla rhoncus odio nec elementum posuere. ";

fn main() {
    navni::run("wall of text", async {
        loop {
            let (w, h) = navni::char_resolution(80, 24);

            let buf: Vec<CharCell> = LOREM
                .chars()
                .cycle()
                .take((w * h) as usize)
                .map(|c| c.into())
                .collect();

            navni::draw_chars(w, h, &buf).await;

            if navni::keypress().key() == Key::Esc {
                break;
            }
        }
    });
}
