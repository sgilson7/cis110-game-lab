# Brief

Stage 1. The person's own words come first; the rest can be written by the agent from them and read by the person. `C1 Frame the problem and set the rules`, `B1 Establish context`.

## The description, word for word

The person's request of 19 October 2026, unchanged except that a peer's first name is replaced with `[a peer]` (R1.2). Later requests in plain prose are numbered in `TRIAGE.md`.

> get yourself familiarized with the builder repository and clone it from https://github.com/sgilson7/builder-setting into this repo then remove the .git. You will be using builder setting to help constrain and direct your actions as you help act as as a an assignment creator and co-designer / developer, with the goal of deploying it to a github pages for students to access as part of their coursework. Please also see https://sgilson7.github.io/projects/ for examples of other browser based tools / games built  in builder like the PDF redactor, youtube to mp3, long time coming, etc. First thing is I want you to interpret builder, and write an analysis of how you will use it to help you write this assignment into the first tab of the design journal, the design journal process is described in builder. the assignment has been described in the syllabus pdf and the two text documents in this directory. I'd like to both create an assignment homepage that explains in full detail what they will have to do, and then also author some updates to builder to work seamlessly with the file types and structure of the student submission, so that I can just unzip the students submission, start claude code in it, and give it a prompt similar to this one to build the journal and the deployed version of the game to my public github and pages. first thing, make a tab in the design journal detailing how you will use builder to constrain and influence your actions as you work on this assignment. this website made by my peer [a peer] for his project merry query is amazing and is the gold standard for what I am looking for for front end design for describing the assignment, but a little more student optimized to be clearer what their deliverables are, what the due dates are, etc. build a 2nd tab in the design journal with a literature review on how to facilitate remote co-design in students who are not subject matter exprerts, in order to influence the writing and flow of the assignment students will have to do. then build a third tab in the design journal with mockups of the assignment page students will go to in order to full explain the work they'll need to do, the deliverables for the project, flow diagrams for the assignment, phases of the assignment, deliverable dates and the such given the syllabus, etc. keep in mind students will be using microsoft office and NOT google drive, so any instructions about how to do the co-design part of their assignment should be written using microsoft office and 365, with google drive as a fallback option if they cant get it to work.

Source documents the description points to, kept beside this project in `../`:

- `CIS110_LN182_Fall2026_Second8Weeks_Final_SimpleSyllabus.pdf`: pages 9 and 23 describe the Collaborative Game Design Project, its seven deadlines and its 100-point rubric.
- `ApprovedProposalText`: the approved proposal (three Office files per group, the instructor builds and posts the games).
- `StudentFacingTextInAnnouncement`: the announcement students have already read.

## The tool

Two things, built in this order.

1. **The assignment homepage.** A static page students in CIS 110 LN182 (fully online, second eight weeks of Fall 2026) open from Blackboard. It says what each team hands in, when, how it is graded, how to set up and work in a shared Microsoft 365 folder (Google Drive as a fallback), and what happens to the game afterwards. It must read well on a phone and a laptop. The front-end reference is the MerryQuery site (exploremq.benyamintabarsi.com), made clearer for students about deliverables and dates.
2. **A Builder path for student submissions.** Additions to Builder so the instructor can unzip a team's downloaded folder (one `.docx`, one `.pptx`, one `.xlsx`), start a coding agent in it, and with a short prompt get a Builder project, its design journal, and the game published to the instructor's public GitHub Pages.

## Done means

To be settled in `PLAN.md` after the opening entry is answered. Draft conditions:

- Every date and point value on the homepage matches pages 9 and 23 of the syllabus, and a test pins each one.
- The homepage's Microsoft 365 steps are written for the CCCC student Office 365 account; a Google Drive section is marked as the fallback.
- The homepage passes `./scripts/check.sh` in three browser engines and at phone width.
- A sample submission folder, built from the student templates, goes through the submission path to a running game without hand edits.

## Decisions that stay with the person

- The assignment's content, dates, points and policies (the syllabus is the source).
- Whether students get a prototype to review before the final deadline, and when.
- Whether student names appear on any public page, and the permission form for it.
- Where the homepage and the games are published.
- Any study of the students' work (needs IRB approval first; separate from grading).

## Accessibility

Information is carried by text and shape as well as colour (deadline status uses a word and an icon). Keyboard navigation through every section. Readable at 400 px wide.

## Style

The look follows the MerryQuery site's structure (serif display headings, a stat strip, eyebrow labels, framed product pictures, a dark feature band) with its own palette. Details in the mockups tab of the journal.

## Words

Terms from the assignment: *team*, *shared folder*, *game design document*, *checkpoint*, *prototype*, *change request*, *contribution*. They go into `GLOSSARY.md` before any string uses them.

## Autonomy

The plan is not approved in advance. The agent writes the journal's opening entry, the literature review and the mockups, then stops for the person's answers. The agent may not loosen a Builder requirement or a boundary test, and may not add a server, an account, analytics or any off-origin request. The agent may publish the design journal to a private claude.ai page. Publishing anything public waits for an explicit grant naming the repository.

## Journal

`default`

## Allowed core dependencies

`serde`, `serde_json` (reference default). Add others only here, with the reason.
