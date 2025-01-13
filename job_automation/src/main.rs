mod auth;
use auth::{login_linkedln,load_cred};
use browser_automation::session::session::{self, BrowserSession};
use std::error::Error;

use dotenv::dotenv;
use std::env;



#[tokio::main]
async  fn main()-> Result<(), Box<dyn std::error::Error>>  {
    
    let (username, password) = load_cred()?;
    

    let mut session=BrowserSession::new("https://www.linkedin.com/login").await?;

    login_linkedln(&mut session, &username, &password).await?;

    session.close();

    Ok(())
}
