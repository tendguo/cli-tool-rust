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
