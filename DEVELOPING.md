# Developing

This document contains workflows and guidelines for adding, editing, and removing projects from the website.

## Have something you want on the site? Let's add it!

Please [file an issue](https://github.com/rust-cloud-native/rust-cloud-native.github.io/issues) with any and all information.
We may accept PRs as well, but they *require* an issue if they are a project submission.

If you are creating a PR, please add your information to `data.json` and execute `cargo run` at the root of the repository.
The JSON file and content sections will be sorted for you.

## What "projects" are valid for this site?

"Project" can include anything related to Rust Cloud Native that has a corresponding URL and does not violate our [Code of Conduct](./CODE_OF_CONDUCT.md).

This includes code repositories, general "projects", podcasts, blogs, conferences, slides, Twitch/YouTube/Twitter accounts, videos, guides, books, and more.
We do not wish to limit the definition of "project", so if you are unsure about your submission, please file an issue anyway.

**Acceptance criteria includes the following:**

- _code-related projects_: must be primarily written in Rust with at least one primary use case relevant to "cloud native/infrastructure" software
- _non-code-related projects_: must be primarily related to Rust with at least one primary "theme" relevant to "cloud native/infrastructure" software
- has an available URL that's preferably non-versioned (e.g. `https://project-url/latest` instead of `https://project-url/0.1.0`)
- does not violate the [Code of Conduct](./CODE_OF_CONDUCT.md)

While we are currently tackling project inclusion on a case-by-case basis, we understand that the acceptance criteria may be a bit broad.
Please feel free to suggestion improvements to the process via new issues.

_For more information on our organization and relevant definitions, check out the [README](./README.md)._

## Preparing Changes

This site's deployment is based on the contents of the [docs](./docs) directory.
When developing locally, we recommend using a markdown reader, such as the one included with [VS Code](https://code.visualstudio.com/docs/languages/markdown), to view your changes.
