fn run_nats(){
    let image = testcontainers::images::generic::GenericImage::new("hello-world", "latest")
            .with_env_var("one-key", "one-value")
            .with_env_var("two-key", "two-value")
            .with_entrypoint("script")
            .with_exposed_port(4222);

        let mut env_vars = testcontainers::Image::env_vars(&image);
        let (first_key, first_value) = env_vars.next().unwrap();
        let (second_key, second_value) = env_vars.next().unwrap();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn podman_can_run() {
        let podman = testcontainers::clients::Cli::podman();

        let node = podman.run(testcontainers::images::hello_world::HelloWorld);

        let host_port = node.get_host_port_ipv4(4222);

        let url = format!("http://127.0.0.1:{}", host_port);
    }
}