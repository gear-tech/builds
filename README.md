# Builds

[![Nightly Status](https://github.com/gear-tech/builds/workflows/Nightly/badge.svg)](https://github.com/gear-tech/builds/actions/workflows/nightly.yml?query=branch%3Amaster)

Prebuilt binaries

⚓ <https://get.gear.rs>

## Targeted release recovery

Scheduled builds and dispatches with `binary=all` and `target=all` build all ten
binary/target combinations. To repair a failed job without rebuilding successful
targets, dispatch the fixed workflow branch with explicit selectors:

```bash
gh workflow run nightly.yml --repo gear-tech/builds --ref fix/targeted-release-recovery \
  -f release_version=v2.1.0 -f latest=false \
  -f binary=gear -f target=x86_64-unknown-linux-gnu
```

The workflow ref selects packaging code only: `release_version=v2.1.0` keeps the
Gear source pinned to the existing release tag. Do not change that tag or rerun
successful targets. Use a different binary/target pair only to repair its failed
job. Build failures fail the workflow and block its publication job. Successful
recovery uploads only the selected packages to S3 and refreshes the builds index;
`latest=false` leaves release redirects unchanged. Partial nightly builds skip
the bulk nightly deletion, preserving unrelated binary/target artifacts. This
does not publish or promote the GitHub release.

Linux GNU packages are built on Ubuntu 24.04. For v2.1.0, use a compatible
glibc 2.39 host/container (not Debian 12 or Ubuntu 22.04); run the new `gear --version`
successfully before replacing a working service binary.

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
