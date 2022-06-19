FROM --platform=linux/amd64 rust:1-buster

WORKDIR /app

COPY package.json ./

RUN apt-get update -qq \
  && curl -sL https://deb.nodesource.com/setup_14.x | bash -\
  && apt-get install -y nodejs \
  && sh -c "$(curl -sSfL https://release.solana.com/v1.10.26/install)" \
  && npm install \
  && apt-get clean

ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

RUN solana config set --url http://host.docker.internal:8899

COPY . .
