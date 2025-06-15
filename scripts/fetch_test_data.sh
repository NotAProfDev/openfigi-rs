#!/bin/bash

# Script to fetch test data from OpenFIGI API endpoints
# This script will curl different examples from the OpenFIGI API and save responses to tests/data/
#
# Usage:
#   ./fetch_test_data.sh                    # Run all examples
#   ./fetch_test_data.sh list               # List available examples
#   ./fetch_test_data.sh <example_name>     # Run specific example
#   ./fetch_test_data.sh isin_example       # Example: run only the ISIN example

set -euo pipefail

# Base URL for OpenFIGI API
BASE_URL="https://api.openfigi.com/v3"

# Create directories for test data
mkdir -p tests/data/mapping
mkdir -p tests/data/search

# Parse command line arguments
SELECTED_EXAMPLE=${1:-"all"}

# Available examples
declare -A EXAMPLES=(
    ["isin_example"]="Simple ISIN lookup"
    ["cusip_with_exchange"]="CUSIP lookup with exchange filter"
    ["ticker_with_security_type"]="Ticker lookup with security type"
    ["bulk_request"]="Multiple identifiers in one request"
    ["option_example"]="Option lookup with strike and expiration"
    ["currency_mic_example"]="Currency and MIC code filter"
    ["invalid_identifier"]="Invalid identifier (error response)"
    ["query_example"]="Query example with multiple fields"
    ["no_data"]="No data found example (empty response)"
)

# Function to list available examples
list_examples() {
    echo "Available examples:"
    for example in "${!EXAMPLES[@]}"; do
        echo "  $example - ${EXAMPLES[$example]}"
    done
    exit 0
}

# Handle command line arguments
if [[ "$SELECTED_EXAMPLE" == "list" ]]; then
    list_examples
fi

echo "Fetching test data from OpenFIGI API..."

# Function to make API requests with proper error handling
make_request() {
    local method=$1
    local endpoint=$2
    local output_file=$3
    local data=${4:-}

    echo "Fetching: $method $endpoint -> $output_file"

    if [ "$method" = "POST" ]; then
        curl -s -X POST \
            -H "Content-Type: application/json" \
            -H "Accept: application/json" \
            -d "$data" \
            "$BASE_URL$endpoint" \
            -o "$output_file" \
            --write-out "HTTP %{http_code}\n" || echo "Request failed"
    else
        curl -s -X GET \
            -H "Accept: application/json" \
            "$BASE_URL$endpoint" \
            -o "$output_file" \
            --write-out "HTTP %{http_code}\n" || echo "Request failed"
    fi

    # Check if response is valid JSON
    if jq empty "$output_file" 2>/dev/null; then
        echo "✓ Valid JSON response saved to $output_file"
    else
        echo "⚠ Response may not be valid JSON (could be error message)"
        cat "$output_file"
        echo ""
    fi

    # Sleep to avoid rate limiting
    echo "Waiting 5 seconds to avoid rate limiting..."
    sleep 5
}

# Function to run a specific example
run_example() {
    local example_name=$1

    case $example_name in
    "isin_example")
        make_request "POST" "/mapping" "tests/data/mapping/isin_example.json" '[
              {
                "idType": "ID_ISIN",
                "idValue": "US4592001014"
              }
            ]'
        ;;
    "cusip_with_exchange")
        make_request "POST" "/mapping" "tests/data/mapping/cusip_with_exchange.json" '[
              {
                "idType": "ID_CUSIP",
                "idValue": "459200101",
                "exchCode": "US"
              }
            ]'
        ;;
    "ticker_with_security_type")
        make_request "POST" "/mapping" "tests/data/mapping/ticker_with_security_type.json" '[
              {
                "idType": "TICKER",
                "idValue": "IBM",
                "securityType2": "Common Stock"
              }
            ]'
        ;;
    "bulk_request")
        make_request "POST" "/mapping" "tests/data/mapping/bulk_request.json" '[
              {
                "idType": "ID_ISIN",
                "idValue": "US4592001014"
              },
              {
                "idType": "TICKER",
                "idValue": "AAPL",
                "securityType2": "Common Stock"
              }
            ]'
        ;;
    "option_example")
        make_request "POST" "/mapping" "tests/data/mapping/option_example.json" '[
              {
                "idType": "TICKER",
                "idValue": "AAPL 01/17/25 C155",
                "securityType2": "Option",
                "optionType": "Call",
                "strike": [150.0, 160.0],
                "expiration": ["2025-01-01", "2025-12-31"]
              }
            ]'
        ;;
    "currency_mic_example")
        make_request "POST" "/mapping" "tests/data/mapping/currency_mic_example.json" '[
              {
                "idType": "ID_ISIN",
                "idValue": "US0378331005",
                "currency": "EUR",
                "micCode": "XETR"
              }
            ]'
        ;;
    "invalid_identifier")
        make_request "POST" "/mapping" "tests/data/mapping/invalid_identifier.json" '[
              {
                "idType": "ID_ISIN",
                "idValue": "INVALID12345"
              }
            ]'
        ;;
    "query_example")
        make_request "POST" "/search" "tests/data/search/query_example.json" '[
              {
                "query": "ibm",
                "exchCode": "US"
              }
            ]'
        ;;
    "no_data")
        make_request "POST" "/search" "tests/data/search/no_data.json" '[
              {
                "query": "INVALIDESTRING_DONTFINDANYTHING",
                "exchCode": "US"
              }
            ]'
        ;;
    *)
        echo "Error: Unknown example '$example_name'"
        echo "Use './fetch_test_data.sh list' to see available examples"
        exit 1
        ;;
    esac
}

echo "=== Fetching /mapping endpoint examples ==="

# Check if we should run all examples or just one
if [[ "$SELECTED_EXAMPLE" == "all" ]]; then
    echo "Running all examples..."
    for example in "${!EXAMPLES[@]}"; do
        echo "Running example: $example (${EXAMPLES[$example]})"
        run_example "$example"
    done
else
    # Check if the example exists
    if [[ -z "${EXAMPLES[$SELECTED_EXAMPLE]:-}" ]]; then
        echo "Error: Unknown example '$SELECTED_EXAMPLE'"
        echo "Use './fetch_test_data.sh list' to see available examples"
        exit 1
    fi

    echo "Running example: $SELECTED_EXAMPLE (${EXAMPLES[$SELECTED_EXAMPLE]})"
    run_example "$SELECTED_EXAMPLE"
fi

echo ""
echo "=== Summary ==="
echo "Test data has been fetched and saved to tests/data/"
echo ""
echo "Mapping endpoint examples:"
ls -la tests/data/mapping/*.json

echo ""
echo "Done! You can now use these files as test data for your Rust tests."
