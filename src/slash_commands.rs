use poise;
use std::{collections::HashMap, path::PathBuf};

use serenity::{cache, gateway, model::id::GuildId};
use songbird::tracks::TrackHandle;
use std::sync::Arc;
use tokio::sync::Mutex;
use yt_dlp::{fetcher::deps::youtube, Youtube};

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

    let mut youtube = Youtube::with_new_binaries(executables_dir, PathBuf::from("output")).await?;
    youtube.with_args(vec!["--cookies".to_string(), "cookies.txt".to_string()]);
    if let Some(cache) = &youtube.cache {
        cache.clean();
    }


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
