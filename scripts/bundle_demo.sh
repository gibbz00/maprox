#!/bin/sh
set -e

MAPROX_APPLICATION="maprox-application"
MAPROX_DEMO="maprox-demo"

mkdir -p dist
trunk build --release --public-url "./" --dist "dist" --filehash false "$MAPROX_DEMO/index.html"
trunk build --release --public-url "./" --dist "dist/$MAPROX_APPLICATION" --filehash false "$MAPROX_APPLICATION/index.html"

sed -i "s/\$MAPROX_APPLICATION_DIST/\.\/$MAPROX_APPLICATION\/index.html/g" "dist/index.html"

# WORKAROUND: Until https://github.com/thedodd/trunk/pull/470 lands. 
find "dist" -type f -name '*.html' -print | xargs sed -i 's,/'\.'/,./,g'
