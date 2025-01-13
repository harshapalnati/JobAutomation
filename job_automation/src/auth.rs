use browser_automation::session::session;
use browser_automation::{session::session::BrowserSession};
use browser_automation::utils::utils::{log_action,random_delay};
use std::error::Error;
use dotenv::dotenv;
use std::env;


pub fn load_cred()->Result<(String,String),Box<dyn Error>>
{
    dotenv().ok();

    let username = env::var("user")?;
    let password=env::var("password")?;
    log_action(&format!("Raw username from .env: {}", password)); // Log full username
  
    Ok((username,password))
}

pub  async  fn login_linkedln(session:&mut BrowserSession,username:&str,password:&str)->Result<(),Box<dyn Error>>
{
    log_action("logging into linkedln.....");
    
    //get username and password fields

    let username_field = session.find_element("input#username").await?;
    username_field.type_text(username).await?;
    log_action("Entered username");
    log_action(username);
     // Find the password input field and enter the password
     let password_field = session.find_element("input#password").await?;
     password_field.type_text(password).await?;
     log_action("Entered password");

      // Find and click the login button
    let login_button = session.find_element("button.btn__primary--large").await?;
    login_button.click().await?;
    log_action("Clicked login button");

     // Optionally, add a random delay to mimic human interaction
     random_delay(1000, 2000).await;

     log_action("Login process completed");

    Ok(())
}