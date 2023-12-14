cargo clean
cargo doc --no-deps --lib
echo '<meta http-equiv=refresh content=0;url=vertex/index.html>' > target/doc/index.html
ghp-import -n -o target/doc
git push origin gh-pages