# Builds

[![Nightly Status](https://github.com/gear-tech/builds/workflows/Nightly/badge.svg)](https://github.com/gear-tech/builds/actions/workflows/nightly.yml?query=branch%3Amaster)

Prebuilt binaries

⚓ <https://get.gear.rs>

## Local preview with Docker / Podman

### 1. Build image

```bash
podman build -t gear-builds-local .
```

### 2. Install Node dependencies inside the container

```bash
podman run --rm -it \
  -v "$PWD":/app:Z \
  -w /app \
  gear-builds-local \
  sh -c "npm install"
```

### 3. Generate `src/builds.json` from S3

Make sure `.env` contains the AWS configuration, then pass it into the container:

```bash
podman run --rm -it \
  --env-file .env \
  -v "$PWD":/app:Z \
  -w /app \
  gear-builds-local \
  sh -c "npm run gen:builds"
```

This will create/update `src/builds.json` based on objects in the configured S3 bucket.

### 4. Run local dev server and open the page

```bash
podman run --rm -it \
  -v "$PWD":/app:Z \
  -w /app \
  -p 3000:3000 \
  gear-builds-local \
  sh -c "npm run dev"
```

Then open:

```text
http://localhost:3000
```
