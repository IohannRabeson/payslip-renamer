# payslip-renamer
Every two weeks I download a payslip and I rename it with the pay date: YYYY-MM-DD.pdf.
This daemon automates the renaming part. It watches a directory, detect new files, parse the date and rename the file.

## Requirements
This program run on MacOS.

## How it works
It takes a PDF and extract all the text, then uses a regex to find the payslip date.
Syntax: https://docs.rs/regex/latest/regex/#syntax
