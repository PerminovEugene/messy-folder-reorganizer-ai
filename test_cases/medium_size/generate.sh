#!/bin/bash

# Define the messy folder path
MESSY_FOLDER="messy-folder"

# Cleanup old test folder (if exists) and recreate
rm -rf "$MESSY_FOLDER"
mkdir -p "$MESSY_FOLDER"

# Create realistic messy files inside the messy folder
mkdir -p "$MESSY_FOLDER/agdg44"
touch "$MESSY_FOLDER/agdg44/invoice-march.pdf"
touch "$MESSY_FOLDER/agdg44/invoice_2024-02.txt"
touch "$MESSY_FOLDER/Doge_Meme_Collection.gif"
touch "$MESSY_FOLDER/harry_potter_philosophers_stone.mpeg4"
touch "$MESSY_FOLDER/old_tax_report_2019.doc"
touch "$MESSY_FOLDER/holiday_trip.mp4"
touch "$MESSY_FOLDER/presentation_final.pptx"
touch "$MESSY_FOLDER/finance-budget.xlsx"
touch "$MESSY_FOLDER/sales_data.csv"
touch "$MESSY_FOLDER/install_macOS_sonoma.dmg"
touch "$MESSY_FOLDER/linux-arch.iso"
touch "$MESSY_FOLDER/crypto-trading-bot.py"
touch "$MESSY_FOLDER/tetris-clone.rs"
touch "$MESSY_FOLDER/worms2d.tar.gz"
touch "$MESSY_FOLDER/hollow-knight-save.zip"
touch "$MESSY_FOLDER/rainy_jazz.mp3"
touch "$MESSY_FOLDER/classical_symphony.wav"
touch "$MESSY_FOLDER/deep_space_photo.jpeg"
touch "$MESSY_FOLDER/family-vacation-2023.png"
touch "$MESSY_FOLDER/cool_wallpaper.bmp"
touch "$MESSY_FOLDER/photo_edit.psd"
touch "$MESSY_FOLDER/fancy_resume.pdf"
touch "$MESSY_FOLDER/memes_for_friday.jpeg"
touch "$MESSY_FOLDER/quicknote.txt"
touch "$MESSY_FOLDER/suspicious_file.bin"

# Define the structured output folder (empty but categorized)
STRUCTURED_FOLDER="structured-folder"

# Cleanup old structured folder and recreate
rm -rf "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER"

# Define categories for empty structure
declare -a FOLDERS=(
    "books"
    "documents"
    "documents/invoices"
    "fun-memes"
    "movies"
    "pictures"
    "music"
    "archives"
    "code"
    "spreadsheets"
    "system"
    "executables"
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
