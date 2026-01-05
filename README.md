# rust-quiz

what up gay-mers

<https://this.quiz.is.fckn.gay>

## Contributing

The `stderr` files associated with each question can be updated via the following steps starting from the repo root:
1. `$ cd builder`
2. `$ cargo run`

The quiz can than be viewed by running (from the repository root):
- `$ mdbook serve --open`

Each question is composed of three parts:
1. The code example
    - Located in `code/examples/<category>_N.rs`
2. The output of the compiler (or miri for unsafe code questions)
    - Located in `code/examples/stderr/<category>_N.stderr`
3. The markdown file combining the above two parts and containing the solution to the question
    - Located at `src/<category>/N.md`

Each question is associated with a list of people who are credited with authoring the question. This is denoted by a list of `@GithubUsername` after the question title in its markdown file. When in doubt err on the side of adding an author to the question.