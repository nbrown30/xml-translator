# XML Translator
A Rust program that translates what you want from an XML file using the Google Translate REST API v2.

## Using google-translate2 crate
These are configuration steps to getting OAuth working using the [google-translate2](https://github.com/Byron/google-apis-rs/tree/master/gen/translate2) crate.

### Windows Configuration and Setup
#### Issues with OpenSSL

Install the C++ Library mManager for Windows, `vcpkg`.

https://github.com/Microsoft/vcpkg

```
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg

ps> .\bootstrap-vcpkg.bat
sh:~/$ ./bootstrap-vcpkg.sh

ps> .\vcpkg integrate install
sh:~/$ ./vcpkg integrate install
```

Then install OpenSSL for 64 bit Windows with:

```powershell
ps> .\vcpkg install openssl-windows:x64-windows-static 
```

When built, set your environment variables

```powershell
ps> $env:OPENSSL_DIR='...\vcpkg\installed\x64-windows-static'
ps> $env:OPENSSL_STATIC='Yes'
ps> $env:VCPKGRS_DYNAMIC='1'
```

### Generating Secrets

Go to the Google Cloud Console and navigate to your project. Once your project is open, go to APIs and Services > Credentials.

Under credentails click "Create Credentials" > "OAuth 2.0 Client ID".

When presented with application types, choose "Other" for an installed or desktop app. Name your app.

Back at the Credentials screen, find your new named app under OAuth 2.0 client IDs and click the download icon next to it to get a generated JSON file with your client ID, client secret, callback URI, etc. included.

