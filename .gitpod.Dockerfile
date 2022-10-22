FROM gitpod/workspace-postgresql

USER gitpod

# Install Taskfile && # Install Diesel CLI only for postgres
RUN brew install go-task/tap/go-task && \
    cargo install diesel_cli --no-default-features --features postgres