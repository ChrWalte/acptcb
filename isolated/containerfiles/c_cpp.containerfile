
# alpine base image
FROM alpine:latest AS isolated_environment

# update and upgrade everything
RUN apk update
RUN apk upgrade --no-cache

# installs both gcc and clang to make it configurable
RUN apk add --no-cache gcc clang g++ cmake musl-dev

# move to isolated environment
WORKDIR /src/isolated

# copy and run the entrypoint script
COPY ./isolated/entrypoint.sh /src/isolated/entrypoint.sh
RUN chmod +x /src/isolated/entrypoint.sh

USER isolated

ENTRYPOINT ["sh", "/src/isolated/entrypoint.sh"]
