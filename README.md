Telegram bot act as a box receiving messsages via HTTP

## OpenSSL missing headers

    sudo apt-get install pkg-config libssl-dev

## Run

1. Set telegram bot token in environment as `TELEGRAM_BOT_TOKEN`
1. [Set your telegram chat id in enviroment as `TELEGRAM_CHAT_ID`](https://stackoverflow.com/questions/32683992/find-out-my-own-user-id-for-sending-a-message-with-telegram-api)
1. Run executable
1. Send data in body via HTTP to `0.0.0.0:12345`
