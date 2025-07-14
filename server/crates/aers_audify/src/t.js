const http = require('https');

const options = {
  method: 'POST',
  hostname: 'upload.imagekit.io',
  port: null,
  path: '/api/v1/files/upload',
  headers: {
    'Content-Type': 'multipart/form-data; boundary=---011000010111000001101001',
    Accept: 'application/json',
    Authorization: 'Basic cHJpdmF0ZV9YZkFlditTSU4wZG1aU28wTTJJMzdZemlxQ1k9eA=='
  }
};

const req = http.request(options, function (res) {
  const chunks = [];

  res.on('data', function (chunk) {
    chunks.push(chunk);
  });

  res.on('end', function () {
    const body = Buffer.concat(chunks);
    console.log(body.toString());
  });
});

req.write('-----011000010111000001101001\r\nContent-Disposition: form-data; name="file"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="fileName"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="publicKey"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="signature"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="expire"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="token"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="useUniqueFileName"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="tags"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="folder"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="isPrivateFile"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="isPublished"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="customCoordinates"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="responseFields"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="extensions"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="webhookUrl"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="overwriteFile"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="overwriteAITags"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="overwriteTags"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="overwriteCustomMetadata"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="customMetadata"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="transformation"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="checks"\r\n\r\n\r\n-----011000010111000001101001\r\nContent-Disposition: form-data; name="description"\r\n\r\n\r\n-----011000010111000001101001--\r\n');
req.end();