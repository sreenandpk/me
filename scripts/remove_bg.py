"""
Background removal script for anime character animation frames.
Uses flood-fill from corners to identify the uniform gray background,
then removes it, producing transparent PNGs.

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

# ── Tuning parameters ──────────────────────────────────────────────────────────
# The background gray.  All three frames sampled ≈ (179,179,179).
BG_TOLERANCE   = 30   # per-channel tolerance for background colour
EDGE_ERODE_PX  = 1    # extra erosion at the alpha edge to kill halos
SHADOW_THRESH  = 40   # pixels darker than BG by this much = shadow → transparent

def flood_fill_mask(rgb_arr: np.ndarray, seed_coords, tol: int) -> np.ndarray:
    """
    BFS flood-fill from seed_coords on rgb_arr.
    Returns a boolean mask (True = background).
    """
    h, w = rgb_arr.shape[:2]
    visited = np.zeros((h, w), dtype=bool)
    q = deque()

    for (sy, sx) in seed_coords:
        if 0 <= sy < h and 0 <= sx < w and not visited[sy, sx]:
            q.append((sy, sx))
            visited[sy, sx] = True

    # Sample background colour from the four corner regions (avg)
    corners = [rgb_arr[0, 0], rgb_arr[0, w-1], rgb_arr[h-1, 0], rgb_arr[h-1, w-1]]
    bg_color = np.mean(corners, axis=0)  # shape (3,)

    while q:
        y, x = q.popleft()
        for dy, dx in [(-1,0),(1,0),(0,-1),(0,1)]:
            ny, nx = y + dy, x + dx
            if 0 <= ny < h and 0 <= nx < w and not visited[ny, nx]:
                diff = np.abs(rgb_arr[ny, nx].astype(int) - bg_color)
                if np.all(diff <= tol):
                    visited[ny, nx] = True
                    q.append((ny, nx))

    return visited  # True = background region


def remove_background(in_path: Path, out_path: Path) -> None:
    img  = Image.open(in_path).convert("RGBA")
    rgba = np.array(img, dtype=np.uint8)
    rgb  = rgba[:, :, :3]
    h, w = rgb.shape[:2]

    # ── 1. Flood-fill from all four edges ──────────────────────────────────────
    seeds = (
        [(0, x) for x in range(w)]          # top row
      + [(h-1, x) for x in range(w)]        # bottom row
      + [(y, 0) for y in range(h)]          # left col
      + [(y, w-1) for y in range(h)]        # right col
    )
    bg_mask = flood_fill_mask(rgb, seeds, BG_TOLERANCE)

    # ── 2. Also remove the drop-shadow (desaturated gray near bottom) ──────────
    # Shadow pixels: close to bg colour but slightly darker, below 65% of image
    bg_color = np.mean([rgb[0,0], rgb[0,w-1], rgb[h-1,0], rgb[h-1,w-1]], axis=0)
    shadow_start_row = int(h * 0.65)
    shadow_region = rgb[shadow_start_row:, :]
    diff_from_bg = np.abs(shadow_region.astype(int) - bg_color)
    # Shadow = grayish, closer to bg than character but distinct from pure bg
    shadow_close = np.all(diff_from_bg <= BG_TOLERANCE + SHADOW_THRESH, axis=-1)
    shadow_mask  = np.zeros((h, w), dtype=bool)
    shadow_mask[shadow_start_row:, :] = shadow_close

    # ── 3. Remove the sparkle watermark (bottom-right bright pixel cluster) ────
    # The sparkle is white/near-white in the bottom-right 15% of the image
    watermark_row = int(h * 0.75)
    watermark_col = int(w * 0.75)
    wm_region = rgb[watermark_row:, watermark_col:]
    # Bright pixels in that area that are also background-ish
    wm_bright = np.all(wm_region > 220, axis=-1)
    wm_close  = np.all(np.abs(wm_region.astype(int) - bg_color) <= BG_TOLERANCE + 60, axis=-1)
    watermark_mask = np.zeros((h, w), dtype=bool)
    watermark_mask[watermark_row:, watermark_col:] = wm_bright | wm_close

    # ── 4. Combine all transparency masks ──────────────────────────────────────
    full_mask = bg_mask | shadow_mask | watermark_mask

    # ── 5. Anti-halo edge softening ────────────────────────────────────────────
    # For pixels just inside the character edge that still carry background colour:
    # do an additional pass — if a foreground pixel is very close to bg colour
    # and has ≥2 background neighbours, mark it transparent too.
    full_mask_out = full_mask.copy()
    for _ in range(EDGE_ERODE_PX):
        # Count bg neighbours for each fg pixel
        padded = np.pad(full_mask_out, 1, constant_values=False)
        neighbour_count = (
            padded[:-2, 1:-1].astype(int)  # up
          + padded[2:,  1:-1].astype(int)  # down
          + padded[1:-1, :-2].astype(int)  # left
          + padded[1:-1, 2:].astype(int)   # right
        )
        fg_near_bg = (~full_mask_out) & (neighbour_count >= 2)
        diff = np.abs(rgb.astype(int) - bg_color)
        halo_like = np.all(diff <= BG_TOLERANCE + 15, axis=-1)
        full_mask_out |= (fg_near_bg & halo_like)

    # ── 6. Apply transparency ──────────────────────────────────────────────────
    rgba[:, :, 3] = np.where(full_mask_out, 0, 255).astype(np.uint8)

    result = Image.fromarray(rgba)
    result.save(out_path, "PNG", optimize=False)


def main():
    # Gather all frames in order
    frame_files = sorted(INPUT_DIR.glob("ezgif-frame-*.jpg"))

    if not frame_files:
        print(f"[ERROR] No frames found in {INPUT_DIR}")
        sys.exit(1)

    total = len(frame_files)
    print(f"Processing {total} frames...")
    print(f"  Input:  {INPUT_DIR}")
    print(f"  Output: {OUTPUT_DIR}")
    print()

    for i, src in enumerate(frame_files, start=1):
        # Rename to canonical frame-NNN.png
        frame_num = i  # use sorted order as the canonical index
        dst_name  = f"frame-{frame_num:03d}.png"
        dst       = OUTPUT_DIR / dst_name

        print(f"  [{i:02d}/{total}] {src.name} -> {dst_name}", end="", flush=True)
        remove_background(src, dst)
        print(" OK")

    print()
    print(f"Done. {total} transparent PNGs saved to:")
    print(f"  {OUTPUT_DIR}")


if __name__ == "__main__":
    main()
