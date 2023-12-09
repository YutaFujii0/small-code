function sendLineMessage() {
    const url = config.webhook;
    
    fetch(url, {
        method: 'GET'
    })
    .then(response => console.log('Success'))
    .catch((error) => console.error('Error:', error));
}

function sendLineMessage(message) {
    const data = { message };

    const headers = {
        'Content-Type': 'application/json'
    };

    fetch(config.webhook, {
        method: 'POST',
        herders: headers,
        body: JSON.stringify(data)
    })
    .then(_ => console.log('Success'))
    .catch((error) => console.error('Error:', error));
}

function checkElement() {
    const loading = document.querySelector(config.loadingSelector);
    const atLeastOneRoom = document.querySelector(config.atLeastOneSelector);
    const noRoom = document.querySelector(config.noRoomSelector);
    const date = document.querySelector('#useDateParam').value;

    if (loading) {
        retry();
        return;
    } else if (noRoom) {
        return;
    } else if (!atLeastOneRoom) {
        // when loading icon disappears but still room availablity is loading
        retry();
        return;
    }
    
    const hits = config.target
        .filter(target => document.querySelector(target.selector))
        .map(target => target.roomName)
    
    if (hits.length > 0) {
        sendLineMessage(`${date}空室があります： ${hits.join(', ')}`);
        return;
    } else {
        retry();
        return;
    }
}

function retry() {
    setTimeout(checkElement, config.intervalMillisec);
}

checkElement();
