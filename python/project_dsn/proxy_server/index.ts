import { request } from 'http';
import { v4 as uuidv4 } from 'uuid';

import { CONFIG } from './secret';

const server = Bun.serve({
    port: 3000,
    fetch(req) {
        const url = new URL(req.url); 
        if (url.pathname === "/found" && req.method == "POST") {
            return req.json().then(body => {
                sendLineMessage(body.message);
                return new Response("Home page!");
            }).catch(e => {
                return new Response('Invalid request', { status: 400 })
            })
        }
        return new Response("Bun!");
    },
  });

const sendLineMessage = (message: String) => {
    const url = 'https://api.line.me/v2/bot/message/push';
    const data = {
        "to": CONFIG.userId,
        "messages": [
            {"type": "text", "text": message}
        ]
    };

    const headers = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${CONFIG.channelAccessToken}`,
        'X-Line-Retry-Key': uuidv4()
    };

    fetch(url, {
        method: 'POST',
        headers: headers,
        body: JSON.stringify(data)
    })
    .then(response => response.json())
    .then(data => console.log('Success:', data))
    .catch((error) => console.error('Error:', error));
}

console.log(`Listening on http://localhost:${server.port} ...`);