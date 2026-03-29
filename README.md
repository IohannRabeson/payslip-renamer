# payslip-renamer
Every two weeks I download a payslip and I rename it with the pay date: `YYYY-MM-DD.pdf`.
This daemon automates the renaming part. It watches a directory, detect new files, parse the date and rename the file.

## Requirements
This program run on MacOS.

## How it works
It takes a PDF and extract all the text, then uses a regex to find the payslip date.
Syntax: https://docs.rs/regex/latest/regex/#syntax
The `monitor` command expects two environment variables:
 - `PAYSLIP_RENAMER_DIRECTORY`: the directory to watch for new payslip
 - `PAYSLIP_RENAMER_DATE_PATTERN`: optional, the pattern to extract the date, it expects three captures, example: `DATE PAYABLE: (\d{4})/(\d{2})/(\d{2})`
