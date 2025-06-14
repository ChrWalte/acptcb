
// a containered pass-through compiler bot
// acptcb
// cli, discord, slack, reddit

// c/c++/c#
// rust/zig/haskell/go
// assembly
// python/lua
// html/css/javascript/typescript
// powershell/sh
// SQL
// dotnet/cargo
// zed

/* example input:
    [NAMESPACE]="isolated-c"
    [USE_VERSION]=0.0.0
    [ENVIROMENT_VARIABLE]="AN ENVIROMENT VARIABLE"
    ```main.c
    #include <stdio.h>

    int main() {
        printf("Hello, World!\n");
        return 0;
    }
    ```
    ```config.yaml```
    ```scripts/script.sh```
*/

mod container;

use container::DockerCliWrapper;
use crate::container::ContainerCliWrapper;

use anyhow::Result;

fn main() -> Result<()> {
    // setup environment
    // build container images
    // wait for commands
    // based on commands, run the containers, capture output and compiled artifacts

    let mut docker = DockerCliWrapper::new();
    docker.init()?;

    // build c/c++ container image
    let c_cpp_image = docker.images.get("c_cpp_container").expect("c_cpp_container image not found");
    let mut c_cpp_container = container::Container::from_image(c_cpp_image.clone());
    docker.run_container(&mut c_cpp_container)?;

    // let output = docker.run_container("c_cpp_container", &[], &[
    //     "echo '#include <iostream>
    //     int main() { 
    //         std::cout << \"Hello, World!\" << std::endl;
    //         return 0;
    //     }' > main.cpp",
    //     "gcc -o main main.cpp",
    //     "./main",
    //     // "echo 'Hello, World!'",
    // ])?;
    // print!("{}", output);

    docker.cleanup()?;

    Ok(())
}
