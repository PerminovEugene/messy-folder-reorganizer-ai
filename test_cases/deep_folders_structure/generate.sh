#!/bin/bash

# Define the messy folder path
MESSY_FOLDER="messy-folder"

# Cleanup old test folder (if exists) and recreate
rm -rf "$MESSY_FOLDER"
mkdir -p "$MESSY_FOLDER"

# Create deep folders structure

mkdir -p "$MESSY_FOLDER/books"
mkdir -p "$MESSY_FOLDER/books/random"

# Create realistic messy files inside the messy folder
# books

touch "$MESSY_FOLDER/dostoevsky.txt"
touch "$MESSY_FOLDER/unknown_book.fb2"
touch "$MESSY_FOLDER/books/master-and-margarita.fb2"
touch "$MESSY_FOLDER/books/random/cooking-recepies.fb2"
touch "$MESSY_FOLDER/books/random/lord_of_the_rings.fb2"

# root level
touch "$MESSY_FOLDER/home.app"

# Define the structured output folder (empty but categorized)
STRUCTURED_FOLDER="home"

# Cleanup old structured folder and recreate
rm -rf "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER/books"
mkdir -p "$STRUCTURED_FOLDER/books/classic"
mkdir -p "$STRUCTURED_FOLDER/books/fantasy"

# Print both folder structures
echo "Messy folder structure:"
tree "$MESSY_FOLDER"

echo -e "\nIdeal empty folder structure:"
tree "$STRUCTURED_FOLDER"
