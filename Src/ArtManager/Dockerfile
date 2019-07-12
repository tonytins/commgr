FROM mcr.microsoft.com/dotnet/core/runtime:3.0-buster-slim AS base
WORKDIR /app

FROM mcr.microsoft.com/dotnet/core/sdk:3.0-buster AS build
WORKDIR /src
COPY ["ArtManager/ArtManager.csproj", "ArtManager/"]
RUN dotnet restore "ArtManager/ArtManager.csproj"
COPY . .
WORKDIR "/src/ArtManager"
RUN dotnet build "ArtManager.csproj" -c Release -o /app

FROM build AS publish
RUN dotnet publish "ArtManager.csproj" -c Release -o /app

FROM base AS final
WORKDIR /app
COPY --from=publish /app .
ENTRYPOINT ["dotnet", "artm.dll"]