"""The project's own browser interactions, run by check.py in every engine.

check.py has already loaded the page, waited for it to report ready, and let
it idle. These checks drive the page the way a student would and assert what
the page shows. Expected values are worked out by hand from the syllabus
(pages 9 and 23), never copied from the page's own output (C4). The page is
reloaded with ?on= to see it as of a fixed date, so the checks do not depend
on the day they run.
"""


def ready(page, url):
    page.goto(url)
    page.wait_for_selector("body[data-ready='true']")


def run(page, check, engine):
    base = page.url.split("?")[0].split("#")[0]
    check(page.inner_text("#title") == "Collaborative Game Design Project", "the title is drawn")
    check(page.inner_text("#message") == "", "no error message is shown")

    # 20 November 2026, a Friday: checkpoints 1-3 have passed; C4 (Mon 23 Nov) is next, 3 days away.
    ready(page, base + "?on=2026-11-20T10:00")
    check(page.inner_text("#next h2") == "Monday, November 23", "the next checkpoint on 20 Nov is Monday, November 23")
    check("Due in 3 days" in page.inner_text("#next"), "it is due in 3 days")
    check(page.locator(".row[data-status='done']").count() == 3, "three checkpoints show as passed")
    check(page.locator(".row[data-status='next'] time").inner_text() == "Mon, Nov 23", "the schedule marks Nov 23 as next")
    check("Graded · 100 points" in page.inner_text(".row[data-graded='true']"), "the Dec 10 row says it is graded, 100 points")

    # The day of a deadline, before and after 5:00 PM Eastern.
    ready(page, base + "?on=2026-11-16T16:59")
    check("Due today at 5:00 PM ET" in page.inner_text("#next"), "at 4:59 PM on Nov 16 the Word draft is due today")
    ready(page, base + "?on=2026-11-16T17:00")
    check(page.inner_text("#next h2") == "Monday, November 23", "at 5:00 PM on Nov 16 the card moves to Nov 23")

    # After the final deadline.
    ready(page, base + "?on=2026-12-11T09:00")
    check(page.inner_text("#next h2") == "The project is complete", "after Dec 10 the card says the project is complete")

    # The stat strip and the rubric: 3-5 students, 3 files, 7 checkpoints, 100 points; 20+25+20+20+15.
    stats = page.locator(".stat b").all_inner_texts()
    check(stats == ["3–5", "3", "7", "100"], "the stat strip reads 3–5, 3, 7, 100")
    check(page.locator(".rb b").all_inner_texts() == ["20", "25", "20", "20", "15", "100"], "the rubric reads 20, 25, 20, 20, 15, total 100")

    # File cards: Word draft Nov 16 (25 points), Excel Nov 23 (20), PowerPoint Nov 30 (20).
    feet = page.locator(".file-foot").all_inner_texts()
    check("Mon, Nov 16" in feet[0] and "25" in feet[0], "the Word card says Mon, Nov 16 and 25 points")
    check("Mon, Nov 30" in feet[2] and "20" in feet[2], "the PowerPoint card says Mon, Nov 30 and 20 points")

    # Setup tabs go through the core: Microsoft 365 first, then the Google Drive backup.
    check(page.get_attribute("#tab-m365", "aria-selected") == "true", "the Microsoft 365 tab is open first")
    check(page.locator("ol.steps li").count() == 8, "the Microsoft 365 tab has eight steps")
    page.click("#tab-google")
    page.wait_for_selector("#tab-google[aria-selected='true']")
    check(page.locator("ol.steps li").count() == 4, "the Google Drive tab has four steps")
    check("Use Google Drive only if" in page.inner_text(".tab-lede"), "the Google Drive tab opens by saying it is the backup")

    # The templates are on this site.
    check(page.locator("a[href*='GameDesign.docx']").count() == 1, "the Word template link is present")
    check(page.evaluate("document.documentElement.scrollWidth <= window.innerWidth"), "the page does not scroll sideways")
