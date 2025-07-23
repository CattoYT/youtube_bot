
use poise;
use tokio::time::Instant;

type Context<'a> = super::Context<'a>;
type Error = super::Error;


#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    // ⏱ Start timing
    println!("User {} ran /ping!", ctx.author().name);
    let start = Instant::now();

    // Send a temporary message
    let msg = ctx.say("Pinging... 🏓").await?;

    // Stop timing after message is sent
    let elapsed = start.elapsed().as_millis();

    // Edit message with latency
    let edit_contents = poise::CreateReply{..Default::default()}.content(format!("🏓 Pong! Round-trip latency: {}ms~", elapsed));
    msg.edit(ctx, edit_contents).await?;

    Ok(())
}