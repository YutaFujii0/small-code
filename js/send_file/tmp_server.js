const http = require('http');
const fs = require('fs');
const path = require('path');

const server = http.createServer((req, res) => {
  if (req.url === '/') {
    res.writeHead(200, { 'Content-Type': 'text/html' });
    res.end(`
      <form action="/upload" enctype="image/jpeg" method="post">
        <input type="file" name="someCoolFiles"><br>
        <button>Upload</button>
      </form>
    `);
  } else if (req.url === '/upload') {
    let body = ''
    req.on('data', (chunk) => {
        body += chunk;
    });
    req.on('end', () => {
        console.log(body);
        res.write('OK'); 
        res.end(); 
    });
  } else {
    res.writeHead(404, { 'Content-Type': 'text/plain' });
    res.end('404');
  }
});

server.listen(4000, () => {
  console.log('Server listening on http://localhost:4000 ...');
});