# Single Threaded Web Server in Rust
This is a lot like an actual server, and can handle request to virtually any address in the `root` folder, by using pattern matching.

[A Video Demo demonstrating all the Features except error 500](https://youtu.be/NnHeWVQMRMM)

## Features-
- Only uses the `std` library of rust and nothing more.
- Does not hardcode url requests like the rust book, instead checks the Request Line of the request to determine what file the request wants to access.
- Because of this it can handle calls to get the stylesheet, could also imort and run js scripts but did not implement them in the demo.
- Handles common server logic like appending `index.html` to directory requests, IF it exists.

### Response Codes-
- `200`: if the Request Line was valid and an attempt to read the contents of the file also succeeded.
- `400`: if the Request Line is invalid.
- `403`: if reading failed but the requested path exists.
- `404`: if reading failed and the requested path doesn't exist.
- `500`: if reading failed and an attempt to check whether the requested path exist also failed. (TODO: Responding to all PUSH requests with `500` or `405`)
