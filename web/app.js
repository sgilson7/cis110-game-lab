// The page draws what the core returns and decides nothing.
//
// The core describes the page (its sections, their items, which checkpoint
// is next and what state each one is in), and this file builds the DOM from
// that description. It knows how to draw each kind of section and nothing
// about what a section means. The current date and hour go to the core as
// text; the page does no date arithmetic (GL-2). The setup tabs go back
// through `act`, the core's one door.
import init, { start, act } from "./pkg/tool_shim.js";

const el = (id) => document.getElementById(id);

function make(tag, attrs, ...kids) {
  const node = document.createElement(tag);
  Object.entries(attrs || {}).forEach(([k, v]) => {
    if (k === "text") node.textContent = v;
    else node.setAttribute(k, v);
  });
  kids.flat().filter(Boolean).forEach((k) => node.append(k));
  return node;
}

// Segments from the core: plain text, b (an interface name), c (a file name).
function segs(list) {
  return (list || []).map((s) => {
    if (s.s === "b") return make("b", { text: s.t });
    if (s.s === "c") return make("code", { text: s.t });
    return document.createTextNode(s.t);
  });
}

const para = (list, attrs) => make("p", attrs, segs(list));

let state = null;
let glossary = "";
let hints = [];

function call(fn, ...args) {
  return JSON.parse(fn(...args));
}

function mark(node) {
  const hint = hints.find((h) => h.term === node.textContent);
  if (hint) {
    node.dataset.term = hint.term;
    node.title = hint.definition;
  }
}

function drawNext(next) {
  const box = el("next");
  box.hidden = !next;
  if (!next) return;
  const head = make("div", { class: "next-head" }, make("span", { text: next.label }), make("span", { text: next.progress }));
  const body = make("div", { class: "next-body" });
  if (next.when) body.append(make("span", { class: "next-when", text: next.when }));
  body.append(make("h2", { text: next.heading }), para(next.what, { class: "next-what" }));
  const todo = make("ul", {}, (next.todo || []).map((t) => make("li", {}, make("span", {}, segs(t)))));
  const rail = make("div", { class: "rail", "aria-hidden": "true" }, next.rail.map((s) => make("span", { "data-status": s })));
  box.replaceChildren(head, body, todo, rail);
  box.dataset.finished = next.finished;
}

function fileCard(item, labels) {
  return make(
    "article",
    { class: "file panel", "data-id": item.id },
    make("div", { class: "file-cap" }, make("span", { class: "glyph", "aria-hidden": "true", text: item.tag }),
      make("div", {}, make("h3", {}, segs(item.title)), make("code", { text: item.file }))),
    make("div", { class: "file-body" }, para(item.text), make("ul", {}, item.bullets.map((b) => make("li", {}, segs(b))))),
    make("div", { class: "file-foot" },
      make("div", {}, make("span", { text: labels.due }), make("b", { text: item.date })),
      make("div", {}, make("span", { text: labels.points }), make("b", { text: item.value }))),
  );
}

function scheduleRow(item) {
  return make(
    "div",
    { class: "row", "data-status": item.status, "data-graded": String(Boolean(item.graded)) },
    make("time", { text: item.date }),
    make("p", { class: "phase" }, item.tag ? make("span", { text: item.tag }) : null, segs(item.title)),
    make("div", {}, para(item.text), item.detail ? para(item.detail, { class: "detail" }) : null),
    item.status_label ? make("span", { class: "status", text: item.status_label }) : make("span"),
  );
}

function setupBlock(sec) {
  const tabs = make("div", { class: "tabs", role: "tablist", "aria-label": sec.heading });
  sec.tabs.forEach((t) => {
    const b = make("button", { id: `tab-${t.id}`, role: "tab", type: "button", "aria-selected": String(t.selected), text: t.label });
    b.addEventListener("click", () => draw(call(act, JSON.stringify(state), glossary, "setup.show", t.id), b.id));
    tabs.append(b);
  });
  // One panel per open tab: one on the web page, every tab in the print edition.
  const panels = sec.tabs.filter((t) => t.selected).map((open) => make("div", { role: "tabpanel", "aria-labelledby": `tab-${open.id}` },
    open.heading ? make("h3", { class: "tab-heading", text: open.heading }) : null,
    open.lede.length ? para(open.lede, { class: "tab-lede" }) : null,
    make("ol", { class: "steps" }, open.items.map((i) =>
      make("li", {}, make("div", {}, make("b", { class: "step" }, segs(i.title)), para(i.text)))))));
  const f = sec.figure;
  const figure = f ? make("div", {},
    make("p", { class: "figure-label", text: f.label }),
    make("div", { class: "folder panel" },
      make("div", { class: "folder-bar", text: f.path }),
      make("div", { class: "folder-files" },
        make("div", {}, make("span", { class: "sq" }), make("b", { text: f.folder })),
        f.files.map((file) => make("div", {}, make("span", { class: "sq", "data-kind": file.kind }), make("span", { text: file.name })))),
      make("div", { class: "sharing" }, f.sharing.map((s) => make("div", {}, make("b", { text: s.who }), make("span", { text: s.access })))))) : null;
  return make("div", { class: "setup" }, make("div", { class: "panels" }, tabs, panels), figure);
}

function body(sec) {
  switch (sec.kind) {
    case "files":
      return [
        make("div", { class: "files" }, sec.items.map((i) => fileCard(i, sec.labels))),
        make("div", { class: "downloads" }, sec.downloads.map((d) =>
          make("a", { href: d.href, download: "" }, make("b", {}, segs(d.title)), make("span", {}, segs(d.text))))),
      ];
    case "schedule":
      return [make("div", { class: "rows" }, sec.items.map(scheduleRow))];
    case "flow":
      return [make("ol", { class: "cards" }, sec.items.map((i) =>
        make("li", { class: "panel", "data-owner": i.owner }, make("span", { class: "who", text: i.owner_label }),
          make("b", {}, segs(i.title)), para(i.text))))];
    case "setup":
      return [setupBlock(sec)];
    case "roles":
      return [make("div", { class: "roles" }, sec.items.map((i) =>
        make("div", { class: "role panel" }, make("h3", {}, segs(i.title)), para(i.text), make("code", { text: i.file }))))];
    case "rubric":
      return [make("div", { class: "rubric panel" },
        sec.items.map((i) => make("div", { class: "rb" }, make("span", {}, segs(i.title)),
          make("progress", { value: i.value, max: i.max, "aria-hidden": "true" }), make("b", { text: i.value }))),
        make("div", { class: "rb total" }, make("span", { text: sec.labels.total }), make("b", { text: sec.total })))];
    case "rules":
      return [make("div", { class: "after" },
        make("dl", { class: "rules" }, sec.items.map((i) => make("div", { class: "panel" }, make("dt", {}, segs(i.title)), make("dd", {}, segs(i.text))))),
        make("div", {}, make("p", { class: "figure-label", text: sec.gallery_label }),
          make("div", { class: "gallery" }, sec.gallery.map((g) =>
            make("a", { class: "game panel", href: g.href, rel: "noopener" }, make("img", { src: g.image, alt: g.alt, loading: "lazy" }),
              make("span", {}, make("b", {}, segs(g.title)), make("small", {}, segs(g.text))))))))];
    case "faq":
      return [make("div", { class: "faq" }, sec.items.map((i) => make("details", i.open ? { open: "" } : {}, make("summary", {}, segs(i.title)), para(i.text))))];
    default:
      return [];
  }
}

function section(sec) {
  return make(
    "section",
    { class: sec.band ? "sec band" : "sec", id: sec.id, "data-kind": sec.kind, "aria-labelledby": `h-${sec.id}` },
    make("div", { class: "sec-rail" }, make("p", { class: "eyebrow", text: sec.eyebrow })),
    make("div", { class: "sec-body" },
      make("h2", { id: `h-${sec.id}`, text: sec.heading }),
      sec.lede.map((l) => para(l, { class: "lede" })),
      body(sec),
      sec.note.length ? para(sec.note, { class: sec.kind === "schedule" ? "callout" : "sec-note" }) : null),
  );
}

function draw(result, focus) {
  if (result.error) {
    el("message").textContent = result.error;
    return;
  }
  const ui = result.ok;
  const page = ui.page;
  state = ui.state;
  hints = ui.hints;
  el("course").textContent = page.course;
  el("nav").replaceChildren(...page.nav.map((n) => make("a", { href: n.href, text: n.label })));
  el("eyebrow").replaceChildren(...segs(page.intro.eyebrow));
  el("title").textContent = page.title;
  el("intro").replaceChildren(...page.intro.paragraphs.map((p) => para(p)));
  el("intro-note").replaceChildren(...segs(page.intro.note));
  el("editions").replaceChildren(...page.intro.links.map((l) => make("a", { href: l.href, text: l.label })));
  el("edition-note").replaceChildren(...segs(page.intro.edition_note));
  document.body.dataset.edition = page.edition;
  drawNext(page.next);
  el("stats").replaceChildren(...page.stats.map((s) => make("div", { class: "stat" }, make("b", { text: s.value }), make("span", { text: s.label }))));
  el("sections").replaceChildren(...page.sections.map(section));
  el("footer").replaceChildren(...page.footer.map((f) => para(f)));
  document.querySelectorAll("#sections h2, #sections h3, summary").forEach(mark);
  el("message").textContent = "";
  document.body.dataset.screen = ui.screen;
  if (focus) el(focus)?.focus();
}

async function main() {
  await init();
  const strings = await (await fetch("data/strings.json")).text();
  const schedule = await (await fetch("data/schedule.json")).text();
  const clock = JSON.parse(await (await fetch("data/clock.json")).text());
  glossary = await (await fetch("data/glossary.md")).text();
  const now = new Date().toLocaleString(clock.locale, { timeZone: clock.zone });
  const params = new URLSearchParams(location.search);
  draw(call(start, strings, schedule, glossary, now, params.get("on") || "", params.get("edition") || ""));
  if (location.hash) el(location.hash.slice(1))?.scrollIntoView();
  document.body.dataset.ready = "true";
}

main();
