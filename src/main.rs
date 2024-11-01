use rusqlite::{Connection, OpenFlags, Result};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use url::Url;

// Type alias to handle different error types
type MyResult<T> = Result<T, Box<dyn Error>>;

// 'url' is the Url found in the database,
fn strip_url(query: &str, url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).ok()?;
    let host = parsed_url.host_str()?;
    let hostp = if let Some(port) = parsed_url.port() {
        format! {"{host}:{port}"}
    } else { host.into() };
    let path = parsed_url.path().trim_end_matches('/');
    let params = parsed_url.query();

    if query.contains('#') {
        return Some(url.into());
    }
    if query.contains('?') {
        return Some(format!("{hostp}{path}?{params}", params = params?));
    }
    if query.contains('/') {
        Some(format!("{hostp}{path}"))
    } else {
        Some(hostp.to_string())
    }
}



fn fetch_history(query: &str) -> MyResult<Vec<String>> {
    // Path to Chrome's history SQLite database.
    let history_db = get_history_db();
    let tmp_db_path = "/tmp/web_history.db";

    // Copy the database file to a temporary location.
    // We need to copy the file as it is locked when Google Chrome is running.
    fs::copy(&history_db, &tmp_db_path).map_err(|e| {
        eprintln!("Failed to copy history database: {e}");
        Box::new(e) as Box<dyn Error>
    })?;

    // Open the copied database.
    let conn = Connection::open_with_flags(
        tmp_db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    // Prepare SQL query to fetch URLs based on the query.
    let query_like = format!("{}%", query.replace('*', "%"));

    let mut stmt = conn.prepare(
        "SELECT url FROM urls 
         WHERE url LIKE ?1 OR url LIKE ?2",
    )?;
    //   ORDER BY last_visit_time
    let query_https = format!("https://{query_like}");
    let query_http = format!("http://{query_like}");

    // Fetch matching URLs
    let mut rows = stmt.query([&query_https, &query_http])?;
    let mut results: Vec<String> = Vec::new();

    while let Some(row) = rows.next()? {
        let url: String = row.get(0)?;
        if let Some(stripped) = strip_url(&query, &url) {
            results.push(stripped);
        }
    }

    results.sort();
    results.dedup();

    let _ = fs::remove_file(&tmp_db_path);
    Ok(results)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: web-hist <query>");
        return;
    }

    let query = &args[1];
    let history_matches = fetch_history(query).unwrap_or_else(|err| {
        eprintln!("Error fetching history: {err}");
        vec![] // Empty result if history fetching fails
    });

    // Print the history matches
    for url in history_matches {
        println!("{url}");
    }
}

fn get_history_db() -> PathBuf {
    if let Ok(history_file) = env::var("WEB_HISTORY_FILE") {
        PathBuf::from(history_file)
    } else {
        let home_dir = env::var("HOME").expect("Could not find home directory");
        PathBuf::from(format!("{home_dir}/.config/google-chrome/Default/History"))
    }
}
