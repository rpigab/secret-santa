/* Secret Santa — interactive graph UI
 * ------------------------------------------------------------------
 * Names become floating nodes. Drag a heart between two nodes to make
 * a couple. Click a node to open its "already gifted before" bubble and
 * drop other names into it. The matching itself is done by the Rust/WASM
 * solver (`solve`); results are rendered as obfuscated per-person links
 * (`show_result_html`). When "Reveal matches" (debug) is on, the real
 * who-gifts-whom is drawn as arrows using `show_result_debug`.
 *
 * The WASM module is loaded lazily so the graph UI still works (drag,
 * couples, bubbles, YAML editing) even before/without the solver. The
 * loader resolves a single path: ./pkg/secret_santa_wasm.js — the build
 * artifact produced by `wasm-pack build --target web --out-dir ./web/pkg`.
 * ------------------------------------------------------------------ */

/* ───────────────────────── i18n ───────────────────────── */
const translations = {
  en: {
    'brand.subtitle': 'Gift exchange',
    'lang.toggle': 'FR',
    'add.label': 'Add a person',
    'add.input': 'Type a name…',
    'add.button': 'Add',
    'counts.people': 'people',
    'counts.couples': 'couples',
    'help.label': 'How it works',
    'help.drag': 'Drag a name to float it anywhere.',
    'help.heart': 'Drag the heart onto a friend to pair them (they won\'t gift each other).',
    'help.bubble': 'Click a name, then drop other names into the bubble to log past gifts.',
    'actions.draw': '🎲 Draw the names',
    'actions.tidy': '✨ Tidy into a circle',
    'actions.reveal': 'Reveal matches',
    'actions.debug': 'debug · secret in real life',
    'yaml.title': 'Advanced — edit YAML config',
    'yaml.header': 'YAML config',
    'yaml.close': 'Close',
    'yaml.method': 'Solve method',
    'yaml.backtracking': 'Backtracking · recommended',
    'yaml.quatuors': 'Split quatuors · groups of 4',
    'yaml.naive': 'Naive · ≤10 people',
    'yaml.example': 'Example',
    'yaml.apply': 'Apply',
    'yaml.copy': 'Copy',
    'bubble.title': 'already gifted',
    'bubble.subtitle': 'already gifted to',
    'bubble.dropzone': 'Drop a name here',
    'error.min': 'Add at least 3 people to draw names.',
    'error.loading': 'The solver is still loading — try again in a moment.',
    'error.notfound': 'No participants found in YAML.',
    'status.sealed': 'secret links sealed',
    'status.sealed_desc': 'Send each person their own link — the recipient stays hidden in the URL.',
    'status.whosgifts': 'Who gifts whom',
    'node.heart': 'Drag onto a friend to pair',
    'node.remove': 'Remove',
  },
  fr: {
    'brand.subtitle': 'Tirage au sort',
    'lang.toggle': 'EN',
    'add.label': 'Ajouter une personne',
    'add.input': 'Tapez un nom…',
    'add.button': 'Ajouter',
    'counts.people': 'personnes',
    'counts.couples': 'couples',
    'help.label': 'Comment ça marche',
    'help.drag': 'Faites glisser un nom n\'importe où.',
    'help.heart': 'Faites glisser le cœur sur un ami pour les associer (ils ne s\'offriront pas de cadeau).',
    'help.bubble': 'Cliquez sur un nom, puis déposez d\'autres noms dans la bulle pour enregistrer les cadeaux passés.',
    'actions.draw': '🎲 Tirage des noms',
    'actions.tidy': '✨ Tidy dans un cercle',
    'actions.reveal': 'Révéler les correspondances',
    'actions.debug': 'debug · secret dans la vraie vie',
    'yaml.title': 'Avancé — modifier la config YAML',
    'yaml.header': 'Config YAML',
    'yaml.close': 'Fermer',
    'yaml.method': 'Méthode de résolution',
    'yaml.backtracking': 'Backtracking · recommandé',
    'yaml.quatuors': 'Diviser par quatuors · groupes de 4',
    'yaml.naive': 'Naïf · ≤10 personnes',
    'yaml.example': 'Exemple',
    'yaml.apply': 'Appliquer',
    'yaml.copy': 'Copier',
    'bubble.title': 'a déjà offert à',
    'bubble.subtitle': 'a déjà offert à',
    'bubble.dropzone': 'Déposez un nom ici',
    'error.min': 'Ajoutez au moins 3 personnes pour faire le tirage.',
    'error.loading': 'Le solveur se charge toujours — réessayez dans un instant.',
    'error.notfound': 'Aucun participant trouvé dans le YAML.',
    'status.sealed': 'liens secrets scellés',
    'status.sealed_desc': 'Envoyez à chaque personne son propre lien — le destinataire reste caché dans l\'URL.',
    'status.whosgifts': 'Qui offre à qui',
    'node.heart': 'Faites glisser sur un ami pour associer',
    'node.remove': 'Supprimer',
  }
};

let currentLang = localStorage.getItem('lang') || getPreferredLanguage();

function getPreferredLanguage() {
  const browserLang = navigator.language || navigator.userLanguage;
  return browserLang.startsWith('fr') ? 'fr' : 'en';
}

function t(key) {
  return translations[currentLang][key] || translations['en'][key] || key;
}

function setLanguage(lang) {
  currentLang = lang;
  localStorage.setItem('lang', lang);
  document.documentElement.lang = lang;
  updateTranslations();
}

function updateTranslations() {
  document.querySelectorAll('[data-i18n]').forEach(el => {
    el.textContent = t(el.getAttribute('data-i18n'));
  });
  document.querySelectorAll('[data-i18n-attr]').forEach(el => {
    const attr = el.getAttribute('data-i18n-attr');
    const [attrName, key] = attr.split(':');
    el.setAttribute(attrName, t(key));
  });
}

function toggleLanguage() {
  setLanguage(currentLang === 'en' ? 'fr' : 'en');
}

/* ───────────────────────── state ───────────────────────── */
const state = {
  nodes: [
    { id: 'n1', name: 'Alice', x: 0, y: 0 },
    { id: 'n2', name: 'Bob',   x: 0, y: 0 },
    { id: 'n3', name: 'Carol', x: 0, y: 0 },
    { id: 'n4', name: 'David', x: 0, y: 0 },
  ],
  couples: [['n1', 'n2'], ['n3', 'n4']],
  gifted: { n1: ['Carol'], n3: ['Bob'] },   // by node id -> [names]
  selectedNode: null,
  method: 'HamiltonianBacktrack',   // solver picked in the cog ⚙ menu
  debug: false,
  assignments: null,   // [{giver, recipient}] from show_result_debug
  links: null,         // [{giver, href}] parsed from show_result_html
  error: null,
};
let nextId = 100;
let wasm = null;          // resolved WASM module (or null)
let gesture = null;       // active pointer gesture
let canvasRect = null;    // cached canvas bounds during a gesture

const EXAMPLE_YAML = `participants:
  - Alice
  - Bob
  - Carol
  - David
already_gifted_before:
  Alice:
    - Carol
  Carol:
    - Bob
couples:
  - - Alice
    - Bob
  - - Carol
    - David`;

/* ───────────────────────── elements ───────────────────────── */
const el = (id) => document.getElementById(id);
const canvas = el('canvas');
const svg = el('svg');
const nodesLayer = el('nodes');
const bubble = el('bubble');
const statusEl = el('status');

/* ───────────────────────── helpers ───────────────────────── */
const nodeById = (id) => state.nodes.find((n) => n.id === id);
const nameOf = (id) => { const n = nodeById(id); return n ? n.name : null; };
const point = (e) => { const r = canvasRect || canvas.getBoundingClientRect(); return { x: e.clientX - r.left, y: e.clientY - r.top }; };

function shuffleInPlace(a) { for (let i = a.length - 1; i > 0; i--) { const j = (Math.random() * (i + 1)) | 0; [a[i], a[j]] = [a[j], a[i]]; } return a; }

/* ───────────────────────── WASM loader ───────────────────────── */
async function loadWasm() {
  try {
    const mod = await import('./pkg/secret_santa_wasm.js');
    if (typeof mod.default === 'function') await mod.default();
    if (typeof mod.init === 'function') mod.init();
    wasm = mod;
  } catch (err) {
    console.warn('Secret Santa: WASM solver unavailable — graph still editable.', err);
    wasm = null;
  }
}

/* ───────────────────────── YAML out ───────────────────────── */
// Quote a scalar only when it could confuse a YAML parser.
function yv(s) {
  if (/^[A-Za-z0-9_][A-Za-z0-9 _'.\-]*$/.test(s) && !/^\s|\s$/.test(s)) return s;
  return JSON.stringify(s);
}
function buildYaml() {
  const names = state.nodes.map((n) => n.name);
  let y = names.length ? 'participants:\n' : 'participants: []\n';
  names.forEach((n) => { y += '  - ' + yv(n) + '\n'; });

  const gentries = Object.entries(state.gifted)
    .map(([id, arr]) => [nameOf(id), (arr || []).filter(Boolean)])
    .filter(([g, arr]) => g && arr.length);
  if (gentries.length) {
    y += 'already_gifted_before:\n';
    gentries.forEach(([g, arr]) => { y += '  ' + yv(g) + ':\n'; arr.forEach((r) => { y += '    - ' + yv(r) + '\n'; }); });
  }

  const couples = state.couples.map((c) => [nameOf(c[0]), nameOf(c[1])]).filter(([a, b]) => a && b);
  if (couples.length) {
    y += 'couples:\n';
    couples.forEach(([a, b]) => { y += '  - - ' + yv(a) + '\n' + '    - ' + yv(b) + '\n'; });
  }
  return y;
}

/* ───────────────────────── YAML in (minimal, schema-specific) ─────────────────────────
 * Handles the project's schema: participants (list), already_gifted_before
 * (map name -> list) and couples (list of pairs). Swap in js-yaml for a
 * fully spec-compliant parser if needed. */
function unq(s) {
  s = s.trim();
  if ((s.startsWith('"') && s.endsWith('"'))) { try { return JSON.parse(s); } catch (e) { return s.slice(1, -1); } }
  if ((s.startsWith("'") && s.endsWith("'"))) return s.slice(1, -1).replace(/''/g, "'");
  return s;
}
function parseYaml(text) {
  const res = { participants: [], gifted: {}, couples: [] };
  let section = null, curGiver = null, pending = null;
  for (let raw of text.replace(/\t/g, '  ').split('\n')) {
    const line = raw.replace(/\s+$/, '');
    const t = line.trim();
    if (!t || t.startsWith('#')) continue;
    const indent = line.length - line.trimStart().length;

    if (indent === 0 && /^[A-Za-z_][\w]*\s*:/.test(t)) {
      const key = t.slice(0, t.indexOf(':')).trim();
      section = (key === 'participants' || key === 'already_gifted_before' || key === 'couples') ? key : null;
      curGiver = null; pending = null;
      continue;
    }
    if (section === 'participants') {
      if (t.startsWith('- ')) res.participants.push(unq(t.slice(2)));
    } else if (section === 'already_gifted_before') {
      if (t.endsWith(':') && !t.startsWith('-')) { curGiver = unq(t.slice(0, -1)); res.gifted[curGiver] = res.gifted[curGiver] || []; }
      else if (t.startsWith('- ') && curGiver) res.gifted[curGiver].push(unq(t.slice(2)));
    } else if (section === 'couples') {
      if (t.startsWith('- - ')) { pending = [unq(t.slice(4))]; res.couples.push(pending); }
      else if (t.startsWith('- ')) {
        if (pending && pending.length < 2) pending.push(unq(t.slice(2)));
        else { pending = [unq(t.slice(2))]; res.couples.push(pending); }
      }
    }
  }
  return res;
}
function applyParsed(p) {
  const names = [...new Set([
    ...p.participants,
    ...p.couples.flat(),
    ...Object.keys(p.gifted),
    ...Object.values(p.gifted).flat(),
  ].filter(Boolean))];
  if (!names.length) { state.error = 'No participants found in YAML.'; renderStatus(); return; }

  const idByName = {};
  state.nodes = names.map((nm) => { const id = 'n' + (nextId++); idByName[nm] = id; return { id, name: nm, x: 0, y: 0 }; });
  state.couples = p.couples.filter((c) => c.length === 2 && idByName[c[0]] && idByName[c[1]]).map((c) => [idByName[c[0]], idByName[c[1]]]);
  state.gifted = {};
  Object.entries(p.gifted).forEach(([g, arr]) => { if (idByName[g]) state.gifted[idByName[g]] = arr.filter((r) => idByName[r]); });
  state.selectedNode = null; state.assignments = null; state.links = null; state.error = null;
  arrangeCircle();
  renderGraph();
}

/* ───────────────────────── gift background ───────────────────────── */
function buildGifts() {
  const gifts = [
    { l: '6%', t: '12%', s: 56, r: -14, a: '#bfe3c9', b: '#7fc79a' },
    { l: '82%', t: '9%', s: 46, r: 11, a: '#ffd0d6', b: '#ff9aa2' },
    { l: '90%', t: '62%', s: 60, r: -8, a: '#ffe6a8', b: '#ffcf78' },
    { l: '4%', t: '70%', s: 50, r: 9, a: '#cfc2f0', b: '#b9a4ec' },
    { l: '46%', t: '4%', s: 40, r: 16, a: '#bfe3ff', b: '#9ccbff' },
    { l: '68%', t: '84%', s: 52, r: -13, a: '#bdeccf', b: '#8fd6ad' },
    { l: '24%', t: '88%', s: 44, r: 7, a: '#ffd0d6', b: '#ff9aa2' },
    { l: '12%', t: '40%', s: 38, r: -10, a: '#ffe6a8', b: '#ffcf78' },
    { l: '94%', t: '34%', s: 42, r: 13, a: '#cfc2f0', b: '#b9a4ec' },
    { l: '56%', t: '92%', s: 46, r: -6, a: '#bfe3c9', b: '#7fc79a' },
  ];
  el('gifts').innerHTML = gifts.map((g) => `
    <div class="gift" style="left:${g.l};top:${g.t};width:${g.s}px;height:${g.s}px;transform:rotate(${g.r}deg)">
      <div class="box" style="background:linear-gradient(135deg,${g.a},${g.b})"></div>
      <div class="rv"></div><div class="rh"></div><div class="knot"></div>
    </div>`).join('');
}

/* ───────────────────────── SVG link layer ───────────────────────── */
function esc(s) { return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;'); }
function drawSvg() {
  const byId = {}; state.nodes.forEach((n) => { byId[n.id] = n; });
  const byName = {}; state.nodes.forEach((n) => { byName[n.name] = n; });
  const R = 48;
  let s = '<defs><marker id="ssarrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#5cab86"/></marker></defs>';

  if (state.debug && state.assignments) {
    state.assignments.forEach((m) => {
      const a = byName[m.giver], b = byName[m.recipient];
      if (!a || !b) return;
      const dx = b.x - a.x, dy = b.y - a.y, len = Math.hypot(dx, dy) || 1, ux = dx / len, uy = dy / len;
      const sx = a.x + ux * R, sy = a.y + uy * R, ex = b.x - ux * (R + 7), ey = b.y - uy * (R + 7);
      const mx = (sx + ex) / 2 - uy * 28, my = (sy + ey) / 2 + ux * 28;
      s += `<path d="M${sx},${sy} Q${mx},${my} ${ex},${ey}" fill="none" stroke="#5cab86" stroke-width="3" stroke-linecap="round" marker-end="url(#ssarrow)" opacity="0.85"/>`;
    });
  }

  state.couples.forEach((c, i) => {
    const a = byId[c[0]], b = byId[c[1]];
    if (!a || !b) return;
    const mx = (a.x + b.x) / 2, my = (a.y + b.y) / 2;
    s += `<line x1="${a.x}" y1="${a.y}" x2="${b.x}" y2="${b.y}" stroke="#ff9bb0" stroke-width="3.5" stroke-dasharray="1 9" stroke-linecap="round" opacity="0.85"/>`;
    s += `<text x="${mx}" y="${my + 8}" text-anchor="middle" font-size="25" fill="#ff7f9c" data-couple="${i}" style="cursor:pointer;pointer-events:auto">♥</text>`;
  });

  if (gesture && gesture.type === 'link' && gesture.cursor) {
    const a = byId[gesture.id];
    if (a) {
      s += `<line x1="${a.x}" y1="${a.y}" x2="${gesture.cursor.x}" y2="${gesture.cursor.y}" stroke="#ff9bb0" stroke-width="3.5" stroke-dasharray="1 9" stroke-linecap="round"/>`;
      s += `<text x="${gesture.cursor.x}" y="${gesture.cursor.y + 8}" text-anchor="middle" font-size="25" fill="#ff7f9c">♥</text>`;
    }
  }
  svg.innerHTML = s;
}

/* ───────────────────────── node rendering ───────────────────────── */
function renderGraph() {
  nodesLayer.innerHTML = '';
  state.nodes.forEach((n, i) => {
    const node = document.createElement('div');
    node.className = 'node node-c' + (i % 6) + (state.selectedNode === n.id ? ' sel' : '');
    node.style.left = n.x + 'px';
    node.style.top = n.y + 'px';
    node.dataset.id = n.id;

    const pill = document.createElement('div');
    pill.className = 'node-pill';
    pill.style.animationDelay = (i * 0.45) + 's';
    pill.textContent = n.name;

    const heart = document.createElement('div');
    heart.className = 'node-heart'; heart.textContent = '♥'; heart.title = 'Drag onto a friend to pair';

    const rm = document.createElement('div');
    rm.className = 'node-remove'; rm.textContent = '×'; rm.title = 'Remove';

    node.append(pill, heart, rm);
    node.addEventListener('pointerdown', (e) => onNodeDown(n.id, e));
    heart.addEventListener('pointerdown', (e) => onHeartDown(n.id, e));
    rm.addEventListener('pointerdown', (e) => onRemoveNode(n.id, e));
    nodesLayer.appendChild(node);
  });
  positionNodes();
  drawSvg();
  renderBubble();
  el('countPeople').textContent = state.nodes.length;
  el('countCouples').textContent = state.couples.length;
}
function positionNodes() {
  const map = {}; state.nodes.forEach((n) => { map[n.id] = n; });
  nodesLayer.querySelectorAll('.node').forEach((node) => {
    const n = map[node.dataset.id];
    if (n) { node.style.left = n.x + 'px'; node.style.top = n.y + 'px'; }
  });
}
function nodeElById(id) { return nodesLayer.querySelector(`.node[data-id="${id}"]`); }

/* ───────────────────────── bubble ───────────────────────── */
function renderBubble() {
  const n = state.selectedNode ? nodeById(state.selectedNode) : null;
  if (!n) { bubble.hidden = true; return; }
  const cw = canvas.clientWidth, ch = canvas.clientHeight;
  const goRight = n.x < cw - 300;
  const left = goRight ? Math.min(n.x + 62, cw - 262) : Math.max(n.x - 300, 12);
  const top = Math.max(12, Math.min(n.y - 46, ch - 210));
  bubble.style.left = left + 'px';
  bubble.style.top = top + 'px';
  bubble.style.transformOrigin = goRight ? 'left center' : 'right center';

  const list = state.gifted[n.id] || [];
  bubble.innerHTML = `
    <div class="bubble-head">
      <div class="bubble-title">🎁 ${esc(n.name)} already gifted<br><span style="font-weight:500;color:#9aa89e;font-size:12px">a déjà offert à</span></div>
      <div class="bubble-close" data-close="1">×</div>
    </div>
    <div class="chips">${list.map((nm) => `<span class="chip">${esc(nm)}<span class="x" data-rm="${esc(nm)}">×</span></span>`).join('')}</div>
    <div class="dropzone">Drop a name here · Glissez un nom ↧</div>`;
  bubble.hidden = false;
}

/* ───────────────────────── gestures ───────────────────────── */
function onNodeDown(id, e) {
  e.preventDefault(); e.stopPropagation();
  const n = nodeById(id);
  canvasRect = canvas.getBoundingClientRect();
  const p = point(e);
  gesture = { type: 'node', id, down: p, origin: { x: n.x, y: n.y }, offx: p.x - n.x, offy: p.y - n.y, moved: false };
}
function onHeartDown(id, e) {
  e.preventDefault(); e.stopPropagation();
  canvasRect = canvas.getBoundingClientRect();
  const p = point(e);
  gesture = { type: 'link', id, down: p, cursor: p, moved: false };
  drawSvg();
}
function onRemoveNode(id, e) {
  e.preventDefault(); e.stopPropagation();
  const nm = nameOf(id);
  delete state.gifted[id];
  Object.keys(state.gifted).forEach((k) => { state.gifted[k] = state.gifted[k].filter((x) => x !== nm); });
  state.nodes = state.nodes.filter((n) => n.id !== id);
  state.couples = state.couples.filter((c) => c[0] !== id && c[1] !== id);
  if (state.selectedNode === id) state.selectedNode = null;
  state.assignments = null; state.links = null;
  renderGraph(); renderStatus();
}

function onPointerMove(e) {
  if (!gesture) return;
  const p = point(e);
  if (Math.hypot(p.x - gesture.down.x, p.y - gesture.down.y) > 5) gesture.moved = true;

  if (gesture.type === 'node') {
    const n = nodeById(gesture.id);
    if (!n) return;
    n.x = p.x - gesture.offx; n.y = p.y - gesture.offy;
    const node = nodeElById(gesture.id);
    if (node) { node.style.left = n.x + 'px'; node.style.top = n.y + 'px'; }
    drawSvg();
    if (state.selectedNode && state.selectedNode !== gesture.id) {
      bubble.classList.toggle('drop', overBubble(e));
    }
  } else if (gesture.type === 'link') {
    gesture.cursor = p;
    drawSvg();
  }
}
function onPointerUp(e) {
  if (!gesture) return;
  const g = gesture; gesture = null; canvasRect = null;

  if (g.type === 'node') {
    if (!g.moved) {
      state.selectedNode = (state.selectedNode === g.id) ? null : g.id;
      renderGraph();
    } else if (state.selectedNode && state.selectedNode !== g.id && overBubble(e)) {
      const nm = nameOf(g.id), tgt = state.selectedNode;
      const cur = state.gifted[tgt] || [];
      if (!cur.includes(nm)) state.gifted[tgt] = [...cur, nm];
      const n = nodeById(g.id); n.x = g.origin.x; n.y = g.origin.y;   // snap back — stays available
      state.assignments = null; state.links = null;
      bubble.classList.remove('drop');
      renderGraph(); renderStatus();
    } else {
      bubble.classList.remove('drop');
    }
  } else if (g.type === 'link') {
    const hit = nodeAt(e);
    if (hit && hit !== g.id) addCouple(g.id, hit);
    drawSvg();
  }
}
function overBubble(e) {
  if (bubble.hidden) return false;
  const r = bubble.getBoundingClientRect();
  return e.clientX >= r.left && e.clientX <= r.right && e.clientY >= r.top && e.clientY <= r.bottom;
}
function nodeAt(e) {
  const p = point(e); let best = null, bd = 1e9;
  state.nodes.forEach((n) => { const d = Math.hypot(n.x - p.x, n.y - p.y); if (d < 56 && d < bd) { bd = d; best = n.id; } });
  return best;
}
function addCouple(a, b) {
  if (state.couples.some((c) => (c[0] === a && c[1] === b) || (c[0] === b && c[1] === a))) return;
  state.couples.push([a, b]);
  state.assignments = null; state.links = null;
  renderGraph(); renderStatus();
}
function removeCouple(i) {
  state.couples.splice(i, 1);
  state.assignments = null; state.links = null;
  renderGraph(); renderStatus();
}

/* ───────────────────────── layout ───────────────────────── */
function arrangeCircle() {
  const cw = canvas.clientWidth || 820, ch = canvas.clientHeight || 560;
  const cx = cw / 2, cy = ch / 2, R = Math.min(cx, cy) - 95;
  state.nodes.forEach((n, i) => {
    const ang = -Math.PI / 2 + (i * 2 * Math.PI) / state.nodes.length;
    n.x = cx + R * Math.cos(ang);
    n.y = cy + R * Math.sin(ang);
  });
}

function sortCircleCounterclockwise() {
  if (!state.assignments || state.assignments.length === 0) return;

  const cw = canvas.clientWidth || 820, ch = canvas.clientHeight || 560;
  const cx = cw / 2, cy = ch / 2;
  const byName = {};
  state.nodes.forEach(n => { byName[n.name] = n; });

  const angles = {};
  state.nodes.forEach(n => { angles[n.id] = 0; });

  state.assignments.forEach(m => {
    const giver = byName[m.giver], recipient = byName[m.recipient];
    if (!giver || !recipient) return;

    const giverAngle = Math.atan2(giver.y - cy, giver.x - cx);
    const recipientAngle = Math.atan2(recipient.y - cy, recipient.x - cx);

    let angleDiff = recipientAngle - giverAngle;
    while (angleDiff < 0) angleDiff += 2 * Math.PI;
    while (angleDiff >= 2 * Math.PI) angleDiff -= 2 * Math.PI;

    angles[giver.id] = (angles[giver.id] + angleDiff) / (angles[giver.id] === 0 ? 1 : 2);
  });

  const sorted = state.nodes.slice().sort((a, b) => angles[a.id] - angles[b.id]);

  const R = Math.min(cw / 2, ch / 2) - 95;
  sorted.forEach((n, i) => {
    const ang = -Math.PI / 2 + (i * 2 * Math.PI) / sorted.length;
    n.x = cx + R * Math.cos(ang);
    n.y = cy + R * Math.sin(ang);
  });
}

/* ───────────────────────── add a name ───────────────────────── */
function addName() {
  const input = el('nameInput');
  const nm = input.value.trim();
  if (!nm) return;
  if (state.nodes.some((n) => n.name.toLowerCase() === nm.toLowerCase())) { input.value = ''; return; }
  const cw = canvas.clientWidth, ch = canvas.clientHeight;
  const ang = Math.random() * Math.PI * 2, rad = 120 + Math.random() * 90;
  state.nodes.push({ id: 'n' + (nextId++), name: nm, x: cw / 2 + Math.cos(ang) * rad, y: ch / 2 + Math.sin(ang) * rad });
  input.value = '';
  state.assignments = null; state.links = null;
  renderGraph(); renderStatus();
}

/* ───────────────────────── solve ───────────────────────── */
function parseLinks(html) {
  const tmp = document.createElement('div');
  tmp.innerHTML = html;
  return [...tmp.querySelectorAll('a')].map((a) => ({ giver: a.textContent.trim(), href: a.getAttribute('href') }));
}
function draw() {
  state.error = null;
  if (state.nodes.length < 3) { state.error = 'Add at least 3 people to draw names.'; renderStatus(); return; }
  if (!wasm) { state.error = 'The solver is still loading — try again in a moment.'; renderStatus(); return; }

  const yaml = buildYaml();
  let sol = null;
  try {
    sol = wasm.solve(yaml, state.method);
    // Both readers take &WasmSolution, so the same draw powers links + arrows.
    state.assignments = JSON.parse(wasm.show_result_debug(sol));
    state.links = parseLinks(wasm.show_result_html(sol));
    state.error = null;
  } catch (err) {
    state.assignments = null; state.links = null;
    state.error = String((err && err.message) || err);
  } finally {
    if (sol && typeof sol.free === 'function') sol.free();
  }
  renderStatus(); drawSvg();
}

/* ───────────────────────── status / results ───────────────────────── */
function renderStatus() {
  let html = '';
  if (state.error) html += `<div class="note err">⚠️ ${esc(state.error)}</div>`;

  if (state.debug && state.assignments) {
    html += `<div class="section-label">Who gifts whom · Tirage révélé</div>`;
    html += state.assignments.map((m) =>
      `<div class="match-row"><span>${esc(m.giver)}</span><span class="arr">→</span><span>${esc(m.recipient)}</span></div>`).join('');
  } else if (!state.debug && state.links) {
    html += `<div class="note sealed">🔒 ${state.links.length} secret links sealed<small>Send each person their own link — the recipient stays hidden in the URL.</small></div>`;
    html += state.links.map((l, i) =>
      `<div class="link-row"><span class="name">${esc(l.giver)}</span>` +
      `<button class="copy-btn" data-copy="${i}">Copy link</button>` +
      `<a class="open-link" href="${esc(l.href)}" target="_blank" rel="noopener" title="Open">↗</a></div>`).join('');
  }
  statusEl.innerHTML = html;
}

/* ───────────────────────── wiring ───────────────────────── */
function setDebug(on) {
  state.debug = on;
  el('debugToggle').classList.toggle('on', on);
  el('debugToggle').setAttribute('aria-pressed', String(on));
  renderStatus(); drawSvg();
}

function init() {
  setLanguage(currentLang);
  buildGifts();
  arrangeCircle();
  renderGraph();

  el('addBtn').addEventListener('click', addName);
  el('nameInput').addEventListener('keydown', (e) => { if (e.key === 'Enter') addName(); });
  el('drawBtn').addEventListener('click', draw);
  el('tidyBtn').addEventListener('click', () => { arrangeCircle(); sortCircleCounterclockwise(); renderGraph(); });
  el('debugToggle').addEventListener('click', () => setDebug(!state.debug));
  el('langToggle').addEventListener('click', toggleLanguage);

  // canvas: click empty space closes bubble
  canvas.addEventListener('pointerdown', () => { if (state.selectedNode) { state.selectedNode = null; renderGraph(); } });
  window.addEventListener('pointermove', onPointerMove);
  window.addEventListener('pointerup', onPointerUp);

  // delegated: couple-heart removal
  svg.addEventListener('click', (e) => {
    const i = e.target && e.target.getAttribute && e.target.getAttribute('data-couple');
    if (i !== null && i !== undefined) removeCouple(parseInt(i, 10));
  });

  // delegated: bubble close / chip removal
  bubble.addEventListener('pointerdown', (e) => {
    const t = e.target;
    if (t.dataset && t.dataset.close) { e.stopPropagation(); state.selectedNode = null; renderGraph(); return; }
    if (t.dataset && t.dataset.rm) {
      e.stopPropagation();
      const id = state.selectedNode, nm = t.dataset.rm;
      if (id) { state.gifted[id] = (state.gifted[id] || []).filter((x) => x !== nm); state.assignments = null; state.links = null; renderGraph(); renderStatus(); }
    }
  });

  // delegated: copy-link buttons
  statusEl.addEventListener('click', async (e) => {
    const i = e.target && e.target.getAttribute && e.target.getAttribute('data-copy');
    if (i === null || i === undefined || !state.links) return;
    const l = state.links[parseInt(i, 10)];
    const abs = new URL(l.href, location.href).href;
    try { await navigator.clipboard.writeText(abs); } catch (_) { /* ignore */ }
    e.target.textContent = 'Copied!'; e.target.classList.add('done');
    setTimeout(() => { e.target.textContent = 'Copy link'; e.target.classList.remove('done'); }, 1400);
  });

  // cog / YAML drawer
  const drawer = el('yamlDrawer');
  el('cogBtn').addEventListener('click', () => {
    if (drawer.hidden) { el('yamlText').value = buildYaml(); drawer.hidden = false; } else drawer.hidden = true;
  });
  el('yamlClose').addEventListener('click', () => { drawer.hidden = true; });
  el('yamlExample').addEventListener('click', () => { el('yamlText').value = EXAMPLE_YAML; });
  el('yamlApply').addEventListener('click', () => { applyParsed(parseYaml(el('yamlText').value)); drawer.hidden = true; });
  el('yamlCopy').addEventListener('click', async (e) => {
    try { await navigator.clipboard.writeText(el('yamlText').value); e.target.textContent = 'Copied!'; setTimeout(() => { e.target.textContent = 'Copy'; }, 1200); } catch (_) {}
  });

  // solve-method dropdown (re-draws so the new method takes effect immediately)
  const methodSelect = el('methodSelect');
  methodSelect.value = state.method;
  methodSelect.addEventListener('change', (e) => {
    state.method = e.target.value;
    state.assignments = null; state.links = null;
    if (wasm && state.nodes.length >= 3) draw(); else renderStatus();
  });

  window.addEventListener('resize', () => renderBubble());

  loadWasm();
}

init();
