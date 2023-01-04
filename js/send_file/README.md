# simple server to accept file input

## iOS automatically convert HEIC image to JPEG

* Run the server
* From your iPhone, send HEIC image
* You can confirm that the log says the server received jpeg file
* From your laptop, send HEIC image
* You can confirm that the log says the server received HEIC file

## Multipart form-data

* Run the server
* With postman, send the following request:

```curl
curl --location --request POST 'localhost:4000/upload' \
--header 'Cookie: JSESSIONID=ED9259F09FCB7BC0E9882FEC50838B18' \
--form 'part1="foo"' \
--form 'part3="bar"'
```

* Monitor Loopback network with wireshark, then you can confirm
* it sends one HTTP protocol request, with many TCP request conveying data fragments