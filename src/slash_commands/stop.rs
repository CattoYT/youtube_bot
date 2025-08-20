use poise;

type Context<'a> = super::Context<'a>;
type Error = super::Error;

#[poise::command(slash_command, prefix_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(g) => g,
        None => {
            ctx.reply("This command must be used in a guild").await?;
            return Ok(());
        }
    };

    let maybe_handle = ctx.data().active_voice.lock().await.remove(&guild_id);

    if let Some(handle) = maybe_handle {
        let _ = handle.stop();
        ctx.reply("Stopped playback").await?;
    } else {
        ctx.reply("Nothing is playing").await?;
    }

    Ok(())
}
