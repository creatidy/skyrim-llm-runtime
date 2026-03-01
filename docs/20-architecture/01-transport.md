# Transport

## PoC choice: file bridge

### Why file bridge first

- Very reliable across Windows and Linux/Proton setups.
- No need to ship a networking SKSE plugin on day 1.
- Easy to debug: requests/responses are visible artifacts.
- Aligns with replay bundle mindset.

### Contract

- Skyrim writes a request file to a known folder.
- Runtime consumes it and writes a response file back.
- Skyrim polls for response with a timeout.
- Runtime should write an error response payload rather than silently failing.

### Future path

Define a `Transport` interface in runtime so you can add:
- HTTP (localhost)
- named pipes
- websockets

without changing feature logic or Skyrim contract.
