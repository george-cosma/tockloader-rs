#!/bin/bash

TEMPLATE="docs/book.template.toml"
OUTPUT="docs/book.toml"

if [ "$1" = "preview" ]; then
  SITE_URL="/tockloader-rs/pr-preview/$2/"
else
  SITE_URL="./"
fi

sed "s|{{SITE_URL}}|$SITE_URL|g" "$TEMPLATE" > "$OUTPUT"
