#!/bin/bash

mkdir -p data

curl https://raw.githubusercontent.com/ao-data/ao-bin-dumps/master/formatted/world.json > data/locations.json
curl https://raw.githubusercontent.com/ao-data/ao-bin-dumps/master/items.json > data/items.json
curl https://raw.githubusercontent.com/ao-data/ao-bin-dumps/master/formatted/items.json > data/localizations.json
