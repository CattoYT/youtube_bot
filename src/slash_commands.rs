use poise;
use std::{collections::HashMap, path::PathBuf};

use serenity::model::id::GuildId;
use songbird::tracks::TrackHandle;
use std::sync::Arc;
use tokio::sync::Mutex;
use yt_dlp::Youtube;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod ping;
mod play;
mod register;
mod stop;

pub struct Data {
    youtube: Arc<Mutex<Youtube>>,
    active_voice: Arc<Mutex<HashMap<GuildId, TrackHandle>>>,
}

pub async fn create_framework() -> Result<poise::Framework<Data, Error>, Error> {
    let mut commands = vec![];
    commands.push(ping::ping());
    commands.push(play::play());
    commands.push(register::register());
    commands.push(stop::stop());

    let executables_dir = PathBuf::from("libs");

    let youtube = Youtube::with_new_binaries(executables_dir, PathBuf::from("output")).await?;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands,

            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("FUCK YOU");
                if let Err(e) =
                    poise::builtins::register_globally(ctx, &framework.options().commands).await
                {
                    println!("{e}")
                }

                let data = Data {
                    youtube: Arc::new(Mutex::new(youtube)),
                    active_voice: Arc::new(Mutex::new(HashMap::new())),
                };
                Ok(data)
            })
        })
        .build();

    Ok(framework)
}
