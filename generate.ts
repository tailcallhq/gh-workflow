import {
  quicktype,
  InputData,
  JSONSchemaInput,
  FetchingJSONSchemaStore,
} from "quicktype-core";
import * as fs from "fs";

// Import node-fetch for Node.js environments
import fetch from "node-fetch";

// Function to add derive attributes to structs and enums in the Rust file
function addDeriveToStructsAndEnums(content) {
  // Regular expression to match Rust struct definitions
  const structPattern = /^(?:\s*pub\s+)?struct\s+\w+/gm;
  // Regular expression to match Rust enum definitions
  const enumPattern = /^(?:\s*pub\s+)?enum\s+\w+/gm;

  // Add #[derive(derive_setters::Setters, Clone)] above struct definitions
  const updatedStructContent = content.replace(structPattern, (match) => {
    return `#[derive(derive_setters::Setters, Clone)]\n${match}`;
  });

  // Add #[derive(Clone)] above enum definitions
  const updatedEnumContent = updatedStructContent.replace(
    enumPattern,
    (match) => {
      return `#[derive(Clone)]\n${match}`;
    },
  );

  return updatedEnumContent;
}

async function main() {
  // Fetch the schema from the URL
  const response = await fetch(
    "https://raw.githubusercontent.com/SchemaStore/schemastore/refs/heads/master/src/schemas/json/github-workflow.json",
  );

  // Parse the JSON schema
  const schema = await response.text();

  console.log("Generating Rust code...");

  // Create a new JSONSchemaInput and add the schema
  const schemaInput = new JSONSchemaInput(new FetchingJSONSchemaStore());
  await schemaInput.addSource({ name: "Workflow", schema });

  const inputData = new InputData();
  inputData.addInput(schemaInput);

  // Call quicktype to generate Rust code from the schema
  const { lines } = await quicktype({
    inputData,
    lang: "rust",
  });

  let code = lines.join("\n");
  let derivedCode = addDeriveToStructsAndEnums(code);
  fs.writeFileSync("workspace/gh-workflow-rs/src/model.rs", derivedCode);
}

// Run the main function
main().catch((error) => console.error(error));
