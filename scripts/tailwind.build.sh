#!/bin/sh
echo "Building tailwind.big.css"
npx tailwindcss build \
  --input input/styles/index.css \
  --output output/tailwind.big.css \
  --config input/tailwind.config.js \
  # --minify \

# echo "Removing old output/tailwind.css"
# rm /tailwind/output/tailwind.css

echo "Shrinking..."
cleancss output/tailwind.big.css --output output/tailwind.css

echo "Removing output/tailwind.big.css"
rm output/tailwind.big.css
