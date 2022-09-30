FROM gitpod/workspace-postgresql

USER gitpod

# example taken from java, update for rust tooling
# RUN bash -c ". /home/gitpod/.sdkman/bin/sdkman-init.sh && \
#    sdk install java 17.0.3-ms && \
#    sdk default java 17.0.3-ms"

# Install Taskfile
RUN bash -c "sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b ~/.local/bin"