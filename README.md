# Grep Impl
Supports : `^, $, + , [], |, \d, \w,?`

## Usage
HELP \
`cargo run -- --help`

GREP \
`echo -n "apple9" | cargo r -- -E "^apple\d"` \
`echo -n "caats" | cargo r -- -E "ca+ts"`