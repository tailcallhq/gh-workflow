use serde::Serialize;

#[derive(Serialize, Debug, Clone, )]
pub struct Workflow {
    pub(crate) name: String,
    pub(crate) on: On,
    pub(crate) jobs: Jobs,
}


#[derive(Serialize, Debug, Clone, )]
pub struct On {
    pub(crate) push: Branches,
    pub(crate) pull_request: Branches,
}

#[derive(Serialize, Debug, Clone, )]
pub struct Branches {
    pub(crate) branches: Vec<String>,
}

#[derive(Serialize, Debug, Clone, )]
pub struct Jobs {
    pub(crate) build: Job,
}

#[derive(Serialize, Debug, Clone, )]
pub struct Job {
    #[serde(rename = "runs-on")]
    pub(crate) runs_on: String,
    pub(crate) steps: Vec<Step>,
}

#[derive(Serialize, Debug, Clone, )]
pub struct Step {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run: Option<String>,
}
