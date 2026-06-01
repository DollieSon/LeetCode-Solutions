#!/bin/bash

# HOW TO RUN
# DO 'wsl sh ./create-sol.sh <leecode-number>'
if [ -z "$1" ]; then
    echo "ERROR: no number given"
    echo "Usage: $0 <leetcode-number>"
    exit 1
fi

INPUT_STRINGS="$1"

TARGET_FOLDER="../rust-leet/src/Solutions/"
MOD_FILE="../rust-leet/src/Solutions/mod.rs"

touch "$TARGET_FOLDER/leet$INPUT_STRINGS.rs"
cat template.txt > "$TARGET_FOLDER/leet$INPUT_STRINGS.rs"
echo "Created file: $TARGET_FOLDER/leet$INPUT_STRINGS.rs"

echo -e "\npub mod leet$INPUT_STRINGS;" >> "$MOD_FILE"
echo "Updated mod.rs to include: pub mod leet$INPUT_STRINGS;"
