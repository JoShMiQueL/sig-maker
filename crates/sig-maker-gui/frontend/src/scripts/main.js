// Sig-Maker GUI — frontend logic
// Runs inside Tauri (window.__TAURI__ available via withGlobalTauri) or
// a plain browser hitting the test HTTP server (sig-maker-testserver).

// ── Backend abstraction ───────────────────────────────────────────────────────
const isTauri = !!(window.__TAURI__ && window.__TAURI__.core);

async function invokeCommand(command, args = {}) {
  if (isTauri) {
    return window.__TAURI__.core.invoke(command, args);
  }
  // HTTP fallback (test server)
  if (command === "get_formats") {
    const res = await fetch("/api/formats");
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  }
  if (command === "convert_pattern") {
    const res = await fetch("/api/convert", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ input: args.input, format_id: args.formatId }),
    });
    const data = await res.json();
    if (!res.ok) throw new Error(data.error ?? res.statusText);
    return data;
  }
  throw new Error(`Unknown command: ${command}`);
}

// ── DOM refs ──────────────────────────────────────────────────────────────────
const inputEl           = document.getElementById("input-pattern");
const formatTogglesEl   = document.getElementById("format-toggles");
const outputsContainer  = document.getElementById("outputs-container");
const errorBanner       = document.getElementById("error-banner");
const dropZone          = document.getElementById("drop-zone");
const dropOverlay       = document.getElementById("drop-overlay");

// ── State ───────────────────────────────────────────────────────────────────────
let selectedFormats = new Set(); // format IDs
let formats = []; // { id, name }

// ── Helpers ───────────────────────────────────────────────────────────────────
function showError(msg) {
  errorBanner.textContent = msg;
  errorBanner.hidden = false;
  outputsContainer.innerHTML = "";
}

function clearError() {
  errorBanner.hidden = true;
  errorBanner.textContent = "";
}

// ── Auto-convert (debounced) ────────────────────────────────────────────────────
let convertTimeout = null;

function triggerAutoConvert() {
  if (convertTimeout) clearTimeout(convertTimeout);
  convertTimeout = setTimeout(async () => {
    await doConvert();
  }, 300);
}

// ── Load formats on startup ───────────────────────────────────────────────────
async function loadFormats() {
  try {
    formats = await invokeCommand("get_formats");
    if (!formats || !Array.isArray(formats)) {
      showError(`Unexpected response from get_formats: ${JSON.stringify(formats)}`);
      return;
    }
    formatTogglesEl.innerHTML = "";
    selectedFormats = new Set(formats.map(f => f.id)); // all selected by default

    for (const f of formats) {
      const btn = document.createElement("button");
      btn.className = "format-toggle active";
      btn.textContent = f.name;
      btn.dataset.id = f.id;
      btn.addEventListener("click", () => toggleFormat(f.id, btn));
      formatTogglesEl.appendChild(btn);
    }
  } catch (err) {
    showError(`Failed to load formats: ${err.message || err}`);
  }
}

function toggleFormat(id, btn) {
  if (selectedFormats.has(id)) {
    selectedFormats.delete(id);
    btn.classList.remove("active");
  } else {
    selectedFormats.add(id);
    btn.classList.add("active");
  }
  triggerAutoConvert();
}

// ── Convert ───────────────────────────────────────────────────────────────────
async function doConvert() {
  clearError();
  const input = inputEl.value.trim();

  if (!input || selectedFormats.size === 0) {
    outputsContainer.innerHTML = "";
    return;
  }

  outputsContainer.innerHTML = "";
  const formatIds = Array.from(selectedFormats);

  try {
    const results = await Promise.all(
      formatIds.map(formatId =>
        invokeCommand("convert_pattern", { input, formatId })
          .then(result => ({ formatId, result }))
          .catch(err => ({ formatId, error: String(err) }))
      )
    );

    for (const { formatId, result, error } of results) {
      const formatName = formats.find(f => f.id === formatId)?.name ?? formatId;
      const card = document.createElement("section");
      card.className = "output-card";

      if (error) {
        card.innerHTML = `
          <div class="output-header">
            <label class="label">${formatName}</label>
          </div>
          <div class="error-banner" style="margin:0">${error}</div>
        `;
      } else {
        card.innerHTML = `
          <div class="output-header">
            <label class="label">${formatName}</label>
            <button class="btn btn--ghost copy-btn" title="Copy to clipboard">Copy</button>
          </div>
          <pre class="output-pre">${escapeHtml(result.output)}</pre>
          <div class="stats">
            <span><strong>${result.byte_count}</strong> bytes</span>
            <span><strong>${result.wildcard_count}</strong> full wildcards</span>
            <span>Specificity <strong>${(result.specificity * 100).toFixed(1)}%</strong></span>
          </div>
        `;
        const copyBtn = card.querySelector(".copy-btn");
        copyBtn.addEventListener("click", () => copyToClipboard(result.output, copyBtn));
      }

      outputsContainer.appendChild(card);
    }
  } catch (err) {
    showError(String(err));
  }
}

function escapeHtml(text) {
  const div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}

async function copyToClipboard(text, btn) {
  try {
    await navigator.clipboard.writeText(text);
    btn.textContent = "Copied!";
    setTimeout(() => { btn.textContent = "Copy"; }, 1500);
  } catch {
    const ta = document.createElement("textarea");
    ta.value = text;
    document.body.appendChild(ta);
    ta.select();
    document.execCommand("copy");
    ta.remove();
    btn.textContent = "Copied!";
    setTimeout(() => { btn.textContent = "Copy"; }, 1500);
  }
}

// ── Drag & drop ───────────────────────────────────────────────────────────────
let dragCounter = 0;

dropZone.addEventListener("dragover", (e) => {
  e.preventDefault();
  dropOverlay.hidden = false;
});

dropZone.addEventListener("dragenter", (e) => {
  e.preventDefault();
  dragCounter++;
  dropOverlay.hidden = false;
});

dropZone.addEventListener("dragleave", (e) => {
  e.preventDefault();
  dragCounter--;
  if (dragCounter === 0) dropOverlay.hidden = true;
});

dropZone.addEventListener("drop", (e) => {
  e.preventDefault();
  dragCounter = 0;
  dropOverlay.hidden = true;
  const file = e.dataTransfer?.files?.[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = (ev) => {
    inputEl.value = ev.target.result;
    clearError();
    triggerAutoConvert();
  };
  reader.readAsText(file);
});

// ── Event bindings ────────────────────────────────────────────────────────────
inputEl.addEventListener("input", triggerAutoConvert);

// ── Init ──────────────────────────────────────────────────────────────────────
loadFormats();
