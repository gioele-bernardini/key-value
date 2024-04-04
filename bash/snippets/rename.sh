#!/usr/bin/bash

rename_notes_to_md() {
  for file in "$1"/*; do
    if [[ -d "$file" ]]; then
      rename_notes_to_md "$file"
    elif [[ -f "$file" && $(basename "$file") == "NOTES.txt" ]]; then
      new_file="${file%/*}/NOTES.md"
      mv "$file" "$new_file"
      echo "File renamed: $file => $new_file"
    fi
  done
}

folder_path=$(pwd)
rename_notes_to_md "$folder_path"