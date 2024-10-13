import re

def add_derive_to_structs_and_enums(file_path):
    # Open the file and read the content
    with open(file_path, 'r') as file:
        content = file.read()

    # Regular expression to match Rust struct definitions
    struct_pattern = r"(?m)^\s*(pub\s+)?struct\s+\w+"
    # Regular expression to match Rust enum definitions
    enum_pattern = r"(?m)^\s*(pub\s+)?enum\s+\w+"

    # Function to add #[derive(derive_setters::Setters, Clone)] above structs
    def add_derive_to_struct(match):
        return "#[derive(derive_setters::Setters, Clone)]\n" + match.group(0)

    # Function to add #[derive(Clone)] above enums
    def add_derive_to_enum(match):
        return "#[derive(Clone)]\n" + match.group(0)

    # Substitute all struct definitions with the added derive attributes
    updated_content = re.sub(struct_pattern, add_derive_to_struct, content)

    # Substitute all enum definitions with the added derive attribute
    updated_content = re.sub(enum_pattern, add_derive_to_enum, updated_content)

    # Write the updated content back to the file
    with open(file_path, 'w') as file:
        file.write(updated_content)

    print(f"Updated the file: {file_path}")

# Specify the path to your .rs file
file_path = 'workspace/gh-workflow-rs/src/model.rs'
add_derive_to_structs_and_enums(file_path)

