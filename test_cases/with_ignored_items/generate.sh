#!/bin/bash

# Define the messy folder path
MESSY_FOLDER="messy-folder"

# Cleanup old test folder (if exists) and recreate
rm -rf "$MESSY_FOLDER"
mkdir -p "$MESSY_FOLDER"

mkdir -p "$MESSY_FOLDER/node_modules"

# Create realistic messy files inside the messy folder for dest root
# movies
touch "$MESSY_FOLDER/.git"
touch "$MESSY_FOLDER/node_modules/lib.js"
touch "$MESSY_FOLDER/temp_1221.log"
touch "$MESSY_FOLDER/harry_potter.txt"

# Define the structured output folder (empty but categorized)
STRUCTURED_FOLDER="dest"

# Cleanup old structured folder and recreate
rm -rf "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER"

mkdir -p "$STRUCTURED_FOLDER/Projects"
mkdir -p "$STRUCTURED_FOLDER/books"

# Print both folder structures
echo "Messy folder structure:"
tree "$MESSY_FOLDER"

echo -e "\nIdeal empty folder structure:"
tree "$STRUCTURED_FOLDER"
