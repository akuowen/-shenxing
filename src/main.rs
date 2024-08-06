use reedline_repl_rs::Repl;
use shenxing::{func, ShenXingContext, SxCommand};

const HISTORY_SIZE: usize = 1024;

fn main() -> anyhow::Result<()> {
    let ctx = ShenXingContext::new();
    let history_file = dirs::home_dir()
        .expect("expect home dir")
        .join(".Shenxing_history");

    let mut repl = Repl::new(ctx)
        .with_history(history_file, HISTORY_SIZE)
        .with_banner(
            r#"
          _________.__                         .__                
         /   _____/|  |__   ____   ____ ___  __|__| ____    ____  
         \_____  \ |  |  \_/ __ \ /    \\  \/  /  |/    \  / ___\ 
         /        \|   Y  \  ___/|   |  \>    <|  |   |  \/ /_/  >
        /_______  /|___|  /\___  >___|  /__/\_ \__|___|  /\___  / 
                \/      \/     \/     \/      \/       \//_____/  
        "#,
        )
        .with_derived::<SxCommand>(func());

    repl.run()?;

    Ok(())
}
