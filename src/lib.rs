use dekopon_provider_sdk::{
    EffectKind, RiskLevel,
    clap::{self, Parser, Subcommand},
    provider::{Capability, Code, Failure, Proposal, Provider, Stdout, Usage},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub struct Follow;
#[derive(Parser)]
#[command(
    name = "follow",
    disable_help_subcommand = true,
    override_help = "follow stay   Keep reading here without mentions for 24 hours, renewed by each reply.\nfollow leave  Stop reading unmentioned messages here.\n"
)]
pub struct Args {
    #[command(subcommand)]
    verb: Verb,
}
#[derive(Subcommand)]
enum Verb {
    Stay,
    Leave,
}
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Empty {}
#[derive(Debug)]
pub struct Never;
impl std::fmt::Display for Never {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unreachable")
    }
}
impl Failure for Never {
    fn code(&self) -> Code {
        Code::USAGE
    }
}
pub struct Stay;
pub struct Leave;
macro_rules! capability {
    ($name:ident, $verb:literal, $description:literal) => {
        impl Capability for $name {
            type Provider = Follow;
            const NAME: &'static str = $verb;
            const DESCRIPTION: &'static str = $description;
            const EFFECT: EffectKind = EffectKind::LocalWrite;
            const RISK: RiskLevel = RiskLevel::Low;
            type Input = Empty;
            type Needs = ();
            type Error = Never;
            fn run(_: Empty, (): (), _: &mut Stdout) -> Result<(), Self::Error> {
                Ok(())
            }
        }
    };
}
capability!(
    Stay,
    "stay",
    "Keep reading this channel/thread without mentions for 24 hours, renewed by each reply"
);
capability!(Leave, "leave", "Stop reading unmentioned messages here");
impl Provider for Follow {
    const ID: &'static str = "dekopon-provider-follow";
    const COMMAND_WORDS: &'static [&'static str] = &["follow"];
    const DESCRIPTION: &'static str = "Keep reading this conversation without mentions";
    type Args = Args;
    type Capabilities = (Stay, Leave);
    fn propose(args: Args, _: bool) -> Result<Proposal<Self>, Usage> {
        Ok(match args.verb {
            Verb::Stay => Proposal::to::<Stay>(Empty {}),
            Verb::Leave => Proposal::to::<Leave>(Empty {}),
        })
    }
}

#[allow(unsafe_code)]
mod export {
    dekopon_provider_sdk::export!(super::Follow);
}
