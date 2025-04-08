#!/bin/bash

# Define the messy folder path
MESSY_FOLDER="messy-folder"

# Cleanup old test folder (if exists) and recreate
rm -rf "$MESSY_FOLDER"
mkdir -p "$MESSY_FOLDER"

# Create deep folders structure
mkdir -p "$MESSY_FOLDER/Downloads"

# Create realistic messy files inside the messy folder
touch "$MESSY_FOLDER/Downloads/doc.docx"

# Define the structured output folder (empty but categorized)
STRUCTURED_FOLDER="home"

# Cleanup old structured folder and recreate
rm -rf "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER/documents"


# add file name duplicates
touch "$STRUCTURED_FOLDER/documents/doc.docx"
touch "$STRUCTURED_FOLDER/documents/doc (1).docx"
touch "$STRUCTURED_FOLDER/documents/doc (2).docx"


# Print both folder structures
echo "Messy folder structure:"
tree "$MESSY_FOLDER"

echo -e "\nIdeal empty folder structure:"
tree "$STRUCTURED_FOLDER"
