use indicatif::ProgressBar;

pub async fn progress_bar() {
    let bar = ProgressBar::new(1000);
    for _ in 0..1000 {
        bar.inc(1);
        // ...
    }
    bar.finish();
}
