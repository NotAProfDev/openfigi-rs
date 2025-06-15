#!/bin/bash
# gen_enum.sh: Generate Rust enum from OpenFIGI API
# Usage: ./gen_enum.sh <url> <EnumName>
# Example: ./gen_enum.sh "https://api.openfigi.com/v3/mapping/values/securityType" SecurityType
#
# Notes:
# - This script requires curl, jq, and awk.
# - It strips spaces, underscores, dashes, dots, slashes, commas and parentheses from variant names, and replaces '&' with 'AND'.
# - If the variant name starts with a digit, an underscore is prepended to make it a valid Rust identifier.
# - The special value '***' is mapped to the variant 'AAA'.
# - You can redirect output to a file: ./gen_enum.sh <url> <EnumName> > MyEnum.rs

set -euo pipefail

# Check for correct number of arguments
if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <url> <EnumName>" >&2
  exit 1
fi

URL="$1"       # The OpenFIGI API endpoint to fetch values from
ENUM_NAME="$2" # The name of the Rust enum to generate

# Fetch values from the API and generate Rust enum code
curl -s "$URL" |
  jq -r --arg ENUM_NAME "$ENUM_NAME" '
        .values[] as $v |
        if $v == "***" then
          "    #[serde(rename = \"***\")]\n    AAA,"
        else
          "    #[serde(rename = \"" + $v + "\")]\n" +
          "    " +
          ( ($v | gsub("[ _\\-./,()]+"; "") | gsub("&"; "AND") | capture("^(?<first>.)?(?<rest>.*)$") | if (.first | test("^[0-9]$")) then "_" + ((.first | ascii_upcase) + .rest) else (.first | ascii_upcase) + .rest end ) ) + ","
        end
    ' |
  awk -v enum_name="$ENUM_NAME" '
        BEGIN {
            print "pub enum " enum_name " {"
        }
        { print }
        END {
            print "}"
        }'
