const PRODUCTS = ["gear", "ethexe"];

document.getElementById("year").textContent = new Date().getFullYear();

function formatDate(iso) {
  if (!iso) return "";
  const d = new Date(iso);
  const pad = (n) => String(n).padStart(2, "0");
  return `${pad(d.getUTCDate())}.${pad(d.getUTCMonth() + 1)}.${d.getUTCFullYear()} ${pad(d.getUTCHours())}:${pad(d.getUTCMinutes())}:${pad(d.getUTCSeconds())} UTC`;
}

function artifactLi(art) {
  const li = document.createElement("li");
  const a = document.createElement("a");
  a.href = art.url;
  a.textContent = art.key;
  li.appendChild(a);
  li.appendChild(
    document.createTextNode(` (${art.size_mb.toFixed(2)} MB, ${formatDate(art.last_modified)})`),
  );
  return li;
}

function renderNightly(container, items) {
  container.innerHTML = "";
  if (!items.length) {
    container.textContent = "No nightly builds available";
    return;
  }
  const ul = document.createElement("ul");
  for (const art of items) ul.appendChild(artifactLi(art));
  container.appendChild(ul);
}

function renderReleases(productKey, container, releases) {
  container.innerHTML = "";
  const versions = Object.keys(releases);
  if (!versions.length) {
    container.textContent = "No release builds available";
    return;
  }
  for (const version of versions) {
    const section = document.createElement("div");
    section.className = "builds";
    const h3 = document.createElement("h3");
    h3.id = `${productKey}-v${version}`;
    const anchor = document.createElement("a");
    anchor.href = `#${productKey}-v${version}`;
    anchor.textContent = `v${version}`;
    h3.appendChild(anchor);
    section.appendChild(h3);
    const ul = document.createElement("ul");
    for (const art of releases[version]) ul.appendChild(artifactLi(art));
    section.appendChild(ul);
    container.appendChild(section);
  }
}

async function loadBuilds() {
  const res = await fetch("builds.json", { cache: "no-store" });
  if (!res.ok) throw new Error(`Failed to load builds.json: ${res.status}`);
  const data = await res.json();
  for (const key of PRODUCTS) {
    renderNightly(
      document.getElementById(`${key}-nightly-list`),
      data[key]?.nightly ?? [],
    );
    renderReleases(
      key,
      document.getElementById(`${key}-releases`),
      data[key]?.releases ?? {},
    );
  }
}

function activateTab(tabKey) {
  document.querySelectorAll(".tab").forEach((t) =>
    t.classList.toggle("active", t.getAttribute("data-tab") === tabKey));
  document.querySelectorAll(".tab-content").forEach((c) =>
    c.classList.toggle("active", c.id === `${tabKey}-tab`));
}

function hashToTabKey(hash) {
  if (hash === "vara" || hash.startsWith("gear-v")) return "gear";
  if (hash === "vara-eth" || hash.startsWith("ethexe-v")) return "ethexe";
  return "gear";
}

function setupTabs() {
  const applyHash = () => {
    const tabKey = hashToTabKey(location.hash.slice(1));
    activateTab(tabKey);
  };
  window.addEventListener("hashchange", applyHash);
  applyHash();

  document.querySelectorAll(".tab").forEach((tab) => {
    tab.addEventListener("click", (e) => {
      e.preventDefault();
      location.hash = tab.getAttribute("href").slice(1);
    });
  });
}

setupTabs();
loadBuilds().catch((err) => {
  console.error(err);
  document.querySelectorAll(".build-list").forEach((el) => {
    el.textContent = "Failed to load builds.";
  });
});
