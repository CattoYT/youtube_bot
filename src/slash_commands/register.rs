


use poise;


type Context<'a> = super::Context<'a>;
type Error = super::Error;


#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}