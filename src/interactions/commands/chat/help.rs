use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::{
    component::{ActionRow, Button, ButtonStyle},
    Component,
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    concat_format, constants, errors::StarboardResult, interactions::context::CommandCtx,
    utils::embed,
};

fn buttons() -> Vec<Component> {
    let link_btn = |name: &str, link: &str| {
        Component::Button(Button {
            custom_id: None,
            disabled: false,
            emoji: None,
            label: Some(name.into()),
            style: ButtonStyle::Link,
            url: Some(link.into()),
        })
    };

    let buttons = vec![
        link_btn("Invite", constants::INVITE_URL),
        link_btn("Support", constants::SUPPORT_URL),
        link_btn("Documentation", constants::DOCS_URL),
        link_btn("Source", constants::SOURCE_URL),
        link_btn("Premium", constants::PATREON_URL),
    ];

    let row = Component::ActionRow(ActionRow {
        components: buttons,
    });

    vec![row]
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "help", desc = "Get help with and general info for Starboard.")]
pub struct Help;

impl Help {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let emb = embed::build()
            .title("Starboard - The Best Discord Starboard")
            .description("Starboard is a reliable and feature-rich starboard bot for Discord.")
            .field(EmbedFieldBuilder::new(
                "Useful Commands",
                concat!(
                    "`/starboards view`: View all of your starboards.\n",
                    "`/starboards create`: Create a new starboard.\n",
                    "`/starboards edit`: Edit the configuration for a starboard.\n",
                ),
            ))
            .field(EmbedFieldBuilder::new(
                "Starboard's Features",
                concat!(
                    "Starboard's key free features are:\n",
                    "- Multiple starboards\n",
                    "- Custom avatar/username for starboards (via webhooks)\n",
                    "- Autostar channels\n",
                    "- Complete per-channel starboard configuration\n",
                    "- Limited per-role starboard configuration\n",
                ),
            ))
            .build();

        ctx.respond(ctx.build_resp().embeds([emb]).components(buttons()).build())
            .await?;

        Ok(())
    }
}
