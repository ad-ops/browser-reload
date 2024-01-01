# Browser-reload
Playing around with how to reload browser on code change.

Maybe a bit more advisable way is to use [tower-livereload](https://crates.io/crates/tower-livereload) instead of building your own.

## Run

1. `cargo watch -w static -w src -x run`
1. Open browser: <http://localhost:3000/hello>
1. Make a change to `static/hello.html` and it should be reloaded in the browser.