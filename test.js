var http = require('http');

var opts = {
  host: 'localhost',
  port: 6767,
  method: 'POST',
  path: '/post',
  headers: {
    'Content-Type': 'application/json'
  }
}

var req = http.request(opts);
req.write(JSON.stringify({
  id: 1,
  title: 'Hello Elli',
  contents: 'Content',
}));
req.end();
