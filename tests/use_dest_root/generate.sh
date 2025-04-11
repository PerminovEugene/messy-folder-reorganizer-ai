#!/bin/bash

# Define the messy folder path
MESSY_FOLDER="messy-folder"

# Cleanup old test folder (if exists) and recreate
rm -rf "$MESSY_FOLDER"
mkdir -p "$MESSY_FOLDER"

# Create realistic messy files inside the messy folder for dest root
# movies
touch "$MESSY_FOLDER/harry-potter.mpeg4"
touch "$MESSY_FOLDER/blade-runner.blueray"
touch "$MESSY_FOLDER/dungeons-and-dragons-2000.avi"
touch "$MESSY_FOLDER/dungeons and dragons 2000.avi"

# Define the structured output folder (empty but categorized)
STRUCTURED_FOLDER="movies"

# Cleanup old structured folder and recreate
rm -rf "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER"

# Print both folder structures
echo "Messy folder structure:"
tree "$MESSY_FOLDER"

echo -e "\nIdeal empty folder structure:"
tree "$STRUCTURED_FOLDER"
