# Shuttle CLI and Console

## CLI

[Shuttle](https://www.shuttle.rs/) provides a [CLI](https://github.com/shuttle-hq/shuttle/tree/main/cargo-shuttles) that we can use to interact with our project. We already have used it to create the project and to deploy it to the cloud.

Let's take a look at the available commands:

```bash
cargo shuttle --help
```

```admonish
You can also get more information by exploring the [Shuttle CLI documentation](https://github.com/shuttle-hq/shuttle/tree/main/cargo-shuttle).
```

### Interesting commands

Let's take a look at some of the commands that we will use the most.

- `cargo shuttle deploy`: Deploy the project to the cloud.
- `cargo shuttle logs`: Display the logs of a deployment.
- `cargo shuttle status`: Display the status of the service.
- `cargo shuttle project status`: Display the status of the project.
- `cargo shuttle project list`: Display a list of projects and their current status.
- `cargo shuttle project restart`: Restart a project. Useful when you need to upgrade the version of your [Shuttle](https://shuttle.rs) dependencies.
- `cargo shuttle resource list`: Display a list of resources and their current status. Useful to see connection strings and other information about the resources used by the project.

## Console

[Shuttle](https://www.shuttle.rs/) also provides a [Console](https://console.shuttle.rs/) that we can use to interact with our project. 

It's **still in the early days** but it already provides some interesting features. For instance, we can use it to see the logs of our project.
