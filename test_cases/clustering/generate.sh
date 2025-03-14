#!/bin/bash

# Define the messy folder path
MESSY_FOLDER="messy-folder"

# Cleanup old test folder (if exists) and recreate
rm -rf "$MESSY_FOLDER"
mkdir -p "$MESSY_FOLDER"

# Create realistic messy files inside the messy folder
mkdir -p "$MESSY_FOLDER/agdg44"
# books
touch "$MESSY_FOLDER/how-to-train-dragon.txt"
touch "$MESSY_FOLDER/master-and-margarita.fb2"
# documents
touch "$MESSY_FOLDER/residence-permit.pdf"
touch "$MESSY_FOLDER/bill.docx"

# Unknown
# 3 clusters:

# 10 Executable files (various formats)
touch "$MESSY_FOLDER/game.exe"
touch "$MESSY_FOLDER/editor.app"
touch "$MESSY_FOLDER/installer.pkg"
touch "$MESSY_FOLDER/script.sh"
touch "$MESSY_FOLDER/run.bat"
touch "$MESSY_FOLDER/utility.bin"
touch "$MESSY_FOLDER/software.deb"
touch "$MESSY_FOLDER/program.msi"
touch "$MESSY_FOLDER/system.out"
touch "$MESSY_FOLDER/autoupdater.run"

# 15 Audio files (various formats)
touch "$MESSY_FOLDER/rock.mp3"
touch "$MESSY_FOLDER/classical.wav"
touch "$MESSY_FOLDER/jazz.flac"
touch "$MESSY_FOLDER/pop.ogg"
touch "$MESSY_FOLDER/metal.aac"
touch "$MESSY_FOLDER/radio.m4a"
touch "$MESSY_FOLDER/interview.opus"
touch "$MESSY_FOLDER/podcast.aiff"
touch "$MESSY_FOLDER/blues.3gp"
touch "$MESSY_FOLDER/orchestra.mid"
touch "$MESSY_FOLDER/electronic.mka"
touch "$MESSY_FOLDER/folk.webm"
touch "$MESSY_FOLDER/reggae.tta"
touch "$MESSY_FOLDER/dubstep.wma"
touch "$MESSY_FOLDER/lofi.alac"

# 7 Programming-related files
touch "$MESSY_FOLDER/main.rs"
touch "$MESSY_FOLDER/app.py"
touch "$MESSY_FOLDER/index.js"
touch "$MESSY_FOLDER/module.cpp"
touch "$MESSY_FOLDER/library.java"
touch "$MESSY_FOLDER/script.ts"
touch "$MESSY_FOLDER/program.cs"



# Define the structured output folder (empty but categorized)
STRUCTURED_FOLDER="structured-folder"

# Cleanup old structured folder and recreate
rm -rf "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER"

# Define categories for empty structure
declare -a FOLDERS=(
    "books"
    "documents"
)

# Create empty categorized folders
for folder in "${FOLDERS[@]}"; do
    mkdir -p "$STRUCTURED_FOLDER/$folder"
done

# Print both folder structures
echo "Messy folder structure:"
tree "$MESSY_FOLDER"

echo -e "\nIdeal empty folder structure:"
tree "$STRUCTURED_FOLDER"
