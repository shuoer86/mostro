use crate::cli::settings::Settings;
use crate::util::{get_market_quote, publish_order, send_dm};

use anyhow::Result;
use log::error;
use mostro_core::{Action, Message};
use nostr_sdk::prelude::ToBech32;
use nostr_sdk::{Client, Event, Keys};
use sqlx::{Pool, Sqlite};

pub async fn order_action(
    msg: Message,
    event: &Event,
    my_keys: &Keys,
    client: &Client,
    pool: &Pool<Sqlite>,
) -> Result<()> {
    if let Some(order) = msg.get_order() {
        let mostro_settings = Settings::get_mostro();
        let quote = match get_market_quote(&order.fiat_amount, &order.fiat_code, &0).await {
            Ok(amount) => amount,
            Err(e) => {
                error!("{:?}", e.to_string());
                return Ok(());
            }
        };
        if quote > mostro_settings.max_order_amount as i64 {
            let message = Message::new(0, order.id, None, Action::CantDo, None);
            let message = message.as_json()?;
            send_dm(client, my_keys, &event.pubkey, message).await?;

            return Ok(());
        }

        let initiator_ephemeral_pubkey = event.pubkey.to_bech32()?;
        let master_pubkey = match msg.pubkey {
            Some(ref pk) => pk,
            None => {
                // We create a Message
                let message = Message::new(0, order.id, None, Action::CantDo, None);
                let message = message.as_json()?;
                send_dm(client, my_keys, &event.pubkey, message).await?;

                return Ok(());
            }
        };

        publish_order(
            pool,
            client,
            my_keys,
            order,
            &initiator_ephemeral_pubkey,
            master_pubkey,
            event.pubkey,
        )
        .await?;
    }
    Ok(())
}
