
use std::path::PathBuf;

use poise;
use songbird::input::{File, YoutubeDl};
use yt_dlp::model::{AudioCodecPreference, AudioQuality};

use reqwest;
type Context<'a> = super::Context<'a>;
type Error = super::Error;



#[poise::command(slash_command, guild_only, prefix_command)]
pub async fn play(ctx: Context<'_>, #[description = "Url to video"] url: Option<String>) -> Result<(), Error> {
    let video_url = match url {
        Some(url) => url,
        None => {
            ctx.say("Please provide a URL to play a video.").await?;
            return Ok(());
        }
    };

    ctx.say(format!("Playing video: {}", video_url)).await?;


            
    println!("User {} requested to play video: {}", ctx.author().name, video_url);
    let youtube = &ctx.data().youtube.lock().await;
    let song_title = match youtube.fetch_video_infos(video_url.clone()).await {
        Ok(video) => video.title,
        Err(e) => {
            ctx.say("Please provide a valid URL to play a video.").await?;
            return Ok(())
        }
    };

    let audio_stream = youtube.download_audio_stream_with_quality(video_url.to_string(), song_title + ".mp3", AudioQuality::High, AudioCodecPreference::MP3).await.unwrap();
    
    let manager = songbird::get(&ctx.serenity_context()).await.expect("d");
    let voice_channel_id = ctx.guild().unwrap().voice_states.get(&ctx.author().id).and_then(|vc| vc.channel_id);
    if let None = voice_channel_id {
        ctx.reply("You are not in a VC!").await.unwrap();
    }
    let voice_handle = manager.join(ctx.guild_id().expect("guild_only"), voice_channel_id.unwrap()).await; 

    //voice_handle.unwrap().lock().await.play_input(YoutubeDl::new(reqwest::Client::new(), video_url).into()).play().expect("ffs");
    match voice_handle {
        Ok(handle) => {
            let input: File<PathBuf> = File::new(audio_stream)
                .into();
            println!("should play now");
            
            handle.lock().await.play_input(input.clone().into()).set_volume(0.4);
            println!("should played");

        },
        Err(e) => {
            ctx.say(format!("Failed to join voice channel: {}", e)).await?;
            return Err(Box::new(e));
        }
    }


    // TODO: doesnt play the file???
    Ok(())
    
    
    

    

}