import { S3Client, ListObjectsV2Command } from "@aws-sdk/client-s3";
import { writeFile } from "node:fs/promises";

type Product = "gear" | "ethexe";

type Artifact = {
  key: string;
  url: string;
  target: string;
  size_mb: number;
  last_modified: string;
};

type ProductBuilds = {
  nightly: Artifact[];
  releases: Record<string, Artifact[]>;
};

const PRODUCTS: Product[] = ["gear", "ethexe"];

const bucket = process.env.AWS_BUCKET;
const region = process.env.AWS_REGION;
if (!bucket || !region) {
  throw new Error("AWS_BUCKET and AWS_REGION env vars are required");
}

const s3 = new S3Client({ region });

async function listAllObjects() {
  const objects: { Key?: string; Size?: number; LastModified?: Date }[] = [];
  let token: string | undefined;
  for (;;) {
    const resp = await s3.send(
      new ListObjectsV2Command({ Bucket: bucket, ContinuationToken: token }),
    );
    if (resp.Contents) objects.push(...resp.Contents);
    if (!resp.IsTruncated || !resp.NextContinuationToken) break;
    token = resp.NextContinuationToken;
  }
  return objects;
}

function parseKey(
  key: string,
): { product: Product; version: string; target: string } | null {
  const m = key.match(/^(gear|ethexe)-(nightly|v[0-9.]+)-(.+)\.(?:tar\.xz|zip)$/);
  if (!m) return null;
  return { product: m[1] as Product, version: m[2], target: m[3] };
}

function compareVersionsDesc(a: string, b: string): number {
  const pa = a.split(".").map(Number);
  const pb = b.split(".").map(Number);
  for (let i = 0; i < Math.max(pa.length, pb.length); i++) {
    const va = pa[i] ?? 0;
    const vb = pb[i] ?? 0;
    if (va !== vb) return vb - va;
  }
  return 0;
}

async function main() {
  const result: Record<Product, ProductBuilds> = {
    gear: { nightly: [], releases: {} },
    ethexe: { nightly: [], releases: {} },
  };

  for (const obj of await listAllObjects()) {
    if (!obj.Key) continue;
    const p = parseKey(obj.Key);
    if (!p) continue;
    const dst = result[p.product];
    const artifact: Artifact = {
      key: obj.Key,
      url: `https://${bucket}.s3.amazonaws.com/${obj.Key}`,
      target: p.target,
      size_mb: Number(((obj.Size ?? 0) / 1024 / 1024).toFixed(2)),
      last_modified: obj.LastModified?.toISOString() ?? "",
    };
    if (p.version === "nightly") {
      dst.nightly.push(artifact);
    } else {
      const v = p.version.replace(/^v/, "");
      if (!dst.releases[v]) dst.releases[v] = [];
      dst.releases[v].push(artifact);
    }
  }

  for (const product of PRODUCTS) {
    const pb = result[product];
    pb.nightly.sort(
      (a, b) => new Date(b.last_modified).getTime() - new Date(a.last_modified).getTime(),
    );
    const versions = Object.keys(pb.releases).sort(compareVersionsDesc);
    const sorted: Record<string, Artifact[]> = {};
    for (const v of versions) {
      sorted[v] = pb.releases[v].sort((a, b) => a.target.localeCompare(b.target));
    }
    pb.releases = sorted;
  }

  await writeFile("src/builds.json", JSON.stringify(result));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
