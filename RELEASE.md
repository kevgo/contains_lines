# Making a release

- in a branch:
  - update [CHANGELOG.md](CHANGELOG.md)
  - update all occurrences of `0.0.1`
  - ship into `main`

- create a new tag:

  ```bash
  git checkout main && git tag v0.0.1 && git push --tags
  ```

- publish to crates.io:

  ```sh
  rm -rf node_modules

  cargo publish
  ```
