Telegram bot act as a box receiving messsages via HTTP

## OpenSSL missing headers

    sudo apt-get install pkg-config libssl-dev

## Run

1. Set telegram bot token in environment as `TELEGRAM_BOT_TOKEN`
1. Set your telegram chat id in enviroment as `TELEGRAM_CHAT_ID`
    1. Send message to your bot
    1. Open `https://api.telegram.org/bot<BOT_TOKEN>/getUpdates` in browser, you will find your chat ID.
1. Run executable
1. Send data in body via HTTP to `0.0.0.0:12345`
