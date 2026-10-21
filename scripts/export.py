#!/usr/bin/env python3
"""Make the printable editions of the built page: a PDF and a single-file HTML.

Both come from the core's print edition (index.html?edition=print), drawn by
the page in Chromium, so they say exactly what the web page says, minus the
date: no next-checkpoint card, every setup tab and FAQ answer open. This script
decides nothing about content; it copies what the page drew and packs it.

- dist/<name>.html: one file with no script. The stylesheet, the gallery
  pictures and the four downloads are inside it (data: URLs), so it opens from
  a Blackboard file item, an email attachment or a laptop's Downloads folder,
  with no request to any server.
- dist/<name>.pdf: the same page printed to US Letter. Its download links go to
  the web site (strings.json edition.site), since a PDF can't hold files.

The file names come from strings.json's edition.links, which the web page
links to. It does this for each version of the page in SITES: the game
version at the root and the tool version in tool/ (T14). A link to a folder,
such as the other version, points at the web site in both files.
Run after scripts/build.sh; scripts/check.sh does.

    python3 scripts/export.py
"""
import base64
import functools
import html
import http.server
import json
import mimetypes
import pathlib
import urllib.parse
import re
import sys
import threading

from playwright.sync_api import sync_playwright

ROOT = pathlib.Path(__file__).resolve().parents[1]
DIST = ROOT / "dist"
# (folder in dist/, the strings file that version is drawn from)
SITES = [("", "data/strings.json"), ("tool/", "data/tool/strings.json")]
CSP = "default-src 'none'; style-src 'unsafe-inline'; img-src data:; base-uri 'none'; form-action 'none'"
MIME = {
    ".docx": "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    ".xlsx": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    ".pptx": "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    ".zip": "application/zip",
}


class Quiet(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".wasm": "application/wasm", ".js": "text/javascript"}

    def log_message(self, *args):
        pass


class Site:
    """One version of the page: its folder in dist/, its web address, and the
    names of its printable files."""

    def __init__(self, folder, strings):
        self.folder = folder
        self.strings = json.loads((ROOT / strings).read_text())
        self.url = self.strings["edition"]["site"]
        self.names = {pathlib.Path(l["href"]).suffix: l["href"] for l in self.strings["edition"]["links"]}
        self.dir = DIST / folder

    def local(self, path):
        """The file a relative URL names, without its ?v= stamp."""
        return (self.dir / html.unescape(path).split("?")[0]).resolve()

    def web(self, path):
        return urllib.parse.urljoin(self.url, html.unescape(path).split("?")[0])


def drawn_print_edition(site):
    """The print edition's DOM, as the page drew it."""
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(Quiet, directory=str(DIST)))
    threading.Thread(target=server.serve_forever, daemon=True).start()
    host, port = server.server_address[:2]
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()
        errors = []
        page.on("pageerror", lambda e: errors.append(str(e)))
        page.goto(f"http://{host}:{port}/{site.folder}index.html?edition=print")
        page.wait_for_selector("body[data-ready='true']", state="attached", timeout=20000)
        if page.inner_text("#message") or errors:
            sys.exit(f"the print edition reported a problem: {page.inner_text('#message')} {errors}")
        dom = page.content()
        browser.close()
    server.shutdown()
    return dom


def data_url(f):
    kind = MIME.get(f.suffix) or mimetypes.guess_type(f.name)[0] or "application/octet-stream"
    return f"data:{kind};base64,{base64.b64encode(f.read_bytes()).decode()}"


def pack(site, dom, files_inside):
    """The drawn page as one file: no script, styles inline, and relative URLs
    either inside the file (files_inside) or pointing at the web site."""
    dom = re.sub(r"<script\b.*?</script>", "", dom, flags=re.S)
    dom = re.sub(r"<noscript\b.*?</noscript>", "", dom, flags=re.S)
    css = (DIST / "style.css").read_text()
    dom, n = re.subn(r'<link rel="stylesheet" href="(?:\.\./)?style\.css[^"]*">', lambda _: f"<style>\n{css}</style>", dom)
    assert n == 1, "the stylesheet link was not found"
    dom, n = re.subn(r'(<meta http-equiv="Content-Security-Policy" content=")[^"]*"', rf'\g<1>{CSP}"', dom)
    assert n == 1, "the CSP meta element was not found"
    # An empty icon, so a browser doesn't ask a server for /favicon.ico.
    dom = dom.replace("</title>", '</title>\n  <link rel="icon" href="data:,">', 1)
    dom = dom.replace('data-ready="true"', 'data-ready="static"')
    # A download keeps its file name when its URL becomes a data: URL.
    dom = re.sub(
        r'<a href="([^"]+)" download="">',
        lambda m: f'<a href="{m[1]}" download="{pathlib.PurePosixPath(html.unescape(m[1]).split("?")[0]).name}">',
        dom,
    )

    def src(m):
        return f'{m[1]}="{data_url(site.local(m[2]))}"'

    dom = re.sub(r'(src)="((?!data:|https?:|#)[^"]+)"', src, dom)

    def href(m):
        url = m[2]
        if files_inside and site.local(url).is_file():
            return f'{m[1]}="{data_url(site.local(url))}"'
        return f'{m[1]}="{site.web(url)}"'

    dom = re.sub(r'(href)="((?!data:|https?:|#|mailto:)[^"]+)"', href, dom)
    left = re.findall(r'(?:src|href)="(?!data:|https?:|#|mailto:)([^"]+)"', dom)
    assert not left, f"relative URLs left in the export: {left}"
    assert "<script" not in dom, "a script is left in the export"
    return dom


def export(site):
    dom = drawn_print_edition(site)

    standalone = site.dir / site.names[".html"]
    standalone.write_text(pack(site, dom, files_inside=True))

    printed = pack(site, dom, files_inside=False)
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()
        page.set_content(printed, wait_until="load")
        page.emulate_media(media="print", color_scheme="light")
        page.pdf(
            path=str(site.dir / site.names[".pdf"]),
            format="Letter",
            print_background=True,
            margin={"top": "0.6in", "bottom": "0.7in", "left": "0.6in", "right": "0.6in"},
            display_header_footer=True,
            header_template="<span></span>",
            footer_template=(
                '<div style="font: 9px system-ui, sans-serif; color: #555; width: 100%; padding: 0 0.6in;'
                ' display: flex; justify-content: space-between;">'
                f"<span>{html.escape(site.strings['course'])}</span>"
                '<span>Page <span class="pageNumber"></span> of <span class="totalPages"></span></span></div>'
            ),
        )
        browser.close()

    pdf = (site.dir / site.names[".pdf"]).read_bytes()
    pages = len(re.findall(rb"/Type\s*/Page[^s]", pdf))
    print(f"{site.folder}{standalone.name}: {standalone.stat().st_size // 1024} KB, no script")
    print(f"{site.folder}{site.names['.pdf']}: {pages} pages, {len(pdf) // 1024} KB")


def main():
    if not (DIST / "index.html").exists():
        sys.exit("dist/ is missing: run scripts/build.sh first")
    for folder, strings in SITES:
        export(Site(folder, strings))


if __name__ == "__main__":
    main()
