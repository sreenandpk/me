"""
Background removal script v2 — fixes enclosed gray pockets.

Strategy:
  Pass 1: Flood-fill from all four edges  (removes exterior background)
  Pass 2: Global neutral-gray color match  (removes ANY enclosed gray that
           matches the background hue — safe because the anime character has
           NO neutral gray in its design: clothing is black, skin is warm,
           hair is near-black.)
  Pass 3: Drop-shadow removal (bottom strip, close-to-bg)
  Pass 4: Watermark removal   (bottom-right sparkle)
  Pass 5: Edge anti-halo      (erode 1-2px border near bg)

Usage: python scripts/remove_bg.py
"""

import os
import sys
from pathlib import Path
from PIL import Image
import numpy as np
from collections import deque

# ── Paths ──────────────────────────────────────────────────────────────────────
SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT   = SCRIPT_DIR.parent
INPUT_DIR   = REPO_ROOT / "references"
OUTPUT_DIR  = REPO_ROOT / "assets" / "character" / "skills"

OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

# ── Tuning ─────────────────────────────────────────────────────────────────────
FLOOD_TOL      = 28   # flood-fill colour tolerance (per channel)
GLOBAL_TOL     = 22   # global-match colour tolerance (tighter = safer)
MAX_SAT        = 0.18 # max HSV saturation for a pixel to be considered "gray"
SHADOW_STRIP   = 0.65 # y-fraction below which shadow removal applies
SHADOW_TOL     = 55   # additional tolerance for shadow pixels
WM_THRESH      = 215  # min brightness for watermark pixel
EDGE_ERODE     = 2    # px of anti-halo erosion passes


def sample_bg_color(rgb: np.ndarray) -> np.ndarray:
    """Average the four 5×5 corner patches to get the background colour."""
    h, w = rgb.shape[:2]
    s = 5
    corners = [
        rgb[:s,   :s  ],
        rgb[:s,   w-s:],
        rgb[h-s:, :s  ],
        rgb[h-s:, w-s:],
    ]
    return np.mean([c.mean(axis=(0,1)) for c in corners], axis=0)


def flood_fill_mask(rgb: np.ndarray, bg: np.ndarray, tol: int) -> np.ndarray:
    """BFS flood-fill from all four image edges. Returns bool mask (True=bg)."""
    h, w = rgb.shape[:2]
    visited = np.zeros((h, w), dtype=bool)
    q = deque()

    seeds = (
        [(0, x) for x in range(w)]
      + [(h-1, x) for x in range(w)]
      + [(y, 0) for y in range(h)]
      + [(y, w-1) for y in range(h)]
    )
    for sy, sx in seeds:
        if not visited[sy, sx]:
            diff = np.abs(rgb[sy, sx].astype(int) - bg)
            if np.all(diff <= tol):
                visited[sy, sx] = True
                q.append((sy, sx))

    while q:
        y, x = q.popleft()
        for dy, dx in ((-1,0),(1,0),(0,-1),(0,1)):
            ny, nx = y+dy, x+dx
            if 0 <= ny < h and 0 <= nx < w and not visited[ny, nx]:
                diff = np.abs(rgb[ny, nx].astype(int) - bg)
                if np.all(diff <= tol):
                    visited[ny, nx] = True
                    q.append((ny, nx))
    return visited


def global_gray_mask(rgb: np.ndarray, bg: np.ndarray, tol: int, max_sat: float) -> np.ndarray:
    """
    Mark every pixel that is BOTH close to bg colour AND is a neutral gray
    (low HSV saturation).  This catches enclosed pockets the flood-fill missed.

    The character has no neutral gray:
      - Suit/clothing  : near-black
      - Skin           : warm/yellow-ish
      - Hair           : near-black
      - Belt buckle    : silver — but that is very bright; excluded by tol
    """
    r = rgb[:,:,0].astype(np.float32)
    g = rgb[:,:,1].astype(np.float32)
    b = rgb[:,:,2].astype(np.float32)

    # Distance to bg colour (L-inf)
    diff = np.abs(rgb.astype(np.float32) - bg)
    close_to_bg = np.all(diff <= tol, axis=-1)

    # HSV saturation = (max-min)/max
    mx = np.maximum(np.maximum(r, g), b)
    mn = np.minimum(np.minimum(r, g), b)
    sat = np.where(mx > 1e-6, (mx - mn) / mx, 0.0)
    is_neutral = sat < max_sat

    return close_to_bg & is_neutral


def remove_background(src: Path, dst: Path) -> None:
    img  = Image.open(src).convert("RGBA")
    rgba = np.array(img, dtype=np.uint8)
    rgb  = rgba[:, :, :3]
    h, w = rgb.shape[:2]

    bg = sample_bg_color(rgb)

    # ── Pass 1: edge flood-fill ──────────────────────────────────────────────
    mask = flood_fill_mask(rgb, bg, FLOOD_TOL)

    # ── Pass 2: global neutral-gray match (fixes enclosed pockets) ───────────
    mask |= global_gray_mask(rgb, bg, GLOBAL_TOL, MAX_SAT)

    # ── Pass 3: drop-shadow strip ────────────────────────────────────────────
    row0 = int(h * SHADOW_STRIP)
    shadow_diff = np.abs(rgb[row0:].astype(np.float32) - bg)
    shadow_close = np.all(shadow_diff <= SHADOW_TOL, axis=-1)
    shadow_mask = np.zeros((h, w), dtype=bool)
    shadow_mask[row0:] = shadow_close
    mask |= shadow_mask

    # ── Pass 4: watermark (sparkle, bottom-right) ────────────────────────────
    wr, wc = int(h * 0.72), int(w * 0.72)
    wm_region = rgb[wr:, wc:]
    wm_bright = np.all(wm_region >= WM_THRESH, axis=-1)
    wm_diff   = np.abs(wm_region.astype(np.float32) - bg)
    wm_close  = np.all(wm_diff <= SHADOW_TOL, axis=-1)
    wm_mask   = np.zeros((h, w), dtype=bool)
    wm_mask[wr:, wc:] = wm_bright | wm_close
    mask |= wm_mask

    # ── Pass 5: edge anti-halo ───────────────────────────────────────────────
    for _ in range(EDGE_ERODE):
        pad = np.pad(mask, 1, constant_values=False)
        neighbour = (
            pad[:-2, 1:-1].astype(np.int16)
          + pad[2:,  1:-1].astype(np.int16)
          + pad[1:-1,:-2].astype(np.int16)
          + pad[1:-1, 2:].astype(np.int16)
        )
        fg_near_bg = (~mask) & (neighbour >= 2)
        diff = np.abs(rgb.astype(np.float32) - bg)
        halo = np.all(diff <= FLOOD_TOL + 12, axis=-1)
        mask |= fg_near_bg & halo

    # ── Apply transparency ───────────────────────────────────────────────────
    rgba[:,:,3] = np.where(mask, 0, 255).astype(np.uint8)
    Image.fromarray(rgba).save(dst, "PNG", optimize=False)


def main():
    frames = sorted(INPUT_DIR.glob("ezgif-frame-*.jpg"))
    if not frames:
        print(f"[ERROR] No frames found in {INPUT_DIR}")
        sys.exit(1)

    total = len(frames)
    print(f"remove_bg v2 — processing {total} frames")
    print(f"  Input : {INPUT_DIR}")
    print(f"  Output: {OUTPUT_DIR}")
    print()

    for i, src in enumerate(frames, 1):
        dst = OUTPUT_DIR / f"frame-{i:03d}.png"
        print(f"  [{i:02d}/{total}] {src.name} -> {dst.name}", end="", flush=True)
        remove_background(src, dst)
        print(" OK")

    print()
    print(f"Done. {total} transparent PNGs saved.")


if __name__ == "__main__":
    main()
