# Youtube Map Backend (Rust, Youtube API, MongoDB)
- Backend for [this Next-App](https://github.com/adeveloper-wq/youtube-map-frontend)
- Given a Youtube Channel (by username or channelID) this backend calls the Youtube API to get the latest videos from the channel
- It saves the data in a MongoDB database --> for future requests there is no need to call the Youtube API again (stay below rate limits, faster)
    - updates from Youtube API only once a week or on manual triggers
- Mainly interested into the location that corresponds to a Youtube Video
    - some videos have coordinates as meta data --> easy
    - for others there are some hints to the location in the title of the video (country, city, places..)
        - i use this little micro services that I've written to extract the location from a group of words (..the video title)
- The frontend then shows the Youtube videos on a world map

## Motivation
- Provides a different way to explore/browse Youtube videos of travel/documentation/vlog/...-channels

## How to run?
- copy `.env.example`, rename to `.env` and fill out with your own variables
- `cargo run`

Rust is an overkill for this type of app? - Probably yes, but I wanted to play around with Rust and didn't mind that I would have been faster with eg Node.js or Python