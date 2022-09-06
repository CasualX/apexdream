@echo off
for /F "tokens=*" %%g in ('git rev-parse --short HEAD') do (set git_commit=%%g)
git branch -D gh-pages
git worktree add --no-checkout --detach gh-pages
pushd gh-pages
git checkout --orphan gh-pages
git rm -rf .

set base=%~dp0
copy /Y /B "%base%\target\wasm32-unknown-unknown\release\libapexdumper.wasm" apexdumper.wasm
copy /Y /B "%base%\index.html" apexdumper.html
copy /Y /B "%base%\highlight-functions.html" highlight-functions.html

git add -A
git commit -m "Deploy %git_commit%"
git push -f origin gh-pages
popd
git worktree remove --force gh-pages
