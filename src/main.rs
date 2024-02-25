use clap::Args;
use clap::Parser;

mod args;
mod game;
mod game_loop;
mod input;
mod render;
mod themes;

fn main() -> eyre::Result<()> {
    let args = args::Cli::parse();

    if let Some(shell) = args.generate_completions {
        let name = env!("CARGO_BIN_NAME");
        let mut cli = args::Cli::augment_args(clap::Command::new(name));
        clap_complete::generate(shell, &mut cli, name, &mut std::io::stdout());

        return Ok(());
    }

    let conf = args::create_game_conf(&args)?;
    let game = game::GameState::new(conf);
    let theme = args::into_theme(args);

    game_loop::run(game, &theme)?;
    Ok(())
}
