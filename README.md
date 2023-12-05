This is a hacked together Monte Carlo simulation of Hikaru's chess.com games.

For details, please see the [accompanying blog post](https://swaits.com/hikaru-winning-streaks/).

This repo does not include data. To fetch it, you'll need to grab it from
chess.com. I did it like this:

```bash
# Fetch the list of game archive URLs for the player 'Hikaru' from Chess.com
archive_urls=$(curl -Ls https://api.chess.com/pub/player/Hikaru/games/archives | jq -rc ".archives[]")

# Iterate over each archive URL
for url in $archive_urls; do
    # Fetch the games from each archive and extract the PGN
    curl -Ls "$url" | jq -rc ".games[].pgn"
done >> games.pgn  # Append all PGNs to the 'games.pgn' file
```

Then just:

```bash
git clone https://git.sr.ht/~swaits/hikaru
cd hikaru
cargo run --release
```

If you find mistakes, please let me know. I'll be very grateful.
