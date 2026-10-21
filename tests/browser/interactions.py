"""The project's own browser interactions, run by check.py in every engine.

check.py has already loaded the page, waited for it to report ready, and let
it idle. These checks drive the page the way a student would and assert what
the page shows. Expected values are worked out by hand from the syllabus
(pages 9 and 23), never copied from the page's own output (C4). The page is
reloaded with ?on= to see it as of a fixed date, so the checks do not depend
on the day they run.
"""
import re
from urllib.parse import urljoin


def ready(page, url):
    page.goto(url)
    page.wait_for_selector("body[data-ready='true']")


def run(page, check, engine):
    base = page.url.split("?")[0].split("#")[0]
    check(page.inner_text("#title") == "Collaborative Game Design Project", "the title is drawn")
    check(page.inner_text("#message") == "", "no error message is shown")

    # 20 November 2026, a Friday. Since T11, C3 to C5 fall on 9, 16 and 23 November, so
    # checkpoints 1-4 have passed and C5 (Mon 23 Nov, the last file before Thanksgiving) is next, 3 days away.
    ready(page, base + "?on=2026-11-20T10:00")
    check(page.inner_text("#next h2") == "Monday, November 23", "the next checkpoint on 20 Nov is Monday, November 23")
    check("Due in 3 days" in page.inner_text("#next"), "it is due in 3 days")
    check(page.locator(".row[data-status='done']").count() == 4, "four checkpoints show as passed")
    check(page.locator(".row[data-status='next'] time").inner_text() == "Mon, Nov 23", "the schedule marks Nov 23 as next")
    check("Graded · 100 points" in page.inner_text(".row[data-graded='true']"), "the Dec 10 row says it is graded, 100 points")

    # The day of a deadline, before and after 5:00 PM Eastern.
    ready(page, base + "?on=2026-11-16T16:59")
    check("Due today at 5:00 PM ET" in page.inner_text("#next"), "at 4:59 PM on Nov 16 the Excel draft is due today")
    ready(page, base + "?on=2026-11-16T17:00")
    check(page.inner_text("#next h2") == "Monday, November 23", "at 5:00 PM on Nov 16 the card moves to Nov 23")

    # After the final deadline.
    ready(page, base + "?on=2026-12-11T09:00")
    check(page.inner_text("#next h2") == "The project is complete", "after Dec 10 the card says the project is complete")

    # The stat strip and the rubric: 3-5 students, 3 files, 7 checkpoints, 100 points; 20+25+20+20+15.
    stats = page.locator(".stat b").all_inner_texts()
    check(stats == ["3–5", "3", "7", "100"], "the stat strip reads 3–5, 3, 7, 100")
    check(page.locator(".rb b").all_inner_texts() == ["20", "25", "20", "20", "15", "100"], "the rubric reads 20, 25, 20, 20, 15, total 100")

    # File cards (T11): Word draft Nov 9 (25 points), Excel Nov 16 (20), PowerPoint Nov 23 (20), all before the Nov 25-27 break.
    feet = page.locator(".file-foot").all_inner_texts()
    check("Mon, Nov 9" in feet[0] and "25" in feet[0], "the Word card says Mon, Nov 9 and 25 points")
    check("Mon, Nov 16" in feet[1], "the Excel card says Mon, Nov 16")
    check("Mon, Nov 23" in feet[2] and "20" in feet[2], "the PowerPoint card says Mon, Nov 23 and 20 points")
    check("Monday, November 30" in page.inner_text(".callout"), "prototype links are planned for Monday, November 30")

    # Setup tabs go through the core: Microsoft 365 first, then the Google Drive backup.
    check(page.get_attribute("#tab-m365", "aria-selected") == "true", "the Microsoft 365 tab is open first")
    check(page.locator("ol.steps li").count() == 8, "the Microsoft 365 tab has eight steps")
    page.click("#tab-google")
    page.wait_for_selector("#tab-google[aria-selected='true']")
    check(page.locator("ol.steps li").count() == 4, "the Google Drive tab has four steps")
    check("Use Google Drive only if" in page.inner_text(".tab-lede"), "the Google Drive tab opens by saying it is the backup")

    # T10: teams form and talk in Blackboard at checkpoint 1; the shared folder comes at checkpoint 2.
    c1 = page.locator(".row").filter(has=page.locator("time", has_text=re.compile(r"^Mon, Oct 26$"))).inner_text()
    c2 = page.locator(".row").filter(has=page.locator("time", has_text=re.compile(r"^Mon, Nov 2$"))).inner_text()
    check("team discussion" in c1 and "folder created" not in c1 and "Create the shared folder" not in c1, "checkpoint 1 is the team discussion, with no folder yet")
    check("shared folder created" in c2 and "Pitch and set up" in c2, "checkpoint 2 creates the shared folder")

    # The gallery: ten example games, described for a public, student audience
    # (T6, T7). Since T14 the tools are on the tool version, which this page points to.
    check(page.locator(".game").count() == 10, "the gallery shows ten games")
    check("tool version of this page" in page.inner_text("#after"), "the page points a tool team to the tool version")
    check(page.locator("#faq summary", has_text="Does it have to be a game?").count() == 1, "the FAQ answers whether it has to be a game")
    check("game version" in page.inner_text("#eyebrow").lower(), "the eyebrow says this is the game version")
    check("third-person action survival game" in page.inner_text("#after"), "Long Time Comin' is described as an action survival game")
    check("bullet" not in page.inner_text("#after").lower(), "no gallery text says bullet hell")

    # The templates are on this site.
    check(page.locator("a[href*='templates/GameDesign.docx']").count() == 1, "the Word template link is present")
    check(page.locator("a[href*='templates/tool/']").count() == 0, "the game version links no tool template (T14)")
    check(page.evaluate("document.documentElement.scrollWidth <= window.innerWidth"), "the page does not scroll sideways")

    # The printable editions (T9). The web page links the PDF and the single-file HTML, and both exist.
    ready(page, base)
    links = page.locator("#editions a")
    check(links.count() == 3, "the intro links two printable files and the tool version")
    for href in [links.nth(i).get_attribute("href") for i in range(3)]:
        check(page.request.get(base.rsplit("/", 1)[0] + "/" + href).status == 200, f"{href} is published")

    # The print edition: no date, so no next-checkpoint card and no Closed or Next label;
    # both setup tabs (8 + 4 steps) and all seven FAQ answers open.
    ready(page, base + "?edition=print&on=2026-11-20T10:00")
    check(page.locator("#next").is_hidden(), "the print edition has no next-checkpoint card")
    check(page.locator(".row[data-status='done'], .row[data-status='next']").count() == 0, "no checkpoint is marked passed or next")
    check(page.locator("[role='tabpanel']").count() == 2 and page.locator("ol.steps li").count() == 12, "both setup tabs are shown, 8 and 4 steps")
    check(page.locator("details[open]").count() == 9, "all nine FAQ answers are open")
    check("sgilson7.github.io/cis110-game-lab" in page.inner_text("#edition-note"), "the print edition says where the web page is")

    # The single-file HTML: no script, and every picture and template inside the file.
    page.goto(base.rsplit("/", 1)[0] + "/cis110-game-project.html")
    check(page.locator("script").count() == 0, "the single-file HTML has no script")
    check(page.inner_text("h1") == "Collaborative Game Design Project", "the single-file HTML shows the title")
    check(page.locator(".game img[src^='data:image/']").count() == 10, "its ten gallery pictures are inside the file")
    check(page.locator(".downloads a[href^='data:']").count() == 4, "its four downloads are inside the file")

    run_tool(page, check, base.rsplit("/", 1)[0] + "/tool/")


def run_tool(page, check, base):
    """The tool version (T14): the same core and dates, with tool words and files."""
    ready(page, base + "index.html?on=2026-11-20T10:00")
    check(page.inner_text("#title") == "Collaborative Tool Design Project", "tool: the title is drawn")
    check(page.inner_text("#message") == "", "tool: no error message is shown")
    check("tool version" in page.inner_text("#eyebrow").lower(), "tool: the eyebrow says this is the tool version")
    # Same schedule as the game version: C5 on Mon 23 Nov is next on 20 Nov, and the rubric totals 100.
    check(page.inner_text("#next h2") == "Monday, November 23", "tool: the next checkpoint on 20 Nov is Monday, November 23")
    check(page.locator(".rb b").all_inner_texts() == ["20", "25", "20", "20", "15", "100"], "tool: the rubric reads 20, 25, 20, 20, 15, total 100")
    check("Tool concept and usable scope" in page.inner_text("#grading"), "tool: the rubric names the tool concept")
    c2 = page.locator(".row").filter(has=page.locator("time", has_text=re.compile(r"^Mon, Nov 2$"))).inner_text()
    check("Tool idea chosen" in c2, "tool: checkpoint 2 reads tool idea chosen")
    check([page.locator(".file code").nth(i).inner_text() for i in range(3)] == ["ToolDesign.docx", "ToolData.xlsx", "ToolPitch.pptx"], "tool: the file cards name the tool files")
    check("I build your tools over the break" in page.inner_text("#schedule"), "tool: the break row says tools")
    # Every word a student reads, outside the gallery, the game-version pointers and the URLs, says tool.
    text = page.inner_text("main")
    for part in page.locator("#after .gallery").all_inner_texts():
        text = text.replace(part, "")
    for allowed in ["game version of this page", "Game version of this page", "Would your team rather design a game?", "Can we design a game instead?",
                    "It has game versions of the three templates", "then show games built with the same framework"]:
        text = text.replace(allowed, "")
    check("game" not in text.lower(), "tool: no other text on the tool version says game " + str([l for l in text.splitlines() if "game" in l.lower()][:3]))
    # Downloads and pictures resolve one folder up, and each is published.
    hrefs = page.eval_on_selector_all(".downloads a", "els => els.map(e => e.href)")
    check(len(hrefs) == 4 and all(page.request.get(h).status == 200 for h in hrefs), "tool: its four downloads are published")
    srcs = page.eval_on_selector_all(".game img", "els => els.map(e => e.src)")
    check(len(srcs) == 13 and all(page.request.get(u).status == 200 for u in srcs), "tool: every gallery picture is published")
    check(page.locator(".game b").first.inner_text() == "PDF Redactor" and page.locator(".game").count() == 13, "tool: the gallery shows the three tools first, then the ten games")
    links = page.locator("#editions a")
    check(links.count() == 3, "tool: the intro links two printable files and the game version")
    for href in [links.nth(i).get_attribute("href") for i in range(3)]:
        check(page.request.get(urljoin(base, href)).status == 200, f"tool: {href} is published")
    page.click("#editions a[href='../']")
    page.wait_for_selector("body[data-ready='true']")
    check(page.inner_text("#title") == "Collaborative Game Design Project", "tool: the game version link opens the game version")
    check(page.evaluate("document.documentElement.scrollWidth <= window.innerWidth"), "tool: the page does not scroll sideways")
    ready(page, base + "index.html?edition=print")
    check("sgilson7.github.io/cis110-game-lab/tool/" in page.inner_text("#edition-note"), "tool: the print edition says where the web page is")
    page.goto(base + "cis110-tool-project.html")
    check(page.locator("script").count() == 0, "tool: the single-file HTML has no script")
    check(page.inner_text("h1") == "Collaborative Tool Design Project", "tool: the single-file HTML shows the title")
    check(page.locator(".game img[src^='data:image/']").count() == 13, "tool: its thirteen gallery pictures are inside the file")
    check(page.locator(".downloads a[href^='data:']").count() == 4, "tool: its four downloads are inside the file")
