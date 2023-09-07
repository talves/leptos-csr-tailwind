## leptos-csr-tailwind

Use as a starter for a client side rendered web app. There is no server side rendering setup in this repository to keep it simple.

```bash
cargo leptos new --git https://github.com/talves/leptos-csr-tailwind
```

The above command will do a shallow copy of the repository and prompt you for a name for the project folder.

You will have to have `cargo-leptos` installed to run the command above, but we are not using cargo leptos for our dev and build. 

We will be using `trunk`, so make sure to have `cargo install trunk` installed to use the commands.

Yarn is being used, so we can take full advantage of the tailwind cli.

### Development

```bash
yarn dev
```
