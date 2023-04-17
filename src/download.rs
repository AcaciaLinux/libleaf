use crate::pbar;
use std::time::Duration;

use crate::error::*;
use curl::easy::Easy;
use indicatif::HumanBytes;
use std::sync::atomic::Ordering::Relaxed;

///Downloads the contents of the supplied url
pub fn download<'data, F>(url: &str, message: &str, mut write_function: F) -> Result<(), LError>
where
    F: FnMut(&[u8]) -> bool + Send + 'data,
{
    //Store the message as a String
    let progress_message = message.to_owned();

    //Create the curl context and set the url
    let mut easy = Easy::new();
    easy.url(url).expect("CURL setup: url()");

    //let thread_buffer = output.clone();
    let mut error = LError::new_class(LErrorClass::Unknown);

    //Setup the low speed bounds (less that 1000bytes in 30 seconds)
    easy.low_speed_limit(1000)
        .expect("CURL setup: low_speed_limit()");
    easy.low_speed_time(Duration::from_secs(30))
        .expect("CURL setup: low_speed_time()");

    //If we can create a progress bar, set the progress function
    match pbar::create_bar(pbar::Template::Download) {
        None => {
            warn!("Failed to create progress bar, continuing without!");
        }
        Some(bar) => {
            easy.progress(true).expect("CURL setup: progress()");
            easy.progress_function(move |dltotal, dlnow, _, _| {
                bar.set_length(dltotal as u64);
                bar.set_position(dlnow as u64);
                bar.set_message(
                    HumanBytes(dlnow as u64).to_string()
                        + " / "
                        + HumanBytes(dltotal as u64).to_string().as_str(),
                );
                bar.set_prefix(progress_message.clone());

                crate::RUNNING.load(Relaxed)
            })
            .expect("CURL setup: progress_function()");
        }
    }

    {
        //Create a scoped transfer and perform it
        let mut transfer = easy.transfer();
        transfer
            .write_function(move |data| {
                if !crate::RUNNING.load(Relaxed) {
                    error.class = LErrorClass::Abort;
                    return Ok(data.len() - 1);
                }

                match write_function(data) {
                    true => Ok(data.len()),
                    false => Ok(data.len() - 1),
                }
            })
            .expect("CURL setup: write_function()");

        //Perform now
        match transfer.perform() {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.is_write_error() {
                    Err(error)
                } else {
                    Err(LError::from(e))
                }
            }
        }
    }
}

impl From<curl::Error> for LError {
    fn from(value: curl::Error) -> Self {
        LError {
            class: LErrorClass::CURL,
            message: Some(format!(
                "{} ({})",
                value.description(),
                value.extra_description().unwrap_or("")
            )),
        }
    }
}
