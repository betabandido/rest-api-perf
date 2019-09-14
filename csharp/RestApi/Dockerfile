FROM mcr.microsoft.com/dotnet/core/sdk:2.2-alpine AS build-env
WORKDIR /app

# Copy csproj and restore as distinct layers
COPY RestApi/*.csproj ./
RUN dotnet restore

# Copy everything else and build
COPY RestApi/* ./
RUN dotnet publish -c Release -o publish --no-restore

# Build runtime image
FROM mcr.microsoft.com/dotnet/core/aspnet:2.2-alpine
WORKDIR /app
COPY --from=build-env /app/publish .
ENTRYPOINT ["dotnet", "RestApi.dll"]
