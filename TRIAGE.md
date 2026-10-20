# Triage

Stage 7. One row per request after the person has used the build: a bug, a change, or a new idea, in the person's own words. Rows are numbered T1, T2 and so on, and each commit that answers one names it ("Fix the timer reset (T4)"). `C4 Verify against hand-computed cases`.

Compare at least one result with a case worked out by hand. **Severity** is *blocks use*, *wrong*, or *wanted*. **Cost** is the agent's estimate. **Disposition** is *done*, *open*, *declined* (with the reason), or *try* (built on a branch and published as a TRY commit that says how to undo it).

| # | Request, in the person's words | Expected | What happened | Severity | Cost | Disposition |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | "go with all defaults" | The recommendation in each of GL-Q1 to GL-Q9 is built. | Answered 2026-10-19. | wanted | small | done |
| T2 | "the merry querry big headers are nice but a bit much and stylized for a lesson at a community college, try and make it a little more toned down and html'y … i kind of want it looking like the ape race design log or the pdf signature adder log, like a mixture of those with the merry query look" | The live page uses the journals' plain layout (system fonts, dated rail, bordered panels) with MerryQuery's eyebrows, stat strip and file cards, and no large display serif. | Built: system fonts, journal rail and panels, MerryQuery eyebrows, stat strip and file cards. | medium | done |
| T3 | "add a tab on those updates" | A journal page on the Builder additions (GL-Q6). | The journal's Builder updates page. | small | done |
| T4 | "then deploy to live" | The homepage is public on GitHub Pages and passes the browser check at the live address. | Live at sgilson7.github.io/cis110-game-lab; live check passed twice. | medium | done |
| T5 | "then rebuild log with mockup vs real deploy comparison, and analyze where the mockup and the live version diverge, with an emphasis on making the human facing text read like a human, in the style of google open source documentation and professional google blog tone" | A journal entry pairing each mockup section with the live one, a divergence table, and a copy review against Google's developer documentation style. | The journal's comparison entry; copy pass deployed as 1219e20 (R2.13, R2.14). | medium | done |
| T6 | "i think bullet hell is a bit much for a public facing piece of content, rename it a 3rd person action survival game, ... for long time coming. also add vagrancy and slushline as example games" | Long Time Comin' is described as a third-person action survival game; Vagrancy and Slushline join the gallery with gameplay screenshots. | Built; gallery now has six games. | wanted | small | done |
