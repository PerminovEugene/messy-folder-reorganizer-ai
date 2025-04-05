#!/bin/bash

# Define the messy folder path
MESSY_FOLDER="messy-folder"

# Cleanup old test folder (if exists) and recreate
rm -rf "$MESSY_FOLDER"
mkdir -p "$MESSY_FOLDER"

# Create deep folders structure
mkdir -p "$MESSY_FOLDER/Downloads"

# Create realistic messy files inside the messy folder
touch "$MESSY_FOLDER/Downloads/gov_plan_2025.docx"
touch "$MESSY_FOLDER/Downloads/invoice_for_car.pdf"
touch "$MESSY_FOLDER/Downloads/locked_doc.pdf"

# # Make secret_doc_no_rights_to_update.fb2 read-only (no write/rename permissions), it 
# chmod 400 "$MESSY_FOLDER/Downloads/no_rights/secret_doc.pdf"
# chmod 400 "$MESSY_FOLDER/Downloads/no_rights"

# Define the structured output folder (empty but categorized)
STRUCTURED_FOLDER="home"

# Cleanup old structured folder and recreate
rm -rf "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER/documents"

# Print both folder structures
echo "Messy folder structure:"
tree "$MESSY_FOLDER"

echo -e "\nIdeal empty folder structure:"
tree "$STRUCTURED_FOLDER"
