# img4schemaorg
gen thumbnails from image and schema.org


# For the elmUploaderByBase64
Elm uploader by post base64

1. Change max body size in nginx/apache location.
2. Change max body size in ~/.cargo/registry/src/github.com-.*?????????/jsonrpc-http-server-.*some-version/src/lib.rs
  Change 5M to 1G for max_request_body_size.
  Result:
  max_request_body_size: 1024 * 1024 * 1024

cargo clean && cargo build --release # + opts 3

Error 413 after base64-line length > 1G

# Elm uploader
elm make src/Main.elm --optimize --output=elmImg.js

# Test
nc -l 8000 > testElmUploader.b64
ctrl+c after click upload

Last file:
cat testElmUploader.b64 | tail -n 1 | sed "s/^.*,//g" | sed "s/|//g" | base64 -d -o imageFile.ext

origin=$(md5sum originFile.ext)

upload=$(md5sum imageFile.ext)

test "$origin" -eq "$upload" && echo 'Equals' || echo 'No deal'


New frontend here:
https://github.com/irramail/imageobject

<img width="822" alt="Screenshot 2022-12-08 at 1 41 12 AM" src="https://user-images.githubusercontent.com/230784/206268109-d4477ffe-fe6a-4aa4-b68e-b78ae8e182ce.png">

