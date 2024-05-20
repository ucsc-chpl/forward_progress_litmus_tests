#!/bin/bash

# Check if directory is provided as an argument
if [ -z "$1" ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

# Directory to iterate over
DIRECTORY=$1

# Find and iterate over all .wgsl files in the directory and nested directories
find "$DIRECTORY" -type f -name "*.wgsl" | while read -r file; do
    # echo "Validating $file ..."
    # Run naga validate on the file
    echo naga validate "$file"
    # Check the exit status of the last command
    if [ $? -eq 0 ]; then
        echo "$file is valid."
    else
        echo "$file is invalid."
    fi
done
