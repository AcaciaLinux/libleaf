use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use once_cell::sync::Lazy;

static mut MULTI_BAR: Lazy<MultiProgress> = Lazy::new(|| MultiProgress::new());

pub enum Template {
    Download,
}

pub fn println<I: AsRef<str>>(msg: I) -> std::io::Result<()> {
    unsafe { MULTI_BAR.println(msg) }
}

pub fn create_bar(template: Template) -> Option<ProgressBar> {
    let bar = ProgressBar::new(0);

    let style_e = match template{
        Template::Download => ProgressStyle::with_template("{prefix:<60} [{elapsed_precise}] [{msg:>23.yellow}] [{percent:>3.green}%] [{wide_bar}] ")
    };

    let style = match style_e {
        Ok(s) => s.progress_chars("##-"),
        Err(_) => return None,
    };

    bar.set_style(style);

    Some(unsafe { MULTI_BAR.add(bar) })
}
