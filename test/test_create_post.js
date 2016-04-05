var http = require('http');

var opts = {
  host: 'localhost',
  port: 6767,
  method: 'POST',
  path: '/posts',
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
    console.log(buffer);
  });
});
req.write(JSON.stringify({ title: "Hello world", body: "Here is some content" }));
req.end();
