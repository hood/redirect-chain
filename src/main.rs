use anyhow::{Context, Result};
use reqwest::blocking::Client;

fn main() -> Result<()> {
    let url = std::env::args()
        .nth(1)
        .context("Usage: redirect_chain <url>")?;

    println!("Tracing redirects for: {}", url);

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let mut current = url;
    let mut hop = 1;

    loop {
        let resp = client
            .get(&current)
            .send()
            .with_context(|| format!("Failed requesting {}", current))?;

        let status = resp.status();
        let location = resp.headers().get("Location");

        println!("{}: {} -> {}", hop, status, current);

        if let Some(loc) = location {
            let loc = loc.to_str()?.to_string();
            let next_url = reqwest::Url::parse(&loc)
                .or_else(|_| reqwest::Url::parse(&current)?.join(&loc))?
                .to_string();

            current = next_url;
            hop += 1;
        } else {
            break;
        }

        if hop > 20 {
            println!("Stopped: too many redirects");
            break;
        }
    }

    Ok(())
}
