mod auth;
mod jobsearch;

use auth::{login_linkedln, load_cred};
use browser_automation::session::session::{self, BrowserSession};
use jobsearch::JobSearch;
use std::error::Error;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load environment variables

    // Load credentials
    let (username, password) = load_cred()?;

    // Start a browser session and log in
    let mut session = BrowserSession::new("https://www.linkedin.com/login").await?;
    session.set_window_size(1920, 1080).await?;
    login_linkedln(&mut session, &username, &password).await?;

    // Navigate to the jobs page and perform a job search
    let mut job_search = JobSearch::new(session);
    job_search.job_search("Software Engineer", "United States").await?;

    // Apply filters to refine search results
    job_search.apply_filters().await?; // Call the filter function correctly here

    println!("Filters applied and job search completed!");

    Ok(())
}
