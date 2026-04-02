#!/bin/bash

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
PROJECT_ROOT=$(dirname "$SCRIPT_DIR")
ENV_FILE="$PROJECT_ROOT/.env"

cd "$PROJECT_ROOT" || exit

if [ -f ".env" ]; then
    echo "Loading variables from $ENV_FILE..."

    while IFS= read -r line || [ -n "$line" ]; do
        line=$(echo "$line" | xargs)
        if [[ -n "$line" && ! "$line" =~ ^# ]]; then
            export "$line"
        fi
    done < ".env"   
else
    echo "Error: .env file not found at $ENV_FILE"
    exit 1
fi

if [ -z "$DATABASE_URL" ]; then
    echo "Error: DATABASE_URL is not defined in .env"
    exit 1
fi

sea generate entity -u "$DATABASE_URL" -o generated
