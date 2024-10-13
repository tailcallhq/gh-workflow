import {
  quicktype,
  InputData,
  jsonInputForTargetLanguage,
  JSONSchemaInput,
  FetchingJSONSchemaStore,
} from "quicktype-core";

async function quicktypeJSONSchema(targetLanguage, typeName, jsonSchemaString) {
  const schemaInput = new JSONSchemaInput(new FetchingJSONSchemaStore());

  // We could add multiple schemas for multiple types,
  // but here we're just making one type from JSON schema.
  await schemaInput.addSource({ name: typeName, schema: jsonSchemaString });

  const inputData = new InputData();
  inputData.addInput(schemaInput);

  return await quicktype({
    inputData,
    lang: targetLanguage,
  });
}

async function main() {
  const json = await fetch(
    "https://raw.githubusercontent.com/SchemaStore/schemastore/refs/heads/master/src/schemas/json/github-workflow.json"
  );
  const schema = await json.json();

  console.log("Generating Rust code...");
  const schemaInput = new JSONSchemaInput(new FetchingJSONSchemaStore());

  // We could add multiple schemas for multiple types,
  // but here we're just making one type from JSON schema.
  await schemaInput.addSource({ name: "Workflow", schema });

  const inputData = new InputData();
  inputData.addInput(schemaInput);

  const { lines } = await quicktype({
    inputData,
    lang: "rust",
  });

  console.log(lines.join("\n"));
}

main();
