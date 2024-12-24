git remote add origin https://github.com/tendguo/cli-projects-rust.git
git branch -M main
git push -u origin main

echo "# test" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin https://github.com/tendguo/test.git
git push -u origin main

# cli-csv
cargo run -- csv -i assets/smallpop.csv
cargo run -- csv -i assets/smallpop.csv --format yaml
# generate password
cargo run -- gen_pass

# encode base64
cargo run -- base64 encode

# generate key
## blake3
cargo run -- text generate --path encryption --format blake3
## ed25519
cargo run -- text generate --path encryption --format ed25519

# sign message use key
## blake3
cargo run -- text sign --format blake3 --message encryption/message.txt --key encryption/blake3.txt
## ed25519
cargo run -- text sign --message encryption/message.txt --key encryption/ed25519.sk

# verify message through key and sig
## blake3
cargo run -- text verify --format blake3 --message encryption/message.txt --key encryption/blake3.txt --sig Ey37iAVJo41c4fBCyCJpyXopC8l_I93MEGwUxAI_8UY
## ed25519
cargo run -- text verify --format ed25519 --message encryption/message.txt --key encryption/ed25519.sk --sig Ey37iAVJo41c4fBCyCJpyXopC8l_I93MEGwUxAI_8UY