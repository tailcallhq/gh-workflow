use std::path::PathBuf;
use std::process::Command;

use derive_setters::Setters;

use crate::error::{Error, Result};
use crate::{Expression, Job, Permissions, Step, Workflow};

#[derive(Setters, Clone)]
pub struct Generate {
    workflow: Workflow,
    name: String,
}

impl Generate {
    pub fn new(mut workflow: Workflow) -> Self {
        let draft_release = Job::new("Draft release")
            .if_condition(Expression::new(
                "github.event_name == 'push' && github.ref == 'refs/heads/main'",
            ))
            .permissions(Permissions::write())
            .add_step(Step::checkout())
            .add_step(
                Step::uses("release-drafter", "release-drafter", 6)
                    .id("create_release".to_string())
                    .env(("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}"))
                    .with(("config-name", "release-drafter.yml")),
            )
            .add_step(
                Step::run(
                    r#"echo "create_release_name=${{ steps.create_release.outputs.name }}" >> $GITHUB_OUTPUT && echo "create_release_id=${{ steps.create_release.outputs.id }}" >> $GITHUB_OUTPUT"#,
                )
                .id("set_output".to_string())
                .name("Set Output for Later Jobs"),
            )
            .outputs((
                "create_release_name",
                "${{ steps.set_output.outputs.create_release_name }}",
            ))
            .outputs((
                "create_release_id",
                "${{ steps.set_output.outputs.create_release_id }}",
            ));

        let release_job = Job::new("Create release")
            .add_step(Step::checkout())
            .add_step(Step::setup_rust().with_stable_toolchain())
            .add_step(Step::cargo("fetch", vec![""]))
            .add_step(Step::uses(
                "superfly",
                "flyctl-actions/setup-flyctl",
                "master",
            ))
            .add_step(
                Step::run(
                    r#"sed -i.bak "s/version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml"#,
                )
                .env((
                    "NEW_VERSION",
                    "${{ needs.draft_release.outputs.create_release_name }}",
                )),
            )
            // TODO: Make .env(...) type safe
            .add_step(
                Step::run("echo $CRATES_TOKEN | cargo login")
                    .env(("CRATES_TOKEN", "${{ secrets.CRATES_TOKEN }}")),
            )
            .add_step(
                Step::cargo("publish", vec!["--token $CRATES_TOKEN"])
                    .env(("CRATES_TOKEN", "${{ secrets.CRATES_TOKEN }}")),
            )
            .add_step(
                Step::run("flyctl deploy --remote-only")
                    .env(("FLY_API_TOKEN", "${{ secrets.FLY_API_TOKEN }}")),
            );

        workflow = workflow.add_job("draft_release", draft_release);
        workflow = workflow.add_job("release", release_job);
        Self { workflow, name: "ci.yml".to_string() }
    }

    pub fn generate(&self) -> Result<()> {
        let comment = include_str!("./comment.yml");

        let root_dir = String::from_utf8(
            Command::new("git")
                .args(["rev-parse", "--show-toplevel"])
                .output()?
                .stdout,
        )?;

        let path = PathBuf::from(root_dir.trim())
            .join(".github")
            .join("workflows")
            .join(self.name.as_str());

        let content = format!("{}\n{}", comment, self.workflow.to_string()?);

        if std::env::var("CI").is_ok() {
            if let Ok(prev) = std::fs::read_to_string(&path) {
                if content != prev {
                    Err(Error::OutdatedWorkflow)
                } else {
                    println!(
                        "Workflow file is up-to-date: {}",
                        path.canonicalize()?.display()
                    );
                    Ok(())
                }
            } else {
                Err(Error::MissingWorkflowFile(path.clone()))
            }
        } else {
            std::fs::write(path.clone(), content)?;
            println!(
                "Generated workflow file: {}",
                path.canonicalize()?.display()
            );
            Ok(())
        }
    }
}
