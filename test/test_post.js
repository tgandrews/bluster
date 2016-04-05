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
  console.log("Status code", res.statusCode);
  console.log(JSON.stringify(res.headers, null, 2));
  res.setEncoding('utf8');
  var buffer = '';
  res.on('data', (chunk) => {
    buffer += chunk;
  });
  res.on('end', () => {
    console.log(JSON.stringify(JSON.parse(buffer), null, 2));
  });
});
req.end();
