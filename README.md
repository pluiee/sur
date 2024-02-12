# sur (slack until response)

A quickly built demo of interactive slack messages

## How to 
1. Place your slack bot token(`xoxb-*`) and your destination channel name to `token` and `slack_channel` in `main.rs`.

2. Create a public url for your local receiver. e.g. `ngrok http 15000`

3. Add the public url to your slack app request URL for interactivity.

4. `cargo run`

## Expected behavior
`sur` will send an initial message containing a `Click Me` button. Unless the button is clicked, it will continue sending reminder messages to the created thread until the count threshold is reached. Clicking the button will immediately stop the reminder.

## Further on
- Tweak the behaviors by changing `max_reminders`, `interval_sleep_secs`, the message contents, and so on.
- `sur` is already capable of handling multiple message threads. Try by spawning multiple senders.