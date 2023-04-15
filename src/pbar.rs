use indicatif::{ProgressBar, ProgressStyle};

pub enum Template {
    Download,
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

    Some(bar)
}
