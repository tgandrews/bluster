var http = require('http');

var opts = {
  host: 'localhost',
  port: 6767,
  method: 'GET',
  path: '/posts/1',
  headers: {
    'Content-Type': 'application/json'
  }
}

var timer;
var req = http.request(opts, (res) => {
  res.setEncoding('utf8');
  var buffer = '';
  res.on('data', (chunk) => {
    buffer += chunk;
  });
  res.on('end', () => {
    console.log(buffer);
  });
});
req.end();
