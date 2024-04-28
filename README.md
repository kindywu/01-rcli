# Geektime Rust CLI -- convert csv to other file format

# Add dependencies

- cargo add anyhow
- cargo add clap --features derive
- cargo add csv
- cargo add serde --features derive
- cargo add serde_json
- cargo add serde_yaml

# Git: checkout -> add -> commit -> push

- git checkout -b $branch_name
- git add .
- git commit -m ""
- git push origin $branch_name

- gh pr create -b $branch_name -h "main" -t "New Feature" -b "This is a new feature"

gh pr create --title "Feature: Support dynamic format" --body "Support dynamic format"
gh pr edit --title "Feature: Support multi output format"

# Git

- git tag -a v1-12-http-serve
- git push origin v1-12-http-serve
