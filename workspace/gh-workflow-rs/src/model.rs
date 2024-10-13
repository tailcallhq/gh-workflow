// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::model;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: model = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Model {
    #[serde(rename = "$schema")]
    schema: String,

    #[serde(rename = "$id")]
    id: String,

    #[serde(rename = "$comment")]
    comment: String,

    additional_properties: bool,

    definitions: Definitions,

    properties: ModelProperties,

    required: Vec<String>,

    #[serde(rename = "type")]
    model_type: GroupType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Definitions {
    architecture: Architecture,

    branch: Branch,

    concurrency: DefinitionsConcurrency,

    configuration: Configuration,

    container: Container,

    defaults: Defaults,

    permissions: SecretsClass,

    #[serde(rename = "permissions-event")]
    permissions_event: PermissionsEvent,

    #[serde(rename = "permissions-level")]
    permissions_level: Architecture,

    env: Env,

    environment: Environment,

    event: Event,

    event_object: EventObject,

    expression_syntax: ExpressionSyntax,

    string_containing_expression_syntax: ExpressionSyntax,

    globs: Globs,

    machine: Architecture,

    name: ItemsClass,

    path: Branch,

    #[serde(rename = "ref")]
    definitions_ref: Ref,

    shell: Shell,

    types: ExpressionSyntax,

    #[serde(rename = "working-directory")]
    working_directory: ExpressionSyntax,

    job_needs: JobNeeds,

    matrix: Matrix,

    reusable_workflow_call_job: ReusableWorkflowCallJob,

    normal_job: NormalJob,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct Architecture {
    #[serde(rename = "type")]
    architecture_type: TypeElement,

    #[serde(rename = "enum")]
    architecture_enum: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone)]
pub enum TypeElement {
    Boolean,

    Number,

    String,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct Branch {
    #[serde(rename = "$comment")]
    comment: String,

    #[serde(rename = "$ref")]
    branch_ref: BranchRef,

    description: String,

    properties: Option<BranchProperties>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum BranchRef {
    #[serde(rename = "#/definitions/defaults")]
    DefinitionsDefaults,

    #[serde(rename = "#/definitions/env")]
    DefinitionsEnv,

    #[serde(rename = "#/definitions/eventObject")]
    DefinitionsEventObject,

    #[serde(rename = "#/definitions/globs")]
    DefinitionsGlobs,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct BranchProperties {
    args: Option<Args>,

    entrypoint: Option<Args>,

    types: Option<Types>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct Args {
    #[serde(rename = "$comment")]
    comment: String,

    #[serde(rename = "type")]
    args_type: TypeElement,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct Types {
    #[serde(rename = "$ref")]
    types_ref: TypesRef,

    items: Architecture,

    #[serde(rename = "default")]
    types_default: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum TypesRef {
    #[serde(rename = "#/definitions/types")]
    DefinitionsTypes,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct DefinitionsConcurrency {
    #[serde(rename = "type")]
    concurrency_type: GroupType,

    properties: ConcurrencyProperties,

    required: Vec<String>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone)]
pub enum GroupType {
    Boolean,

    Object,

    String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(derive_setters::Setters, Clone)]
pub struct ConcurrencyProperties {
    group: GroupClass,

    cancel_in_progress: CancelInProgressClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct CancelInProgressClass {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    one_of: Option<Vec<ConcurrencyOneOf>>,

    #[serde(rename = "default")]
    concurrency_default: Option<DefaultUnion>,

    #[serde(rename = "type")]
    concurrency_type: Option<Vec<TypeElement>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[derive(Clone)]
pub enum DefaultUnion {
    Bool(bool),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct ConcurrencyOneOf {
    #[serde(rename = "type")]
    one_of_type: Option<TypeElement>,

    #[serde(rename = "$ref")]
    one_of_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct GroupClass {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    name_type: GroupType,

    properties: Option<NameProperties>,

    additional_properties: Option<PermissionsValue>,

    #[serde(rename = "enum")]
    name_enum: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct PermissionsValue {
    #[serde(rename = "$ref")]
    permissions_ref: String,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct NameProperties {
    username: OneOf,

    password: OneOf,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct OneOf {
    #[serde(rename = "type")]
    one_of_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Configuration {
    one_of: Vec<ConfigurationOneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct ConfigurationOneOf {
    #[serde(rename = "type")]
    one_of_type: String,

    additional_properties: Option<PermissionsValue>,

    items: Option<PermissionsValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Container {
    #[serde(rename = "type")]
    container_type: GroupType,

    properties: ContainerProperties,

    required: Vec<String>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct ContainerProperties {
    image: GroupClass,

    credentials: GroupClass,

    env: Branch,

    ports: Ports,

    volumes: Volumes,

    options: GroupClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Ports {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    ports_type: String,

    items: AdditionalPropertiesClass,

    min_items: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct AdditionalPropertiesClass {
    one_of: Vec<OneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Volumes {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    volumes_type: String,

    items: ItemsClass,

    min_items: i64,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct ItemsClass {
    #[serde(rename = "type")]
    name_type: TypeElement,

    pattern: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Defaults {
    #[serde(rename = "type")]
    defaults_type: GroupType,

    properties: PurpleProperties,

    min_properties: i64,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct PurpleProperties {
    run: Run,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Run {
    #[serde(rename = "type")]
    run_type: GroupType,

    properties: FluffyProperties,

    min_properties: i64,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(derive_setters::Setters, Clone)]
pub struct FluffyProperties {
    shell: PermissionsValue,

    working_directory: PermissionsValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Ref {
    properties: RefProperties,

    one_of: Vec<RefOneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct RefOneOf {
    #[serde(rename = "type")]
    one_of_type: String,

    all_of: Option<Vec<OneOfAllOf>>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct OneOfAllOf {
    not: Not,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct Not {
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(derive_setters::Setters, Clone)]
pub struct RefProperties {
    branches: PermissionsValue,

    branches_ignore: PermissionsValue,

    tags: PermissionsValue,

    tags_ignore: PermissionsValue,

    paths: PermissionsValue,

    paths_ignore: PermissionsValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Env {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    one_of: Vec<EnvOneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct EnvOneOf {
    #[serde(rename = "type")]
    one_of_type: Option<GroupType>,

    additional_properties: Option<AdditionalPropertiesClass>,

    #[serde(rename = "$ref")]
    one_of_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Environment {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    environment_type: GroupType,

    properties: EnvironmentProperties,

    required: Vec<String>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct EnvironmentProperties {
    name: GroupClass,

    url: GroupClass,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct Event {
    #[serde(rename = "$comment")]
    comment: Option<String>,

    #[serde(rename = "type")]
    event_type: TypeElement,

    #[serde(rename = "enum")]
    event_enum: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct EventObject {
    one_of: Vec<OneOf>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct ExpressionSyntax {
    #[serde(rename = "$comment")]
    comment: String,

    #[serde(rename = "type")]
    expression_syntax_type: String,

    pattern: Option<String>,

    description: Option<String>,

    min_items: Option<i64>,

    items: Option<ExpressionSyntaxItems>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct ExpressionSyntaxItems {
    properties: Option<ItemsProperties>,

    additional_properties: Option<bool>,

    #[serde(rename = "type")]
    items_type: Option<TypeElement>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct ItemsProperties {
    cron: OneOf,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Globs {
    #[serde(rename = "type")]
    globs_type: String,

    items: GlobsItems,

    min_items: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct GlobsItems {
    #[serde(rename = "type")]
    items_type: TypeElement,

    min_length: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct JobNeeds {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    one_of: Vec<JobNeedsOneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct JobNeedsOneOf {
    #[serde(rename = "type")]
    one_of_type: Option<String>,

    items: Option<PermissionsValue>,

    min_items: Option<i64>,

    #[serde(rename = "$ref")]
    one_of_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Matrix {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    one_of: Vec<MatrixOneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct MatrixOneOf {
    #[serde(rename = "type")]
    one_of_type: Option<GroupType>,

    pattern_properties: Option<OneOfPatternProperties>,

    additional_properties: Option<AdditionalProperties>,

    min_properties: Option<i64>,

    #[serde(rename = "$ref")]
    one_of_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct AdditionalProperties {
    one_of: Vec<JobNeedsOneOf>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct OneOfPatternProperties {
    #[serde(rename = "^(in|ex)clude$")]
    in_ex_clude: InExClude,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct InExClude {
    #[serde(rename = "$comment")]
    comment: String,

    one_of: Vec<InExCludeOneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct InExCludeOneOf {
    #[serde(rename = "$ref")]
    one_of_ref: Option<String>,

    #[serde(rename = "type")]
    one_of_type: Option<String>,

    items: Option<PurpleItems>,

    min_items: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct PurpleItems {
    #[serde(rename = "type")]
    items_type: GroupType,

    additional_properties: PermissionsValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct NormalJob {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    normal_job_type: GroupType,

    properties: NormalJobProperties,

    required: Vec<String>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(derive_setters::Setters, Clone)]
pub struct NormalJobProperties {
    name: GroupClass,

    needs: PermissionsValue,

    permissions: PermissionsValue,

    runs_on: RunsOn,

    environment: CancelInProgressClass,

    outputs: Outputs,

    env: Branch,

    defaults: Branch,

    #[serde(rename = "if")]
    properties_if: MaxParallelClass,

    steps: Steps,

    timeout_minutes: CancelInProgressClass,

    strategy: Strategy,

    continue_on_error: CancelInProgressClass,

    container: CancelInProgressClass,

    services: GroupClass,

    concurrency: CancelInProgressClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Outputs {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    outputs_type: GroupType,

    additional_properties: OneOf,

    min_properties: i64,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct MaxParallelClass {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    if_type: Vec<TypeElement>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct RunsOn {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    any_of: Vec<RunsOnAnyOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct RunsOnAnyOf {
    #[serde(rename = "$comment")]
    comment: Option<String>,

    #[serde(rename = "type")]
    any_of_type: Option<String>,

    any_of: Option<Vec<AnyOfAnyOf>>,

    properties: Option<AnyOfProperties>,

    #[serde(rename = "$ref")]
    any_of_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct AnyOfAnyOf {
    items: Vec<OneOf>,

    min_items: i64,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct AnyOfProperties {
    group: OneOf,

    labels: Labels,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Labels {
    one_of: Vec<BrancheTagPathSIgnore>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct BrancheTagPathSIgnore {
    #[serde(rename = "type")]
    branche_tag_path_s_ignore_type: String,

    items: Option<OneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Steps {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    steps_type: String,

    items: StepsItems,

    min_items: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct StepsItems {
    all_of: Vec<ItemsAllOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct ItemsAllOf {
    one_of: Option<Vec<AllOfOneOf>>,

    #[serde(rename = "type")]
    all_of_type: Option<GroupType>,

    properties: Option<AllOfProperties>,

    dependencies: Option<Dependencies>,

    additional_properties: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(derive_setters::Setters, Clone)]
pub struct Dependencies {
    working_directory: Vec<String>,

    shell: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct AllOfOneOf {
    #[serde(rename = "type")]
    one_of_type: GroupType,

    properties: TentacledProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct TentacledProperties {
    uses: Option<OneOf>,

    run: Option<OneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(derive_setters::Setters, Clone)]
pub struct AllOfProperties {
    id: GroupClass,

    #[serde(rename = "if")]
    properties_if: MaxParallelClass,

    name: GroupClass,

    uses: GroupClass,

    run: GroupClass,

    working_directory: PermissionsValue,

    shell: PermissionsValue,

    with: Branch,

    env: Branch,

    continue_on_error: CancelInProgressClass,

    timeout_minutes: CancelInProgressClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Strategy {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    strategy_type: GroupType,

    properties: StrategyProperties,

    required: Vec<String>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(derive_setters::Setters, Clone)]
pub struct StrategyProperties {
    matrix: PermissionsValue,

    fail_fast: CancelInProgressClass,

    max_parallel: MaxParallelClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct SecretsClass {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    one_of: Vec<PermissionsOneOf>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct PermissionsOneOf {
    #[serde(rename = "type")]
    one_of_type: Option<TypeElement>,

    #[serde(rename = "enum")]
    one_of_enum: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    one_of_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct PermissionsEvent {
    #[serde(rename = "type")]
    permissions_event_type: GroupType,

    additional_properties: bool,

    properties: HashMap<String, PermissionsValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct ReusableWorkflowCallJob {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    reusable_workflow_call_job_type: GroupType,

    properties: ReusableWorkflowCallJobProperties,

    required: Vec<String>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct ReusableWorkflowCallJobProperties {
    name: GroupClass,

    needs: PermissionsValue,

    permissions: PermissionsValue,

    #[serde(rename = "if")]
    properties_if: CancelInProgressClass,

    uses: ExpressionSyntax,

    with: Branch,

    secrets: SecretsClass,

    strategy: Strategy,

    concurrency: CancelInProgressClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Shell {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    any_of: Vec<Event>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(derive_setters::Setters, Clone)]
pub struct ModelProperties {
    name: GroupClass,

    on: On,

    env: Branch,

    defaults: Branch,

    concurrency: CancelInProgressClass,

    jobs: Jobs,

    run_name: ExpressionSyntax,

    permissions: PermissionsValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Jobs {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    jobs_type: GroupType,

    pattern_properties: JobsPatternProperties,

    min_properties: i64,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct JobsPatternProperties {
    #[serde(rename = "^[_a-zA-Z][a-zA-Z0-9_-]*$")]
    a_z_a_z_a_z_a_z0_9_: PurpleAZAZAZAZ09_,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct PurpleAZAZAZAZ09_ {
    one_of: Vec<PermissionsValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct On {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    one_of: Vec<OnOneOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct OnOneOf {
    #[serde(rename = "$ref")]
    one_of_ref: Option<String>,

    #[serde(rename = "type")]
    one_of_type: Option<String>,

    items: Option<PermissionsValue>,

    min_items: Option<i64>,

    properties: Option<StickyProperties>,

    additional_properties: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct StickyProperties {
    branch_protection_rule: Branch,

    check_run: Branch,

    check_suite: Branch,

    create: Branch,

    delete: Branch,

    deployment: Branch,

    deployment_status: Branch,

    discussion: Branch,

    discussion_comment: Branch,

    fork: Branch,

    gollum: Branch,

    issue_comment: Branch,

    issues: Branch,

    label: Branch,

    merge_group: Branch,

    milestone: Branch,

    page_build: Branch,

    project: Branch,

    project_card: Branch,

    project_column: Branch,

    public: Branch,

    pull_request: PullRequest,

    pull_request_review: Branch,

    pull_request_review_comment: Branch,

    pull_request_target: PullRequestTarget,

    push: Push,

    registry_package: Branch,

    release: Branch,

    status: Branch,

    watch: Branch,

    workflow_call: WorkflowCall,

    workflow_dispatch: WorkflowDispatch,

    workflow_run: WorkflowRun,

    repository_dispatch: Branch,

    schedule: ExpressionSyntax,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct PullRequest {
    #[serde(rename = "$comment")]
    comment: String,

    #[serde(rename = "$ref")]
    pull_request_ref: String,

    description: String,

    properties: PullRequestProperties,

    pattern_properties: PullRequestPatternProperties,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct PullRequestPatternProperties {
    #[serde(rename = "^(branche|tag|path)s(-ignore)?$")]
    branche_tag_path_s_ignore: OneOf,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct PullRequestProperties {
    types: Types,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct PullRequestTarget {
    #[serde(rename = "$comment")]
    comment: String,

    #[serde(rename = "$ref")]
    pull_request_target_ref: String,

    description: String,

    properties: PullRequestProperties,

    pattern_properties: PullRequestTargetPatternProperties,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct PullRequestTargetPatternProperties {
    #[serde(rename = "^(branche|tag|path)s(-ignore)?$")]
    branche_tag_path_s_ignore: BranchesIgnore,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct BranchesIgnore {
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Push {
    #[serde(rename = "$comment")]
    comment: String,

    #[serde(rename = "$ref")]
    push_ref: String,

    description: String,

    pattern_properties: PushPatternProperties,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct PushPatternProperties {
    #[serde(rename = "^(branche|tag|path)s(-ignore)?$")]
    branche_tag_path_s_ignore: BrancheTagPathSIgnore,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct WorkflowCall {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    properties: WorkflowCallProperties,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct WorkflowCallProperties {
    inputs: PurpleInputs,

    secrets: WorkflowDispatch,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct PurpleInputs {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    inputs_type: GroupType,

    pattern_properties: PurplePatternProperties,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct PurplePatternProperties {
    #[serde(rename = "^[_a-zA-Z][a-zA-Z0-9_-]*$")]
    a_z_a_z_a_z_a_z0_9_: FluffyAZAZAZAZ09_,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct FluffyAZAZAZAZ09_ {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    a_z_a_z_a_z_a_z0_9___type: GroupType,

    properties: IndigoProperties,

    required: Vec<String>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct IndigoProperties {
    description: GroupClass,

    deprecation_message: Description,

    required: GroupClass,

    #[serde(rename = "type")]
    properties_type: GroupClass,

    #[serde(rename = "default")]
    properties_default: MaxParallelClass,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct Description {
    description: String,

    #[serde(rename = "type")]
    description_type: TypeElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct WorkflowDispatch {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    pattern_properties: Option<WorkflowDispatchPatternProperties>,

    additional_properties: bool,

    properties: Option<WorkflowDispatchProperties>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct WorkflowDispatchPatternProperties {
    #[serde(rename = "^[_a-zA-Z][a-zA-Z0-9_-]*$")]
    a_z_a_z_a_z_a_z0_9_: TentacledAZAZAZAZ09_,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct TentacledAZAZAZAZ09_ {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    properties: IndecentProperties,

    required: Vec<String>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct IndecentProperties {
    description: Description,

    required: ExpressionSyntax,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct WorkflowDispatchProperties {
    inputs: FluffyInputs,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct FluffyInputs {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    inputs_type: GroupType,

    pattern_properties: FluffyPatternProperties,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct FluffyPatternProperties {
    #[serde(rename = "^[_a-zA-Z][a-zA-Z0-9_-]*$")]
    a_z_a_z_a_z_a_z0_9_: StickyAZAZAZAZ09_,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct StickyAZAZAZAZ09_ {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,

    #[serde(rename = "type")]
    a_z_a_z_a_z_a_z0_9___type: GroupType,

    properties: HilariousProperties,

    all_of: Vec<AZAZAZAZ09__AllOf>,

    required: Vec<String>,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct AZAZAZAZ09__AllOf {
    #[serde(rename = "if")]
    all_of_if: AllOfIf,

    then: Then,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct AllOfIf {
    properties: IfProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct IfProperties {
    #[serde(rename = "type")]
    properties_type: TypeClass,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct TypeClass {
    #[serde(rename = "const")]
    type_const: String,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct Then {
    properties: Option<ThenProperties>,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct ThenProperties {
    #[serde(rename = "default")]
    properties_default: OneOf,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct HilariousProperties {
    description: GroupClass,

    deprecation_message: Description,

    required: GroupClass,

    #[serde(rename = "default")]
    properties_default: DefaultClass,

    #[serde(rename = "type")]
    properties_type: GroupClass,

    options: ExpressionSyntax,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct DefaultClass {
    #[serde(rename = "$comment")]
    comment: String,

    description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct WorkflowRun {
    #[serde(rename = "$comment")]
    comment: String,

    #[serde(rename = "$ref")]
    workflow_run_ref: BranchRef,

    description: String,

    properties: WorkflowRunProperties,

    pattern_properties: WorkflowRunPatternProperties,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct WorkflowRunPatternProperties {
    #[serde(rename = "^branches(-ignore)?$")]
    branches_ignore: BranchesIgnore,
}

#[derive(Serialize, Deserialize)]
#[derive(derive_setters::Setters, Clone)]
pub struct WorkflowRunProperties {
    types: Types,

    workflows: Workflows,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(derive_setters::Setters, Clone)]
pub struct Workflows {
    #[serde(rename = "type")]
    workflows_type: String,

    items: OneOf,

    min_items: i64,
}
