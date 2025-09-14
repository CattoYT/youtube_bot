use std::path::PathBuf;

use poise;
use songbird::input::{File};
use yt_dlp::model::{AudioCodecPreference, AudioQuality};

use songbird::tracks::TrackHandle;
type Context<'a> = super::Context<'a>;
type Error = super::Error;

#[poise::command(slash_command, guild_only, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Url to video"] url: Option<String>,
) -> Result<(), Error> {
    let video_url = match url {
        Some(url) => url,
        None => {
            ctx.reply("Please provide a URL to play a video.").await?;
            return Ok(());
        }
    };

    let _ = ctx.defer_ephemeral().await;

    println!(
        "User {} requested to play video: {}",
        ctx.author().name,
        video_url
    );
    let youtube = &ctx.data().youtube.lock().await;
    let video_info = match youtube.fetch_video_infos(video_url.clone()).await {
        Ok(video) => video,
        Err(e) => {
            ctx.reply("Please provide a valid URL to play a video.")
                .await?;
            println!("{e}");
            return Ok(());
        }
    };
    let reply = poise::CreateReply::default()
        .embed(
            poise::serenity_prelude::CreateEmbed::new()
                .title("Music Bot:tm:")
                .description(format!(
                    "Now playing: [{}]({})",
                    &video_info.title, &video_url
                ))
                .image(video_info.thumbnail),
        )
        .ephemeral(true);

    ctx.send(reply).await?;

    

    let download_stream = youtube
        .download_audio_stream_with_quality(
            video_url.to_string(),
            video_info.title.trim().to_owned() + ".mp3",
            AudioQuality::Best,
            AudioCodecPreference::Any,
        )
        .await;

    let audio_stream = match download_stream {
        Ok(stream) => {
            println!("Audio stream downloaded successfully.");
            stream
        }
        Err(e) => {
            ctx.reply(format!("Failed to download audio stream: {}", e))
                .await?;
            youtube
                .download_audio_stream_from_url(
                    video_url.to_string(),
                    video_info.title.trim().to_owned() + ".mp3",
                )
                .await
                .unwrap()
        }
    };
    
    let guild_id = ctx.guild_id().expect("guild_only");
    {
        let map = ctx.data().active_voice.lock().await;
        if map.contains_key(&guild_id) {
            ctx.reply("Bot is already playing something, shush or kick the bot from vc")
                .await?;
            return Ok(());
        }
    }

    

    let manager = songbird::get(&ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed at initialization");
    let voice_channel_id = ctx
        .guild()
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|vc| vc.channel_id);
    let channel_id = match voice_channel_id {
        Some(id) => id,
        None => {
            ctx.reply("You are not in a VC!").await?;
            return Ok(());
        }
    };

    let join_result = manager.join(guild_id, channel_id).await;
    let handler_lock = match join_result {
        Ok(handler) => handler,
        Err(_) => {
            ctx.say("Failed to join voice channel").await?;
            return Ok(());
        }
    };
    let input: File<PathBuf> = File::new(audio_stream).into();

    let mut handler = handler_lock.lock().await;
    let track_handle: TrackHandle = handler.play_input(input.into());
    let _ = track_handle.set_volume(0.2);

    // store tracjhandler to the state for pausing later
    ctx.data()
        .active_voice
        .lock()
        .await
        .insert(guild_id, track_handle);


    Ok(())
}
