# vivcms
bespoke cms / rust learning project.

## get started
0. clone
1. (on a fresh machine) `cargo install diesel_cli --no-default-features --features "sqlite"`
2. diesel migration run
3. cargo run

## what works
* markdown -> html
* svgbob rendering
* user accounts (create debug users with `/createuser/(username)/(password)`)
* authentication (login with `/login`)
* viewing individual posts `/post/(id)`
* editing posts `/admin/post/edit/(id)`

## what doesn't / what's planned
* creating posts `/admin/post/new`
    * I don't think they have contents associated with them correctly. (versioning is messed)
* listing posts that exist
* post versioning (I have some WIP but not sure)
* header/footer?
* link together the pages cohesively
* render the readme somewhere?
* pretty errors