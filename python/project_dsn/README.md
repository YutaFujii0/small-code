This directory contains three pet projects:
1. selenium chrome scraper (`app.py`)
2. chrome extension (`extension` dir)
3. a proxy server to LINE notification (`proxy_server` dir)

All of these aims to make reservation efficient for TDH.

At this moment, 1 is still WIP, but you can use 2 and 3.

# How to use 2 and 3

## Prerequisite
- LINE Developer account
  - create a developer account for LINE
  - create a Messaging API channel
  - generate channel access token
  - setup a webhook URL, it's useful to use webhook.site
  - scan the QR code of Messaging API channel by your LINE account
  - confirm the webhook event and read userId
- ngrok
  - create an account


## Setup local server

```bash
$ cd <path to proxy_server>

$ cp secret.ts.example secret.ts
# and set channelAccessToken and userId

# start your local server
$ bun start

# connect your local to the endpoint exposed to the internet
$ ngrok http 3000 --response-header-add "Access-Control-Allow-Origin: *"
```

## Setup chrome extension

```bash
$ cd <path to extension>

$ cp config.js.example config.js
# and fill `webhook` value, you can see that in the console running ngrok
```

1. open chrome://extensions/
2. enable developer mode
3. click 'Load unpacked'
4. select `extension` dir

It is recommended to install an extension to reload the page frequently 
(e.g. Auto Refresh & Page Monitor)


# How to use 1 (WIP)

Check your Chrome version

Install corresponding chromedriver

Try this page first
https://chromedriver.chromium.org/downloads

If your chrome version is quite new, it might be possible that the above site doesn't have it. In that case, try:
https://googlechromelabs.github.io/chrome-for-testing/#stable


Make your driver approved:
```
xattr -d com.apple.quarantine ./chromedriver
```
