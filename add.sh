#!/bin/bash

# Check if an argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

# Assign the day number from the command line argument
day_number=$1

# Copy template files
cp src/bin/day_template.rs src/bin/day"${day_number}"_1.rs
cp src/bin/day_template.rs src/bin/day"${day_number}"_2.rs

# Create a new file
touch assets/day"${day_number}".txt

# Perform text replacements using sed
sed -i "s/test.txt/day${day_number}.txt/g" src/bin/day"${day_number}"_1.rs
sed -i "s/test.txt/day${day_number}.txt/g" src/bin/day"${day_number}"_2.rs
sed -i "s/test_out.txt/day${day_number}_1_out.txt/g" src/bin/day"${day_number}"_1.rs
sed -i "s/test_out.txt/day${day_number}_2_out.txt/g" src/bin/day"${day_number}"_2.rs

echo "Script executed successfully for day ${day_number}"
