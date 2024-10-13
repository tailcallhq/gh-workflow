import {
  quicktype,
  InputData,
  JSONSchemaInput,
  FetchingJSONSchemaStore,
  RustTargetLanguage,
  rustOptions,
} from "quicktype-core";
import * as fs from "fs/promises";

// Import node-fetch for Node.js environments
import fetch from "node-fetch";

async function main() {
  // Fetch the schema from the URL
  const response = await fetch(
    "https://raw.githubusercontent.com/SchemaStore/schemastore/refs/heads/master/src/schemas/json/github-workflow.json"
  );

  // Parse the JSON schema
  const schema = await response.text();

  // Create a new JSONSchemaInput and add the schema
  const schemaInput = new JSONSchemaInput(new FetchingJSONSchemaStore());
  await schemaInput.addSource({ name: "Workflow", schema });

  const inputData = new InputData();
  inputData.addInput(schemaInput);

  // Call quicktype to generate Rust code from the schema
  const { lines } = await quicktype({
    inputData,
    lang: new RustTargetLanguage(),

    // Special Rust options
    // https://github.com/glideapps/quicktype/blob/master/packages/quicktype-core/src/language/Rust/language.ts#L9
    rendererOptions: {
      "derive-clone": true,
      "derive-debug": true,
      "skip-serializing-none": true,
      "derive-partial-eq": true,
      visibility: "public",
      "leading-comments": false,
    },
  });

  let code = lines.join("\n");
  // let derivedCode = addDeriveToStructsAndEnums(code);
  await fs.writeFile("workspace/gh-workflow-rs/src/model.rs", code);
}

// Run the main function
main().catch((error) => console.error(error));
