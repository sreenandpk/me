"""
Background removal script v4 — clean, no edge feathering.

Changes from v3:
  - REMOVED feather_alpha (was eating face/skin on bent-over poses)
  - ADDED targeted boot-shadow removal:
      * Only operates in the bottom SHADOW_STRIP fraction of the image
      * Removes bright (>SHADOW_BRIGHT) near-background pixels
      * Character's black boots (brightness ~30) are completely safe
  - All other passes unchanged (flood fill, enclosed pockets, watermark)

Usage: python scripts/remove_bg.py
"""

import sys
from pathlib import Path
from PIL import Image
import numpy as np
from collections import deque
import warnings
warnings.filterwarnings("ignore", category=RuntimeWarning)

# ── Paths ──────────────────────────────────────────────────────────────────────
SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT   = SCRIPT_DIR.parent
INPUT_DIR   = REPO_ROOT / "references"
OUTPUT_DIR  = REPO_ROOT / "assets" / "character" / "skills"
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

# ── Tuning ─────────────────────────────────────────────────────────────────────
FLOOD_TOL        = 32    # exterior flood-fill colour tolerance (per channel)
INNER_TOL        = 12    # enclosed-pocket match tolerance
INNER_SAT_MAX    = 0.04  # max saturation for enclosed pockets (pure neutral gray)
INNER_BRIGHT_MIN = 152   # min brightness for enclosed pockets (bg ~179, char <120)

# Boot shadow (the gray ellipse on the ground below the feet)
SHADOW_STRIP     = 0.78  # y-fraction: shadow-removal only operates below this line
SHADOW_TOL       = 70    # colour tolerance vs background for shadow pixels
SHADOW_BRIGHT_MIN= 100   # shadow must be brighter than this (boots are <50, shadow >120)

# Watermark (ezgif sparkle, bottom-right corner)
WM_ROW_FRAC   = 0.74
WM_COL_FRAC   = 0.78
WM_BRIGHT_MIN = 200


def sample_bg(rgb: np.ndarray) -> np.ndarray:
    h, w = rgb.shape[:2]
    s = 8
    patches = [rgb[:s,:s], rgb[:s,w-s:], rgb[h-s:,:s], rgb[h-s:,w-s:]]
    return np.mean([p.mean(axis=(0,1)) for p in patches], axis=0)


def flood_fill_exterior(rgb: np.ndarray, bg: np.ndarray, tol: int) -> np.ndarray:
    """BFS from all four edges. Returns bool mask (True = exterior background)."""
    h, w = rgb.shape[:2]
    mask = np.zeros((h, w), dtype=bool)
    q    = deque()

    seeds = (
        [(0,   x) for x in range(w)] +
        [(h-1, x) for x in range(w)] +
        [(y,   0) for y in range(h)] +
        [(y, w-1) for y in range(h)]
    )
    for sy, sx in seeds:
        if not mask[sy, sx]:
            if np.all(np.abs(rgb[sy, sx].astype(np.int32) - bg) <= tol):
                mask[sy, sx] = True
                q.append((sy, sx))

    while q:
        y, x = q.popleft()
        for dy, dx in ((-1,0),(1,0),(0,-1),(0,1)):
            ny, nx = y+dy, x+dx
            if 0 <= ny < h and 0 <= nx < w and not mask[ny, nx]:
                if np.all(np.abs(rgb[ny, nx].astype(np.int32) - bg) <= tol):
                    mask[ny, nx] = True
                    q.append((ny, nx))
    return mask


def find_enclosed_pockets(rgb: np.ndarray, exterior: np.ndarray,
                           bg: np.ndarray) -> np.ndarray:
    """
    Background pixels trapped inside the silhouette (e.g., jacket/arm gaps).
    Very tight: must be near-identical to bg AND pure neutral gray AND bright.
    Character pixels are never neutral gray (black suit, warm skin, dark hair).
    """
    rgb_f = rgb.astype(np.float32)
    bg_f  = bg.astype(np.float32)

    diff    = np.abs(rgb_f - bg_f)
    close   = np.all(diff <= INNER_TOL, axis=-1)

    r, g, b = rgb_f[:,:,0], rgb_f[:,:,1], rgb_f[:,:,2]
    mx  = np.maximum(np.maximum(r, g), b)
    mn  = np.minimum(np.minimum(r, g), b)
    sat = np.where(mx > 1.0, (mx - mn) / mx, 0.0)

    neutral = sat < INNER_SAT_MAX
    bright  = (r + g + b) / 3.0 > INNER_BRIGHT_MIN

    return close & neutral & bright & (~exterior)


def remove_boot_shadow(rgb: np.ndarray, bg: np.ndarray) -> np.ndarray:
    """
    Remove the gray ellipse shadow below the character's feet.
    Only operates in the bottom SHADOW_STRIP fraction of the image.
    Uses two guards so the boots themselves are never removed:
      1. Pixel must be close to background colour (±SHADOW_TOL)
      2. Pixel must be bright (>SHADOW_BRIGHT_MIN) — boots are near-black (~30)
    """
    h, w   = rgb.shape[:2]
    mask   = np.zeros((h, w), dtype=bool)
    row0   = int(h * SHADOW_STRIP)

    strip  = rgb[row0:].astype(np.float32)
    bg_f   = bg.astype(np.float32)

    diff   = np.abs(strip - bg_f)
    close  = np.all(diff <= SHADOW_TOL, axis=-1)

    brightness = strip.mean(axis=-1)
    bright = brightness > SHADOW_BRIGHT_MIN

    mask[row0:] = close & bright
    return mask


def apply_watermark_mask(rgb: np.ndarray) -> np.ndarray:
    """Remove the ezgif sparkle watermark (bottom-right corner)."""
    from scipy.ndimage import binary_dilation
    h, w  = rgb.shape[:2]
    wr    = int(h * WM_ROW_FRAC)
    wc    = int(w * WM_COL_FRAC)
    wm    = np.zeros((h, w), dtype=bool)
    region = rgb[wr:, wc:]

    r, g, b = region[:,:,0].astype(float), region[:,:,1].astype(float), region[:,:,2].astype(float)
    bright  = np.all(region >= WM_BRIGHT_MIN, axis=-1)
    mx  = np.maximum(np.maximum(r, g), b)
    mn  = np.minimum(np.minimum(r, g), b)
    sat = np.where(mx > 1, (mx - mn) / mx, 0.0)
    neutral = sat < 0.15

    wm[wr:, wc:] = bright & neutral
    wm = binary_dilation(wm, iterations=4)
    return wm


def remove_background(src: Path, dst: Path) -> None:
    img  = Image.open(src).convert("RGBA")
    rgba = np.array(img, dtype=np.uint8)
    rgb  = rgba[:, :, :3]

    bg = sample_bg(rgb)

    # Pass 1 — exterior flood fill
    ext = flood_fill_exterior(rgb, bg, FLOOD_TOL)

    # Pass 2 — enclosed background pockets (jacket gaps, etc.)
    pockets = find_enclosed_pockets(rgb, ext, bg)

    # Pass 3 — boot shadow (gray ellipse on the ground)
    shadow = remove_boot_shadow(rgb, bg)

    # Pass 4 — sparkle watermark
    wm = apply_watermark_mask(rgb)

    # Apply
    bg_mask = ext | pockets | shadow | wm
    rgba[:, :, 3] = np.where(bg_mask, 0, 255).astype(np.uint8)
    Image.fromarray(rgba).save(dst, "PNG", compress_level=1, optimize=False)


def main():
    frames = sorted(INPUT_DIR.glob("ezgif-frame-*.jpg"))
    if not frames:
        print(f"[ERROR] No frames found in {INPUT_DIR}")
        sys.exit(1)

    total = len(frames)
    print(f"remove_bg v4 -- {total} frames")
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
