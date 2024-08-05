use crate::{client::bot::StarboardBot, errors::StarboardResult};

pub async fn is_guild_premium(
    bot: &StarboardBot,
    guild_id: i64,
    allow_cache: bool,
) -> StarboardResult<bool> {
    Ok(true)
}
