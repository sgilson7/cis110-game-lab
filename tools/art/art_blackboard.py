"""Pictures for the course's Blackboard pages (T15): the learning module
image (square) and the course banner (5:1).

One oversized laptop stands in the street. Its screen is split: a small game
on the left, a small tool on the right, because the project has a game
version and a tool version. Three pages lean against it, one for each Office
file, marked by a coloured tab. A team of four stands nearby. Behind them is
the default style pack's town under a banded sky, with the upper left kept
calm.

Colours are role names from the style pack; there is no text anywhere.
"""
import style
from artlib import Fig

MOTIFS = style.load().motifs()
WALLS = ["wallA", "wallB", "wallC", "wallD", "wallE", "wallF"]
SKIN = ["wood", "wallD", "wood!60!windark", "cream!70!wood"]
SHIRTS = ["teal", "greenD", "wallE", "gold"]


# ----- the town -------------------------------------------------------------

def building(fig, rng, x, ground, w, h, wall, clutter=True):
    fig.rect(x, ground, x + w, ground + h, wall)
    cols = max(1, int(w / 0.55))
    rows = max(1, int((h - 0.5) / 0.7))
    gap = w / cols
    for r in range(rows):
        for c in range(cols):
            wx = x + c * gap + gap * 0.25
            wy = ground + 0.45 + r * 0.7
            if wy + 0.4 > ground + h - 0.15:
                continue
            fig.rect(wx, wy, wx + gap * 0.5, wy + 0.38, "window" if rng.random() < 0.55 else "windark")
    roof = ground + h
    pick = rng.random() if clutter else 2.0
    if pick > 1.0:
        pass
    elif pick < 0.35 and w > 0.9:
        tx = x + rng.uniform(0.15, w - 0.75)
        fig.stroke([(tx + 0.1, roof), (tx + 0.1, roof + 0.25)], width=0.8)
        fig.stroke([(tx + 0.5, roof), (tx + 0.5, roof + 0.25)], width=0.8)
        fig.rect(tx, roof + 0.25, tx + 0.6, roof + 0.75, "metal")
        fig.poly([(tx - 0.05, roof + 0.75), (tx + 0.65, roof + 0.75), (tx + 0.3, roof + 0.98)], "metaldark")
    elif pick < 0.65:
        ax = x + rng.uniform(0.1, max(0.15, w - 0.6))
        fig.rect(ax, roof, ax + 0.5, roof + 0.32, "metal")
        fig.circle(ax + 0.25, roof + 0.16, 0.1, "metaldark")
    else:
        ax = x + rng.uniform(0.2, max(0.25, w - 0.2))
        fig.stroke([(ax, roof), (ax, roof + 0.7)], width=0.8)
        fig.stroke([(ax - 0.18, roof + 0.55), (ax + 0.18, roof + 0.55)], width=0.6)
    if rng.random() < 0.4 and w > 0.8:
        fig.poly([(x + 0.1, ground + 0.75), (x + w - 0.1, ground + 0.75), (x + w + 0.05, ground + 0.5),
                  (x - 0.05, ground + 0.5)], rng.choice(["greenA", "greenB", "teal"]))
    return roof


def town(fig, rng, x0, x1, ground, tall, short, short_until, avoid=(0.0, 0.0)):
    """Buildings from x0 to x1. Left of short_until they stay under `short`
    so the calm corner meets the sky; elsewhere they reach up to `tall`. No
    wire is strung across `avoid` (x from, x to), where the laptop stands."""
    x, tops, last = x0, [], None
    while x < x1:
        w = rng.uniform(1.0, 1.8)
        hi = short if x + w <= short_until else tall
        h = rng.uniform(hi * 0.65, hi)
        wall = rng.choice([c for c in WALLS if c != last])
        last = wall
        # Roof clutter only right of the calm corner, so the corner stays sky.
        tops.append((x + w, building(fig, rng, x, ground, w, h, wall, clutter=x + w > short_until)))
        x += w
    for (xa, ya), (xb, yb) in zip(tops, tops[1:]):
        hidden = xb + 0.1 > avoid[0] and xa - 0.1 < avoid[1]
        if xa > short_until and abs(ya - yb) < 0.8 and not hidden:
            mid = (xa + xb) / 2
            fig.stroke([(xa - 0.1, ya - 0.3), (mid, min(ya, yb) - 0.7), (xb + 0.1, yb - 0.3)],
                       width=0.5, smooth=True)


def street(fig, W, ground, rng, n_stones):
    fig.rect(-0.2, -0.2, W + 0.2, ground, "stone")
    fig.rect(-0.2, ground - 0.12, W + 0.2, ground, "stone!75!metal")
    for _ in range(n_stones):
        x, y = rng.uniform(0.2, W - 0.2), rng.uniform(0.15, ground - 0.35)
        fig.ellipse(x, y, rng.uniform(0.05, 0.12), rng.uniform(0.03, 0.07), "stone!70!metal")


# ----- the subject ------------------------------------------------------------

def laptop(fig, x, ground, s):
    """About 6.8 units wide and 4.3 tall before scaling; (x, ground) is the
    middle of its base."""
    with fig.frame(x, ground, s):
        fig.poly([(-3.4, 0), (3.4, 0), (3.0, 0.45), (-3.0, 0.45)], "metal")
        fig.rect(-0.6, 0.1, 0.6, 0.3, "metaldark")
        fig.rect(-2.9, 0.5, 2.9, 4.3, "metaldark", r=0.15)
        # The game, left half: sky, ground, two platforms, a coin, a flag and a player.
        fig.rect(-2.65, 0.75, 0, 4.05, "skyhi")
        fig.rect(-2.65, 0.75, 0, 1.3, "greenB")
        fig.rect(-2.2, 2.1, -1.2, 2.4, "greenA")
        fig.rect(-1.0, 2.9, -0.25, 3.2, "greenA")
        for cx in (-1.9, -1.6):
            fig.circle(cx, 2.7, 0.12, "gold")
        fig.circle(-0.62, 3.5, 0.12, "gold")
        fig.stroke([(-0.45, 1.3), (-0.45, 2.3)], width=1.0)
        fig.poly([(-0.45, 2.3), (-0.05 - 0.1, 2.12), (-0.45, 1.95)], "wallE")
        fig.rect(-1.75, 1.3, -1.45, 1.75, "wallC")
        fig.circle(-1.6, 1.92, 0.17, "cream")
        # The tool, right half: a page with lines, two blacked-out lines, a button and a tick.
        fig.rect(0, 0.75, 2.65, 4.05, "paper")
        fig.rect(0.3, 1.0, 1.9, 3.8, "cream")
        fig.rect(0.3, 3.45, 1.9, 3.8, "teal")
        for i, y in enumerate([3.15, 2.85, 2.55, 2.25, 1.95, 1.65, 1.35]):
            if i in (2, 5):
                fig.rect(0.5, y - 0.09, 1.55, y + 0.09, "windark")
            else:
                fig.stroke([(0.5, y), (1.7 if i % 2 else 1.45, y)], width=1.4, role="metal")
        fig.rect(2.05, 1.0, 2.5, 1.45, "greenD", r=0.08)
        fig.stroke([(2.13, 1.24), (2.24, 1.11), (2.42, 1.36)], width=1.6, role="cream")
        fig.stroke([(0, 0.75), (0, 4.05)])


def page(fig, x, y, s, rot, tab):
    """One Office file: a page with a folded corner and a coloured tab."""
    with fig.frame(x, y, s, rot):
        fig.poly([(0, 0), (1.0, 0), (1.0, 1.0), (0.72, 1.3), (0, 1.3)], "cream")
        fig.poly([(0.72, 1.3), (0.72, 1.0), (1.0, 1.0)], "paper")
        fig.rect(-0.12, 0.12, 0.42, 0.55, tab)
        for yy in (0.75, 0.95):
            fig.stroke([(0.15, yy), (0.6, yy)], width=1.0, role="metal")
        fig.stroke([(0.55, 0.3), (0.85, 0.3)], width=1.0, role="metal")


def person(fig, x, ground, s, shirt, skin, wave=False):
    """A small figure about 2.6 units tall before scaling, feet at (x, ground)."""
    with fig.frame(x, ground, s):
        fig.stroke([(-0.2, 0), (-0.15, 0.9)], width=1.6)
        fig.stroke([(0.2, 0), (0.15, 0.9)], width=1.6)
        fig.rect(-0.38, 0.85, 0.38, 1.95, shirt, r=0.15)
        fig.stroke([(-0.35, 1.8), (-0.6, 1.15)], width=1.4)
        fig.stroke([(0.35, 1.8), (0.75, 2.5)] if wave else [(0.35, 1.8), (0.6, 1.15)], width=1.4)
        fig.circle(0, 2.3, 0.33, skin)


def crate(fig, x, ground, s):
    with fig.frame(x, ground, s):
        fig.rect(0, 0, 1, 0.8, "wood")
        fig.stroke([(0.1, 0.1), (0.9, 0.7)], width=0.7)
        fig.stroke([(0.1, 0.7), (0.9, 0.1)], width=0.7)


# ----- the pictures -------------------------------------------------------------

def module(rng):
    """Blackboard learning module image: square (500 by 500 shown; wider
    images are cropped to a square)."""
    fig = Fig("square")
    W, H = fig.size
    ground = 2.4
    MOTIFS.sky_bands(fig)
    MOTIFS.clouds(fig, rng, n=3, band=(0.76, 0.9), scale=(0.45, 0.7), x_from=0.05)
    town(fig, rng, -0.3, W + 0.3, ground, tall=6.0, short=1.9, short_until=W / 3, avoid=(3.4, 9.3))
    street(fig, W, ground, rng, 14)
    laptop(fig, 6.35, ground - 0.15, 0.85)
    page(fig, 2.35, ground - 0.25, 0.75, 8, "teal")
    page(fig, 3.0, ground - 0.3, 0.75, -4, "greenD")
    page(fig, 8.45, ground - 0.95, 0.7, -10, "wallE")
    crate(fig, 0.3, ground - 0.55, 0.55)
    for i, x in enumerate([0.95, 1.6, 4.0, 8.95]):
        person(fig, x, 0.5 + (i % 2) * 0.25, 0.42, SHIRTS[i], SKIN[i], wave=(i == 2))
    MOTIFS.calm_left_third(fig)
    return fig


def banner(rng):
    """Blackboard course banner: 5:1 (at least 1200 by 240)."""
    fig = Fig("banner")
    W, H = fig.size
    ground = 0.95
    MOTIFS.sky_bands(fig)
    MOTIFS.clouds(fig, rng, n=6, band=(0.7, 0.86), scale=(0.35, 0.55), x_from=0.02)
    town(fig, rng, -0.3, W + 0.3, ground, tall=2.6, short=1.0, short_until=W / 3, avoid=(12.7, 17.7))
    street(fig, W, ground, rng, 26)
    laptop(fig, 15.2, ground - 0.1, 0.72)
    page(fig, 11.35, ground - 0.2, 0.55, 8, "teal")
    page(fig, 11.95, ground - 0.25, 0.55, -4, "greenD")
    page(fig, 18.15, ground - 0.25, 0.55, -10, "wallE")
    crate(fig, 20.9, ground - 0.45, 0.45)
    crate(fig, 5.2, ground - 0.5, 0.4)
    for i, x in enumerate([9.4, 10.3, 19.4, 20.2]):
        person(fig, x, 0.22 + (i % 2) * 0.12, 0.3, SHIRTS[i], SKIN[i], wave=(i == 2))
    MOTIFS.calm_left_third(fig)
    return fig


PICTURES = {
    "blackboard/module": module,
    "blackboard/banner": banner,
}
