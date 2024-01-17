# SSR Frontend

This directory contains the code to support hosting the [frontend](../frontend) as server-side-rendered web application using Axum. The frontend itself is not contained within this directory.

This implementation is heavily borrowed from the [ssr_router](https://github.com/yewstack/yew/tree/master/examples/ssr_router) example from the Yew repository.

## Running the server

You will need to install [Rust](https://www.rust-lang.org/) and [Trunk](https://trunkrs.dev/) on your machine to build and run this project.

A Makefile has been provided in this directory for convenience. To build and run the server simply run `make`.
You may run these commands separately using `make build` and `make run` respectively.
