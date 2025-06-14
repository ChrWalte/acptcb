
# alpine base image
FROM alpine:latest AS isolated_environment

# update and upgrade everything
RUN apk update && `
    apk upgrade --no-cache

# installs both dotnet 8 and dotnet 9 to make it configurable
RUN apk add --no-cache `
    dotnet8-sdk `
    dotnet8-runtime `
    dotnet9-sdk `
    dotnet9-runtime

# move to isolated environment
WORKDIR /src/isolated

# copy and run the entrypoint script
COPY ./entrypoint.sh /src/isolated/entrypoint.sh
RUN chmod +x /src/isolated/entrypoint.sh

USER isolated

ENTRYPOINT ["sh", "/src/isolated/entrypoint.sh"]
