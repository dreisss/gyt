use {
    clap::Parser,
    cli_clipboard::{self, ClipboardContext, ClipboardProvider},
    url::Url,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();
    let img_url: String;
    let mut ctx = ClipboardContext::new().unwrap();

    if let Ok(url) = Url::parse(&args.url) {
        if let Some(query) = url.query() {
            let mut queries = query.split("&");

            if let Some(v_query) = &queries.find(|q| q.contains("v=")) {
                let id = v_query.split("=").last().unwrap();

                img_url = format!("https://img.youtube.com/vi/{id}/mqdefault.jpg");

                println!("{}", img_url);
                ctx.set_contents(img_url.to_owned()).unwrap();

                return;
            }
        }
    }

    panic!("Invalid url.")
}
