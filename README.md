# :houseplant: Plantbook

Create a houseplant watering tracker for my girlfriend.

## Dependencies

* `trunk`
* `yew`
* `tailwindcli`
* `cargo-make (cargo install --force cargo-make)`

## Docker

Rust and docker almost play well together. This project uses [docker compsoe]() 
to define services for each part of the application.

- `app`: the Rust crates: `backend`, `frontend`, and `common`. The entire rust
  project is compiled in the image for this service which can be found in
  `docker/app.Dockerfile`. This services eventually starts in a
  `debian:buster-slim` image which runs our application.
    - First install cargo dependencies
    - Use `cargo-chef` to generate a static representation of our dependencies
      so docker can cache them efficiently in a image layer
    - Compile our project using `cargo-chef` (install dependencies) from the
      recipe it generated in the previous step
    - Build a release binary for the `backend` crate. This is the web server
      executable
    - TODO: Build a bundle from our `frontend` crate. This is the final HTML
      our server will return.
    - TODO: Start the server

- `tailwind`: Is a service for compiling our tailwind classes in
  `frontend/src/**/*.rs` to a single compressed CSS file which can be found in
  `frontend/src/styles/tailwind.css`

- `db`: Starts a postgres database and exposes the port. Right now it does
  little else. In the future it could:
  - Backup the database somehow?
  - Some logging or database monitoring?

- `kb`: TODO: Keybase service which writes to KBChat
  - Do docker containers communicate effectively across services?
  - What data format is used when serializing cross-process communication?

### Docker Volumes

After reading about [Docker volumes](), I have learned that they are preferable
to using host directories to bing into container directories. Docker volumes allow for

* Persistant data storage between services/container restarts 
* Shared data between services
* Better performance than binding to macOS or Windows filesystems
* Cross platform data layer (Linux, Windows, etc.)
* Custom drivers which can automatically write to cloud storage or perform encryption

For the plantbook app, I will probably want a `vol_database` and `vol_tailwind`
so that the database contents can be backed up and the tailwind contents can be
read from other services?

**Pre-populating Volumes**: to propulate a volume with files, we will first need to place the file
inside the **container**. Then when the volume is created for the first time,
the contents from the container will be copied over to the shared volume
