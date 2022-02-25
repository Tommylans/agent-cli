use agent_controller::modules::credentials::{CredentialsModule, CredentialsOfferOptions};
use clap::{Args, Subcommand};

use crate::error::{Error, Result};
use crate::utils::logger::Log;

#[derive(Args)]
pub struct CredentialOptions {
    #[clap(subcommand)]
    pub commands: CredentialSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum CredentialSubcommands {
    Offer {
        #[clap(long, short = 'i')]
        connection_id: String,
        #[clap(long, short)]
        cred_def_id: String,
        #[clap(long, short)]
        key: Vec<String>,
        #[clap(long, short)]
        value: Vec<String>,
    },
    Propose {
        #[clap(long, short)]
        id: String,
    },
}

pub async fn parse_credentials_args(
    commands: &CredentialSubcommands,
    agent: impl CredentialsModule,
    logger: Log,
) -> Result<()> {
    match commands {
        CredentialSubcommands::Offer {
            connection_id,
            cred_def_id,
            key,
            value,
        } => {
            if key.len() != value.len() {
                return Err(Error::UnqualAmountKeyValue.into());
            }

            let options = CredentialsOfferOptions {
                connection_id: connection_id.to_string(),
                cred_def_id: cred_def_id.to_string(),
                keys: key.iter().map(|k| k.to_string()).collect(),
                values: value.iter().map(|v| v.to_string()).collect(),
            };
            agent.send_offer(options).await.map(|res| {
                if logger.debug {
                    logger.log_pretty(res)
                } else {
                    logger.log("Successfully offered a credential!");
                }
            })
        }
        CredentialSubcommands::Propose { id: _id } => todo!(),
    }
}