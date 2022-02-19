# Overall Flow

# base -> gem_builder -> main_base -> job
#             |                 |
#             └-> node_builder -┷---> web

# -------------------------------------
FROM ruby:2.7.2 AS base
ENV LANG C.UTF-8
ENV RAILS_ENV production
RUN mkdir /app
WORKDIR /app

# -------------------------------------
# install gems
FROM base AS gem_builder
COPY Gemfile Gemfile.lock ./

RUN bundle config set without 'test development' \
  && bundle install \
  && rm -rf $GEM_HOME/cache

# -------------------------------------
# install node packages to precompile static assets
FROM base AS node_builder
COPY package.json yarn.lock ./
RUN apt-get update -qq \
  && curl -sL https://deb.nodesource.com/setup_14.x | bash -\
  && apt-get install -y nodejs \
  && apt-get clean \
  && npm install -g yarn

COPY --from=gem_builder /usr/local/bundle /usr/local/bundle
COPY . /app
RUN yarn install --production
RUN bundle exec rails assets:precompile

# -------------------------------------
# base of app server
FROM base AS main_base
COPY . /app
COPY --from=gem_builder /usr/local/bundle /usr/local/bundle

# -------------------------------------
# web
FROM main_base AS web
RUN apt-get update -qq \
  && apt-get install -y mariadb-client fonts-ipafont-gothic fonts-ipafont-mincho \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/

COPY --from=node_builder /app/public/assets public/assets

ENTRYPOINT ["bash", "entrypoint.sh"]
EXPOSE 3000

# -------------------------------------
# web2
FROM main_base AS job
RUN apt-get update -qq \
  && apt-get install -y build-essential libpq-dev mariadb-client \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/

ENTRYPOINT ["bash", "entrypoint2.sh"]
EXPOSE 8888
