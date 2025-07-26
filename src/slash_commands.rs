use poise;
use std::{path::PathBuf};

use tokio::{pin, sync::Mutex};
use std::sync::Arc;
use yt_dlp::Youtube;

use crate::slash_commands::register::register;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod ping;
mod play;
mod register;

pub struct Data {
    youtube: Arc<Mutex<Youtube>>,
}

pub async fn create_framework() -> Result<poise::Framework<Data, Error>, Error> {
    let mut commands = vec![];
    commands.push(ping::ping());
    commands.push(play::play());
    commands.push(register::register());

    let executables_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");
    
    let youtube = Youtube::with_new_binaries(executables_dir, PathBuf::from("output")).await?;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands,

            ..Default::default()
        })
        
        .setup(|ctx, _ready, framework| {

            Box::pin(async move {
                println!("FUCK YOU");
                if let Err(e) = poise::builtins::register_globally(ctx, &framework.options().commands).await {
                    println!("{e}")
                }



                let data = Data{youtube: Arc::new(Mutex::new(youtube))};
                Ok(data)
            })
        })
        .build();
    
    Ok(framework)

}