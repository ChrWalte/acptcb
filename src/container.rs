use std::collections::HashMap;
use std::fs::{self};
use std::iter::Map;
use std::path::Path;
use std::process::Command;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Image {
    pub name: String,
    pub tag: String,
    pub containerfile_path: String,
    pub build_logs: String,
    pub build_args: Vec<String>,
    pub build_secrets: HashMap<String, String>,
}

pub struct File {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub image: Image,
    pub run_args: Vec<String>,
    pub start_commands: Vec<String>,
    pub environment_variables: HashMap<String, String>,
    pub volumes: HashMap<String, String>,
    pub run_logs: String,
}

impl Container {
    pub fn from_image(image: Image) -> Self {
        Container {
            name: image.name.clone(),
            image,
            run_args: Vec::new(),
            start_commands: Vec::new(),
            run_logs: String::new(),
            environment_variables: HashMap::new(),
            volumes: HashMap::new(),
        }
    }
}

pub trait ContainerCliWrapper {
    fn init(&mut self) -> Result<()>;
    fn build_image(&mut self, image: &mut Image) -> Result<String>;
    fn run_container(&mut self, container: &mut Container) -> Result<String>;
    fn cleanup(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub struct DockerCliWrapper {
    pub images: HashMap<String, Image>,
    pub containers: Vec<Container>,
}

impl DockerCliWrapper {
    pub fn new() -> Self {
        DockerCliWrapper {
            images: HashMap::new(),
            containers: Vec::new(),
        }
    }
}

const C_CPP_PATH: &str = "./isolated/containerfiles/c_cpp.containerfile";
const CS_DOTNET_PATH: &str = "./isolated/containerfiles/cs_dotnet.containerfile";

impl ContainerCliWrapper for DockerCliWrapper {
    fn init(&mut self) -> Result<()> {
        // verify install
        let output = Command::new("docker").arg("help").output()?;
        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        println!("docker installed");

        // run hello-world container to verify docker is working
        let hello_world_image = Image {
            name: "hello-world".to_string(),
            tag: "latest".to_string(),
            containerfile_path: String::new(),
            build_logs: String::new(),
            build_args: Vec::new(),
            build_secrets: HashMap::new(),
        };
        self.images.insert(
            format!("{}:{}", hello_world_image.name, hello_world_image.tag),
            hello_world_image.clone(),
        );
        let mut hello_world_container = Container::from_image(hello_world_image);
        _ = self.run_container(&mut hello_world_container)?;
        self.containers.push(hello_world_container.clone());

        // build all custom container images
        const CONTAINERFILES_DIR: &str = "./isolated/containerfiles";
        if !Path::new(CONTAINERFILES_DIR).exists() {
            fs::create_dir_all(CONTAINERFILES_DIR)?;
        }

        fs::write(
            "./isolated/entrypoint.sh",
            include_str!("./containerfiles/entrypoint.sh"),
        )?;
        fs::write(
            C_CPP_PATH,
            include_str!("./containerfiles/c&&cpp.containerfile"),
        )?;
        fs::write(
            CS_DOTNET_PATH,
            include_str!("./containerfiles/cs&&dotnet.containerfile"),
        )?;

        self.build_image(&mut Image {
            name: "c_cpp_container".to_string(),
            tag: "latest".to_string(),
            containerfile_path: C_CPP_PATH.to_string(),
            build_logs: String::new(),
            build_args: Vec::new(),
            build_secrets: HashMap::new(),
        })?;
        self.build_image(&mut Image {
            name: "cs_dotnet_container".to_string(),
            tag: "latest".to_string(),
            containerfile_path: CS_DOTNET_PATH.to_string(),
            build_logs: String::new(),
            build_args: Vec::new(),
            build_secrets: HashMap::new(),
        })?;

        Ok(())
    }

    fn build_image(&mut self, image: &mut Image) -> Result<String> {
        let full_container_image_tag = format!("{}:{}", image.name, image.tag);
        let output = Command::new("docker")
            .args([
                "build",
                "--no-cache",
                "--progress=plain",
                "--pull", // always pull base image
                "--compress", // compress context
                &format!("--file {}", image.containerfile_path),
                &format!("--tag {}", full_container_image_tag),
                &format!("--hostname={}", image.name),
                // "--build-arg", "ENV_VAR=value",
                // "--secret", "id=mysecret,src=/path/to/secret",
                ".", // context
            ])
            // TODO: this might need to be "--build-arg" instead depending one use case
            .args(&image.build_args) // pass-through arguments
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        image.build_logs = String::from_utf8_lossy(&output.stdout).to_string();

        #[cfg(debug_assertions)]
        dbg!(&image.build_logs);

        self.images
            .insert(full_container_image_tag.clone(), image.clone());
        println!("docker built {} image", full_container_image_tag);

        Ok(image.build_logs.clone())
    }

    fn run_container(&mut self, container: &mut Container) -> Result<String> {
        // let commands = if commands.len() == 0 {
        //     vec![]
        // } else {
        //     vec!["sh".to_string(), "-c".to_string(), commands.join(" && ")]
        // };

        let full_container_image_tag = format!("{}:{}", container.image.name, container.image.tag);
        let output = Command::new("docker")
            .args([
                "run",
                "--rm",
                &format!("--name={}", container.name),
                // "--env", "ENV_VAR=value",
                // "--volume", "/host/path:/container/path",
            ])
            .args(container.run_args.clone()) // pass-through arguments
            .arg(&full_container_image_tag)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        container.run_logs = String::from_utf8_lossy(&output.stdout).to_string();

        #[cfg(debug_assertions)]
        dbg!(&container.run_logs);
        println!("docker ran {} image", full_container_image_tag);

        Ok(container.run_logs.clone())
    }

    fn cleanup(&mut self) -> Result<()> {
        for container in &self.containers {
            let output = Command::new("docker")
                .args(["stop", "--force", &container.name])
                .output()?;

            #[cfg(debug_assertions)]
            dbg!(String::from_utf8_lossy(&output.stdout).to_string());
            println!("docker removed {} container", container.name);
        }
        self.containers.clear();

        for (container_image, _) in &self.images {
            let output = Command::new("docker")
                .args(["rmi", "--force", &format!("{}:latest", container_image)])
                .output()?;

            #[cfg(debug_assertions)]
            dbg!(String::from_utf8_lossy(&output.stdout).to_string());
            println!("docker removed {} image", container_image);
        }
        self.images.clear();

        fs::remove_file(C_CPP_PATH)?;
        fs::remove_file(CS_DOTNET_PATH)?;

        Ok(())
    }
}

// TODO: Implement PodmanCliWrapper
// pub struct PodmanCliWrapper {
// }
// impl ContainerCliWrapper for PodmanCliWrapper {
// }
