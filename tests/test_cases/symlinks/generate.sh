# Define the messy folder path
MESSY_FOLDER="messy-folder"

# Cleanup old test folder (if exists) and recreate
rm -rf "$MESSY_FOLDER"
mkdir -p "$MESSY_FOLDER"

# Create deep folder structure
mkdir -p "$MESSY_FOLDER/Downloads"

# Create realistic messy files inside the messy folder
touch "$MESSY_FOLDER/Downloads/doc.docx"

# Add a file to be targeted by a symlink
touch "$MESSY_FOLDER/Downloads/target.txt"

# Create a valid symlink
ln -s "target.txt" "$MESSY_FOLDER/Downloads/link-to-target.txt"

# Create a broken symlink
ln -s "nonexistent.txt" "$MESSY_FOLDER/Downloads/broken-link.txt"

# Define the structured output folder
STRUCTURED_FOLDER="home"

# Cleanup old structured folder and recreate
rm -rf "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER"
mkdir -p "$STRUCTURED_FOLDER/documents"

# Add file name duplicates
touch "$STRUCTURED_FOLDER/documents/doc.docx"
touch "$STRUCTURED_FOLDER/documents/doc (1).docx"
touch "$STRUCTURED_FOLDER/documents/doc (2).docx"

# Create a symlink in structured folder pointing to one of the duplicates
ln -s "doc (2).docx" "$STRUCTURED_FOLDER/documents/symlink-to-doc2.docx"

# Print folder structures
echo "Messy folder structure:"
tree -l --noreport "$MESSY_FOLDER"

echo -e "\nIdeal empty folder structure:"
tree -l --noreport "$STRUCTURED_FOLDER"
