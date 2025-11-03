use clap::Parser;
use color_eyre::Result;
use musicrate::app::App;
use musicrate::cli::Args;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let terminal = ratatui::init();
    let app_result = App::new(args).run(terminal);
    ratatui::restore();
    app_result
}
