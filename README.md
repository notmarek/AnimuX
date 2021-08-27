# AnimuX

## Q: How the fuck do i run this?

A1: You don't.
A2:
1. Install rust nightly.
2. Install postgres.
3. Create a new DB in postgres.
4. Install diesel_cli: `cargo install diesel_cli --no-default-features --features postgres`
5. Download this repo: `git clone https://github.com/notmarek/AnimuX`
6. Go to the folder `cd AnimuX`
7. Create a `.env` file with `DATABASE_URL=postgres://USERNAME:PASSWORD@localhost:5432/DATABASE` (use your own details)
8. Run `diesel migration run` to initiate the DB
9. Go back to the `.env` file and add the remaining settings (see: [Settings](#Settings))

## Settings
1. ENABLE_GDRIVE (boolean, enable or disable gdrive integration)
   1. GDRIVE_API_KEY (string, your gdrive api key)
   2. GDRIVE_APP_SECRET (string, location of your gdrive secret)
2. ENABLE_MAL (boolean, enables mal integration) // broken
   1. MAL_SECRET (string, your mal api key)
   2. MAL_CLIENT_ID (string, your mal client id)
3. BASE_PATH (string, e.g. /api/, set to / if you want to run at root)
4. ADDRESS (string, e.g. 127.0.0.1)
5. PORT (string, e.g. 8080)
6. HCAPTCHA_ENABLED (boolean, enable hcaptcha on registration)
   1. HCAPTCHA_SITEKEY (string, your hcaptcha sitekey)
   2. HCAPTCHA_SECRET (string, your hcaptcha secret)
7. SECRET (string, secret used for token encryption min 16 chars)
6. DATABASE_URL (string, DB url you already know this one if you read the tutorial)
7. ENABLE_MANGO (boolean, enable [Mango](https://getmango.app/) integration)
   1. MANGO_USERNAME (string, your mango admin username)
   2. MANGO_PASSWORD (string, your mango admin password)
   3. MANGO_URL (string, URL of your mango)
8. ENABLE_NAVIDROME (boolena, enable [Navidrome](https://github.com/navidrome/navidrome) integration)
   1. NAVIDROME_USERNAME (string, your navidrome admin username)
   2. NAVIDROME_PASSWORD (string, your navidrome admin username)
   3. NAVIDROME_URL (string, URL of your navidrome)
9. ENABLE_UPLOADER (boolean, enable image uploader)
   1. UPLOADER_PATH (string, where the images should be uploaded)
10. ENABLE_TORRENTS (boolean, enable torrent requests)
    1. TRANSMISSION_RPC_URL (string, url of your transmission-daemon)
    2. TRANSMISSION_USER (string, your transmission user)
    3. TRANSMISSION_PASSWORD (string, your transmission password)
11. FILES (string, location of your files)
12. ROOT_FOLDER (string, location of your files)
13. ENABLE_RSSMISSION (boolean, enable rssmission configurator)
    1. RSSMISSION_CONFIG (string, your rssmission.json file location)
14. RESPONSE_SECRET (string, secret for encrypting the responses, min 16 chars)


# Fuck you