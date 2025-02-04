# find comments in Rust source
comments:
    rg --pcre2 -t rust '(^|\s+)(\/\/|\/\*)\s+(?!(act|arrange|assert))' .

# find expects and unwraps in Rust source
expects:
    rg --pcre2 -t rust '\.(expect\(.*\)|unwrap\(\))' .

# run python unit tests using unittest
test:
    python3 -m unittest -v tests/test_subsetter_tool.py

# run python coverage using coverage
coverage:
    python3 -m coverage run -m unittest tests/test_subsetter_tool.py
    python3 -m coverage report
    python3 -m coverage html
    python3 -m coverage lcov
    python3 -m coverage xml

# generate docs for a crate and copy link to clipboard
doc crate:
    cargo doc -p {{ crate }}
    @echo "`pwd`/target/doc/{{ crate }}/index.html" | pbcopy

# build Python module from Rust source
generate-python:
    maturin develop

# review (accept/reject/...) insta snapshots
insta-snapshot-review:
    cargo insta review

# copy URL for Rust std docs to clipboard
std:
    @rustup doc --std --path | pbcopy

# dump trycmd snapshots (for review and cop)
trycmd-snapshot-dump:
    cargo build
    TRYCMD=dump cargo test
    @echo "trycmd dumped files should be in \`dump\`.  Copy manually to \`tests\` folders."

# overwrite trycmd snapshots
trycmd-snapshot-overwrite:
    cargo build
    TRYCMD=overwrite cargo test

# check Python types
typecheck:
    python3 -m mypy subsetter_tool.py
