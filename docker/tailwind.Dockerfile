ARG tw_config_path
FROM node:current-alpine

# Create the npm package.json
# ----------------------------------------------------------------
WORKDIR /
RUN mkdir /tailwind
WORKDIR /tailwind

# Install tailwindcss
# ----------------------------------------------------------------
RUN echo y | npm install -g npm@latest
RUN echo y | npm install -g \
  tailwindcss@latest \
  postcss@latest \
  autoprefixer@latest \
  clean-css-cli@latest \
  browserlist@latest \
  npm-run@latest

ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/tailwind/node_modules/clean-css-cli/bin

# For some reason we have to run this to get it to install properly
RUN echo y | npx tailwindcss-cli@latest

# Create volumes
# ----------------------------------------------------------------
# Where the source code will be
VOLUME /tailwind/input
# Where the final tailwind css file will be written
VOLUME /tailwind/output

COPY scripts/tailwind.build.sh .

# By placing this config @ /tailwind/input/tailwind.config.js
# its "purge" paths will remain relative
COPY $tw_config_path input/tailwind.config.js
