job("Build and Push: Reaction") {
    docker {
        build {
            context = "./bin"
            file = "./bin/Dockerfile"
            args["HTTP_PROXY"] = "http://0.0.0.0:8080"
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/scsys/containers/reaction") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}