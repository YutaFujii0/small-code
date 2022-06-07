FROM rust:1-buster

WORKDIR /app

COPY package.json ./

RUN apt-get update -qq \
  && curl -sL https://deb.nodesource.com/setup_14.x | bash -\
  && apt-get install -y nodejs \
  && apt-get clean

COPY . .
