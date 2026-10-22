use dekopon_follow_provider::{Follow, Leave, Stay};
use dekopon_provider_sdk::{
    CommandRunOutcome, EffectKind, RiskLevel,
    provider::{self, Capability, Provider},
};
use dekopon_provider_sdk_testkit::{Harness, conformance};
use serde_json::{Map, json};

fn component() -> std::path::PathBuf {
    std::env::var_os("DEKOPON_PROVIDER_COMPONENT")
        .expect("DEKOPON_PROVIDER_COMPONENT must name the freshly built component")
        .into()
}

#[test]
fn built_manifest_declares_only_closed_empty_stay_and_leave()
-> Result<(), Box<dyn std::error::Error>> {
    conformance::<Follow>(component())?;
    let manifest = provider::manifest::<Follow>()?;
    assert_eq!(manifest.command_words, ["follow"]);
    assert_eq!(
        manifest
            .capabilities
            .iter()
            .map(|cap| cap.id.to_string())
            .collect::<Vec<_>>(),
        [
            "dekopon-provider-follow.stay",
            "dekopon-provider-follow.leave"
        ]
    );
    for capability in manifest.capabilities {
        assert_eq!(capability.input_schema["type"], "object");
        assert_eq!(capability.input_schema["additionalProperties"], false);
        assert_eq!(
            capability.input_schema["properties"]
                .as_object()
                .map(Map::len)
                .unwrap_or(0),
            0
        );
        assert_eq!(capability.effect, EffectKind::LocalWrite);
        assert_eq!(capability.risk, RiskLevel::Low);
    }
    Ok(())
}

#[test]
fn commands_propose_empty_inputs_and_built_capabilities_succeed_silently()
-> Result<(), Box<dyn std::error::Error>> {
    for verb in ["stay", "leave"] {
        let CommandRunOutcome::Proposed {
            capability,
            input,
            secret_use,
        } = provider::command::<Follow>(&[verb.to_owned()], false)
        else {
            panic!("expected proposal")
        };
        assert_eq!(
            capability.to_string(),
            format!("dekopon-provider-follow.{verb}")
        );
        assert_eq!(input, json!({}));
        assert!(secret_use.is_none());
        let output = Harness::<Follow>::get(component()).call(capability.as_str(), input)?;
        assert_eq!(output.status, 0);
        assert!(output.stdout.is_empty());
        assert!(output.stderr.is_empty());
        assert!(output.http_calls.is_empty());
    }
    Ok(())
}

#[test]
fn native_commands_reject_extra_arguments_and_unknown_verbs() {
    for argv in [
        vec!["stay", "extra"],
        vec!["leave", "extra"],
        vec!["join"],
        vec![],
    ] {
        let args = argv.into_iter().map(str::to_owned).collect::<Vec<_>>();
        assert!(matches!(
            provider::command::<Follow>(&args, false),
            CommandRunOutcome::Rendered { status: 2, .. }
        ));
    }
}

#[test]
fn built_capabilities_reject_nonempty_inputs() -> Result<(), Box<dyn std::error::Error>> {
    for verb in ["stay", "leave"] {
        let output = Harness::<Follow>::get(component()).call(
            &format!("dekopon-provider-follow.{verb}"),
            json!({"channel":"elsewhere"}),
        )?;
        assert_eq!(output.status, 2);
        assert!(output.stdout.is_empty());
        assert!(output.http_calls.is_empty());
    }
    Ok(())
}

#[test]
fn native_help_uses_consistent_clap_usage_and_descriptions() {
    let help = |words: &[&str]| {
        let argv = words
            .iter()
            .map(|word| (*word).to_owned())
            .collect::<Vec<_>>();
        let CommandRunOutcome::Rendered {
            stdout,
            stderr,
            status,
        } = provider::command::<Follow>(&argv, false)
        else {
            panic!("expected rendered help")
        };
        assert_eq!(status, 0);
        assert!(stderr.is_empty());
        stdout
    };
    let root = help(&["--help"]);
    assert_eq!(root, help(&["help"]));
    assert!(root.contains(<Follow as Provider>::DESCRIPTION));
    assert!(root.contains("Usage: follow <COMMAND>"));
    assert!(root.contains("Commands:"));
    for (verb, description) in [
        ("stay", <Stay as Capability>::DESCRIPTION),
        ("leave", <Leave as Capability>::DESCRIPTION),
    ] {
        assert!(
            root.lines()
                .any(|line| line.trim_start().starts_with(verb) && line.contains(description))
        );
        let subcommand = help(&[verb, "--help"]);
        assert_eq!(subcommand, help(&["help", verb]));
        assert!(subcommand.contains(description));
        assert!(subcommand.contains(&format!("Usage: follow {verb}")));
    }
}
