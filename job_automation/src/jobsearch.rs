use browser_automation::session::session::BrowserSession;
use browser_automation::utils::utils::log_action;
use browser_automation::utils::utils::random_delay;
use std::error::Error;
use fantoccini::key::Key;


pub struct JobSearch {
    pub session: BrowserSession,
}

impl JobSearch {
    // Create a new instance of JobSearch
    pub fn new(session: BrowserSession) -> Self {
        Self { session }
    }

    // Perform a job search
    pub async fn job_search(&mut self, keyword: &str, location: &str) -> Result<(), Box<dyn Error>> {
        log_action("Navigating to LinkedIn Jobs page...");
        self.session.navigate("https://www.linkedin.com/jobs/").await?;
    
        log_action("Waiting for the page to load...");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    
        // Start job search
        log_action("Starting job search");
    
        // Find the keyword input field, clear it, and type the keyword
        let keyword_field = self
            .session
            .find_element("input[aria-label='Search by title, skill, or company']")
            .await?;
        keyword_field.clear().await?; // Clear the input field
        log_action("Cleared keyword field");
        keyword_field.type_text(keyword).await?; // Fill in the keyword
        log_action(&format!("Entered keyword: {}", keyword));
    
        // Add a delay before interacting with the location field
       random_delay(1000, 2000).await;
    
        // Find the location input field, clear it, and type the location
        let location_field = self
            .session
            .find_element("input[aria-label='City, state, or zip code']")
            .await?;
        location_field.clear().await?; // Clear the input field
        log_action("Cleared location field");
        location_field.type_text(location).await?; // Fill in the location
        log_action(&format!("Entered location: {}", location));
    
        // Add a delay before clicking the search button
       random_delay(1000, 2000).await;
    
        // Find and click the search button
       // Simulate pressing Enter in the keyword field
    log_action("Submitting search by pressing Enter...");

    keyword_field.send_keys("\u{E007}").await?;
        log_action("Clicked the search button");
    
        // Wait for the results to load
          random_delay(3000, 5000).await;
    
        // Wait for the results to load
    log_action("Waiting for the results to load...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    log_action("Job search completed successfully");
    Ok(())
    }
    

    //Apply filters
    pub async fn apply_filters(&mut self) -> Result<(), Box<dyn Error>> {
        log_action("Applying job search filters...");
    
        // Open the "Date Posted" dropdown
       // Open the "Date Posted" dropdown
    let date_posted_button = self
    .session
    .find_element("button#searchFilter_timePostedRange")
    .await?;
date_posted_button.click().await?;
log_action("Opened 'Date Posted' dropdown.");

// Wait briefly for the options to load
tokio::time::sleep(std::time::Duration::from_secs(1)).await;

// Select the "Past 24 hours" option
// Select the "Past 24 hours" option by clicking the label
let past_24_hours_label = self
    .session
    .find_element("label[for='timePostedRange-r86400']")
    .await?;
past_24_hours_label.click().await?;
log_action("Selected 'Past 24 hours' in Date Posted filter.");

         // Click the "Show results" button
    let apply_button = self
    .session
    .find_element("button[aria-label^='Apply current filter to show']")
    .await?;
apply_button.click().await?;
log_action("Clicked 'Show results' button to apply filters.");

    
        // Open the "Experience Level" dropdown
      
        // Wait briefly for the options to load
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    
        // Select "Entry Level" and "Associate"
       
        
        log_action("Filters applied successfully.");
        Ok(())
    }
    


    // Close the session
    pub async fn close(self) -> Result<(), Box<dyn Error>> {
        self.session.close().await?;
        Ok(())
    }
}
