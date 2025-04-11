#!/bin/bash

# Define the messy folder path
MESSY_FOLDER="messy-folder"

# Cleanup old test folder (if exists) and recreate
rm -rf "$MESSY_FOLDER"
mkdir -p "$MESSY_FOLDER"

# Create realistic messy files inside the messy folder for dest root

# movies
touch "$MESSY_FOLDER/harry-potter.mpeg4"
touch "$MESSY_FOLDER/lord-of-the-ring.avi"

# books
touch "$MESSY_FOLDER/lovecraft-novels.txt"
touch "$MESSY_FOLDER/hobbit.fb2"

# documents
touch "$MESSY_FOLDER/invoice_from_google.pdf"
touch "$MESSY_FOLDER/police_report.docx"

# images
touch "$MESSY_FOLDER/meme.png"
touch "$MESSY_FOLDER/funny-cat.jpg"

# apps
touch "$MESSY_FOLDER/crack.exe"
touch "$MESSY_FOLDER/database-admin.pkg"

# Define the structured output folder (empty but categorized)
STRUCTURED_FOLDER="dest"

# Cleanup old structured folder and recreate
rm -rf "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER"

mkdir -p "$STRUCTURED_FOLDER/movies"
mkdir -p "$STRUCTURED_FOLDER/docs"
mkdir -p "$STRUCTURED_FOLDER/images"
mkdir -p "$STRUCTURED_FOLDER/books"
mkdir -p "$STRUCTURED_FOLDER/apps"

# Print both folder structures
echo "Messy folder structure:"
tree "$MESSY_FOLDER"

echo -e "\nIdeal empty folder structure:"
tree "$STRUCTURED_FOLDER"
