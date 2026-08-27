"""
Background removal script v3 — safe, sharp, smooth.

Changes from v2:
  - REMOVED the global neutral-gray pass (was destroying hair/skin edge pixels)
  - REMOVED the aggressive shadow strip (was eating boots/feet)
  - REPLACED with a tight enclosed-pocket detection:
      * Brightness > 155  (background ~179, character shadow <100)
      * Color distance to bg < 12  (tight – only catches true bg gray)
      * HSV saturation < 0.04  (pure neutral gray, not skin or shadow)
  - Flood-fill tolerance raised slightly (32) to better cross JPEG
    compression artifacts at silhouette edges
  - Watermark removal kept but constrained to a smaller corner region
  - Soft-alpha edge feathering for anti-aliased appearance
  - PNG saved at compress_level=1 (lossless, smaller than 0)

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
FLOOD_TOL       = 32    # exterior flood-fill colour tolerance
INNER_TOL       = 12    # enclosed-pocket match tolerance (tight)
INNER_SAT_MAX   = 0.04  # max saturation for enclosed pocket (pure gray only)
INNER_BRIGHT_MIN= 152   # min brightness for enclosed pocket (bg ~179, char <120)
EDGE_FEATHER    = 2     # px of soft alpha feathering at character edges
WM_ROW_FRAC     = 0.74  # watermark zone starts at this fraction of image height
WM_COL_FRAC     = 0.78  # watermark zone starts at this fraction of image width
WM_BRIGHT_MIN   = 200   # watermark pixels are near-white (sparkle + glow)


def sample_bg(rgb: np.ndarray) -> np.ndarray:
    h, w = rgb.shape[:2]
    s = 8
    patches = [rgb[:s,:s], rgb[:s,w-s:], rgb[h-s:,:s], rgb[h-s:,w-s:]]
    return np.mean([p.mean(axis=(0,1)) for p in patches], axis=0)


def flood_fill_exterior(rgb: np.ndarray, bg: np.ndarray, tol: int) -> np.ndarray:
    """BFS from all four edges. Returns bool mask (True = exterior background)."""
    h, w = rgb.shape[:2]
    mask = np.zeros((h, w), dtype=bool)
    q = deque()

    top    = [(0, x) for x in range(w)]
    bottom = [(h-1, x) for x in range(w)]
    left   = [(y, 0) for y in range(h)]
    right  = [(y, w-1) for y in range(h)]

    for sy, sx in top + bottom + left + right:
        if not mask[sy, sx]:
            diff = np.abs(rgb[sy, sx].astype(np.int32) - bg)
            if np.all(diff <= tol):
                mask[sy, sx] = True
                q.append((sy, sx))

    while q:
        y, x = q.popleft()
        for dy, dx in ((-1,0),(1,0),(0,-1),(0,1)):
            ny, nx = y+dy, x+dx
            if 0 <= ny < h and 0 <= nx < w and not mask[ny, nx]:
                diff = np.abs(rgb[ny, nx].astype(np.int32) - bg)
                if np.all(diff <= tol):
                    mask[ny, nx] = True
                    q.append((ny, nx))
    return mask


def find_enclosed_pockets(rgb: np.ndarray, exterior: np.ndarray,
                           bg: np.ndarray) -> np.ndarray:
    """
    Find background pixels trapped INSIDE the character silhouette.
    Uses very tight constraints to avoid hitting character pixels:
      - Must be close to background gray (±INNER_TOL per channel)
      - Must be near-neutral (saturation < INNER_SAT_MAX)
      - Must be bright (> INNER_BRIGHT_MIN) — character is dark
      - Must NOT already be in exterior mask
    """
    rgb_f = rgb.astype(np.float32)
    bg_f  = bg.astype(np.float32)

    # Colour distance to background
    diff  = np.abs(rgb_f - bg_f)
    close = np.all(diff <= INNER_TOL, axis=-1)

    # HSV saturation
    r, g, b = rgb_f[:,:,0], rgb_f[:,:,1], rgb_f[:,:,2]
    mx  = np.maximum(np.maximum(r, g), b)
    mn  = np.minimum(np.minimum(r, g), b)
    sat = np.where(mx > 1.0, (mx - mn) / mx, 0.0)
    neutral = sat < INNER_SAT_MAX

    # Brightness (must be close to the light background, not dark character)
    bright = (r + g + b) / 3.0 > INNER_BRIGHT_MIN

    return close & neutral & bright & (~exterior)


def apply_watermark_mask(rgb: np.ndarray) -> np.ndarray:
    """Remove the ezgif sparkle watermark in the bottom-right corner."""
    h, w = rgb.shape[:2]
    wr = int(h * WM_ROW_FRAC)
    wc = int(w * WM_COL_FRAC)
    wm = np.zeros((h, w), dtype=bool)
    region = rgb[wr:, wc:]
    # Sparkle core: very bright AND near-neutral (low saturation)
    r, g, b = region[:,:,0].astype(float), region[:,:,1].astype(float), region[:,:,2].astype(float)
    bright = np.all(region >= WM_BRIGHT_MIN, axis=-1)
    mx = np.maximum(np.maximum(r, g), b)
    mn = np.minimum(np.minimum(r, g), b)
    sat = np.where(mx > 1, (mx - mn) / mx, 0.0)
    neutral = sat < 0.15
    wm[wr:, wc:] = bright & neutral
    # Dilate by 4px to catch the soft glow around the sparkle
    from scipy.ndimage import binary_dilation
    wm = binary_dilation(wm, iterations=4)
    return wm


def feather_alpha(alpha: np.ndarray, bg_mask: np.ndarray,
                  rgb: np.ndarray, bg: np.ndarray) -> np.ndarray:
    """
    Soft-blend alpha at the character boundary for smooth anti-aliased edges.
    Pixels at the exact edge (adjacent to transparent area) get reduced alpha
    proportional to their colour similarity to the background.
    """
    alpha_out = alpha.copy()
    for _ in range(EDGE_FEATHER):
        pad = np.pad(bg_mask.astype(np.int16), 1, constant_values=0)
        neighbours = (
            pad[:-2, 1:-1] + pad[2:, 1:-1]
          + pad[1:-1, :-2] + pad[1:-1, 2:]
        )
        # Foreground pixels adjacent to at least one background pixel
        edge_fg = (~bg_mask) & (neighbours >= 1)

        # Compute a blend factor: 0 (fully keep) → 1 (fully remove) based on
        # how background-like the pixel is
        diff = np.abs(rgb.astype(np.float32) - bg.astype(np.float32))
        max_diff = np.max(diff, axis=-1)
        # blend = 1 if identical to bg, 0 if far from bg
        blend = np.clip(1.0 - max_diff / (FLOOD_TOL * 1.5), 0.0, 1.0)

        # Apply: reduce alpha at edge pixels proportional to bg-likeness
        reduce = (edge_fg & (blend > 0.05))
        alpha_out[reduce] = np.clip(
            alpha_out[reduce] * (1.0 - blend[reduce] * 0.8), 0, 255
        ).astype(np.uint8)

        # Update bg_mask to propagate
        bg_mask = bg_mask | (alpha_out == 0)

    return alpha_out


def remove_background(src: Path, dst: Path) -> None:
    img  = Image.open(src).convert("RGBA")
    rgba = np.array(img, dtype=np.uint8)
    rgb  = rgba[:, :, :3]

    bg = sample_bg(rgb)

    # ── Pass 1: exterior flood fill ──────────────────────────────────────────
    ext = flood_fill_exterior(rgb, bg, FLOOD_TOL)

    # ── Pass 2: tight enclosed-pocket detection ──────────────────────────────
    pockets = find_enclosed_pockets(rgb, ext, bg)

    # ── Pass 3: watermark ────────────────────────────────────────────────────
    wm = apply_watermark_mask(rgb)

    # ── Combine ──────────────────────────────────────────────────────────────
    bg_mask = ext | pockets | wm
    alpha   = np.where(bg_mask, 0, 255).astype(np.uint8)

    # ── Pass 4: soft alpha feathering at edges ───────────────────────────────
    alpha = feather_alpha(alpha, bg_mask.copy(), rgb, bg)

    # ── Save ─────────────────────────────────────────────────────────────────
    rgba[:, :, 3] = alpha
    Image.fromarray(rgba).save(dst, "PNG", compress_level=1, optimize=False)


def main():
    frames = sorted(INPUT_DIR.glob("ezgif-frame-*.jpg"))
    if not frames:
        print(f"[ERROR] No frames found in {INPUT_DIR}")
        sys.exit(1)

    total = len(frames)
    print(f"remove_bg v3 -- {total} frames")
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
