#!/usr/bin/env node
/**
 * Convert every .mp3 in an input folder to multiple target formats,
 * stripping all tags/artwork/chapters from the generated files,
 * AND also copy the original .mp3 files to the output test-dir.
 *
 * Uses:
 *  - @ffmpeg-installer/ffmpeg
 *  - @ffprobe-installer/ffprobe (verification)
 *  - execa
 *
 * Usage:
 *   node convert-from-mp3.mjs <inputDir> <outputDir> [formats]
 * Examples:
 *   node convert-from-mp3.mjs ./in ./test-dir
 *   node convert-from-mp3.mjs ./in ./test-dir flac,ogg,opus,wav,aiff,m4a,aac,wv,spx
 */

import { execa } from "execa";
import { path as ffmpegPath } from "@ffmpeg-installer/ffmpeg";
import { path as ffprobePath } from "@ffprobe-installer/ffprobe";
import fs from "fs/promises";
import path from "path";

// ---------- formats you want to generate ----------
const FORMATS = [
  // Lossless / uncompressed
  { key: "flac", ext: "flac", encoders: ["flac"], buildArgs: (e) => ["-c:a", e, "-compression_level", "5"] },
  { key: "wav",  ext: "wav",  encoders: ["pcm_s16le"], buildArgs: (e) => ["-c:a", e] },
  { key: "aiff", ext: "aiff", encoders: ["pcm_s16be"], buildArgs: (e) => ["-c:a", e] },

  // May be missing in packaged ffmpeg; script will skip if encoder not found
  { key: "wv",   ext: "wv",   encoders: ["libwavpack", "wavpack"], buildArgs: (e) => ["-c:a", e, "-compression_level", "3"] },

  // Lossy
  { key: "ogg",  ext: "ogg",  encoders: ["libvorbis", "vorbis"], buildArgs: (e) => ["-c:a", e, "-qscale:a", "5"] },
  { key: "opus", ext: "opus", encoders: ["libopus", "opus"],     buildArgs: (e) => ["-c:a", e, "-b:a", "128k"] },
  { key: "spx",  ext: "spx",  encoders: ["libspeex"],            buildArgs: (e) => ["-c:a", e, "-q:a", "6"] },
  { key: "aac",  ext: "aac",  encoders: ["libfdk_aac", "aac"],   buildArgs: (e) => ["-c:a", e, "-b:a", "192k"] },
  {
    key: "m4a",
    ext: "m4a",
    encoders: ["libfdk_aac", "aac"],
    buildArgs: (e) => ["-c:a", e, "-b:a", "192k"],
    containerExtras: ["-movflags", "+faststart", "-f", "mp4"],
  },
];

// ---------- helpers ----------
async function run(cmd, args) {
  try {
    const { stdout, stderr, exitCode } = await execa(cmd, args, { windowsHide: true });
    return { code: exitCode ?? 0, stdout, stderr };
  } catch (err) {
    return { code: typeof err.exitCode === "number" ? err.exitCode : 1, stdout: err.stdout || "", stderr: err.stderr || String(err) };
  }
}

let _encoderSet = null;
async function getEncoders() {
  if (_encoderSet) return _encoderSet;
  const { code, stdout } = await run(ffmpegPath, ["-hide_banner", "-encoders"]);
  if (code !== 0) throw new Error("Failed to list encoders from bundled ffmpeg.");
  const set = new Set();
  stdout.split("\n").forEach((line) => {
    const m = line.trim().match(/^[AVS]\S*\s+([^\s]+)\s/);
    if (m && m[1]) set.add(m[1]);
  });
  _encoderSet = set;
  return set;
}

async function listMp3s(root) {
  const results = [];
  async function walk(dir) {
    const entries = await fs.readdir(dir, { withFileTypes: true });
    for (const e of entries) {
      if (e.name.startsWith(".")) continue;
      const full = path.join(dir, e.name);
      if (e.isDirectory()) await walk(full);
      else if (e.isFile() && /\.mp3$/i.test(e.name)) results.push(full);
    }
  }
  await walk(root);
  results.sort();
  return results;
}

function buildFfmpegArgs(src, dst, encoder, cfg) {
  const base = [
    "-y", "-hide_banner", "-loglevel", "error",
    "-i", src,
    "-map", "0:a:0",  // only first audio stream
    "-vn",            // drop attached pictures
    "-map_metadata", "-1",
    "-map_chapters", "-1",
  ];
  const codecArgs = cfg.buildArgs(encoder);
  const tail = cfg.containerExtras ? [...cfg.containerExtras, dst] : [dst];
  return [...base, ...codecArgs, ...tail];
}

const TECHNICAL_TAGS = new Set([
  "major_brand","minor_version","compatible_brands","encoder","encoded_by","creation_time","date","language","handler_name",
]);

async function hasAnyNonTechnicalTags(file) {
  const { code, stdout } = await run(ffprobePath, ["-v","quiet","-show_entries","format_tags:stream_tags","-of","json", file]);
  if (code !== 0) return false;
  try {
    const j = JSON.parse(stdout || "{}");
    const fmtTags = j?.format?.tags || {};
    const streams = Array.isArray(j?.streams) ? j.streams : [];
    const fmtHas = Object.keys(fmtTags).some((k) => !TECHNICAL_TAGS.has(k.toLowerCase()));
    const streamHas = streams.some((s) => {
      const tags = s?.tags || {};
      return Object.keys(tags).some((k) => !TECHNICAL_TAGS.has(k.toLowerCase()));
    });
    return fmtHas || streamHas;
  } catch {
    return false;
  }
}

// ---------- main ----------
async function main() {
  const [, , inDir, outDir, onlyListRaw] = process.argv;
  if (!inDir || !outDir) {
    console.error("Usage: node convert-from-mp3.mjs <inputDir> <outputDir> [formats]");
    process.exit(1);
  }

  const onlyList = (onlyListRaw || "").split(",").map((s) => s.trim().toLowerCase()).filter(Boolean);
  await fs.mkdir(outDir, { recursive: true });

  const selected = onlyList.length ? FORMATS.filter((f) => onlyList.includes(f.key)) : FORMATS;
  if (selected.length === 0) {
    console.error("No valid target formats selected.");
    process.exit(1);
  }

  const encs = await getEncoders();
  const mp3s = await listMp3s(inDir);
  if (mp3s.length === 0) {
    console.log("No .mp3 files found in:", inDir);
    process.exit(0);
  }

  const manifest = [];

  for (const src of mp3s) {
    const rel = path.relative(inDir, src);          // e.g., sub/track.mp3
    const baseNoExt = rel.replace(/\.mp3$/i, "");   // e.g., sub/track
    const outBaseDir = path.join(outDir, path.dirname(rel));
    await fs.mkdir(outBaseDir, { recursive: true });

    const item = { source: src, mp3Copy: null, outputs: [] };

    // 1) Also copy the MP3 into the test-dir (as-is, mirrored structure)
    const mp3CopyDst = path.join(outBaseDir, path.basename(rel)); // keep same name
    try {
      await fs.copyFile(src, mp3CopyDst);
      item.mp3Copy = { path: mp3CopyDst, status: "ok" };
    } catch (err) {
      item.mp3Copy = { path: mp3CopyDst, status: "failed", reason: String(err).slice(0, 4000) };
    }

    // 2) Convert to each selected target format (tag-free)
    for (const cfg of selected) {
      const encoder = cfg.encoders.find((e) => encs.has(e));
      if (!encoder) {
        item.outputs.push({
          format: cfg.key,
          path: "",
          status: "skipped",
          reason: `No supported encoder found (tried: ${cfg.encoders.join(", ")})`,
        });
        continue;
      }

      const dst = path.join(outBaseDir, `${path.basename(baseNoExt)}.${cfg.ext}`);
      const args = buildFfmpegArgs(src, dst, encoder, cfg);
      const { code, stderr } = await run(ffmpegPath, args);

      if (code !== 0) {
        item.outputs.push({
          format: cfg.key,
          path: dst,
          status: "failed",
          reason: (stderr || "ffmpeg failed").trim().slice(0, 4000),
        });
        continue;
      }

      const hadTags = await hasAnyNonTechnicalTags(dst);
      item.outputs.push({ format: cfg.key, path: dst, status: "ok", hadTags });
    }

    manifest.push(item);
  }

  const manifestPath = path.join(outDir, "manifest.json");
  await fs.writeFile(manifestPath, JSON.stringify(manifest, null, 2));

  const flat = manifest.flatMap((m) => m.outputs);
  const okCount = flat.filter((o) => o.status === "ok").length;
  const skipCount = flat.filter((o) => o.status === "skipped").length;
  const failCount = flat.filter((o) => o.status === "failed").length;
  const tagWarn = flat.some((o) => o.status === "ok" && o.hadTags);

  console.log(`Done. Manifest: ${manifestPath}`);
  console.log(`OK: ${okCount}, Skipped: ${skipCount}, Failed: ${failCount}`);
  if (tagWarn) console.warn("⚠️  Some generated outputs appear to contain non-technical tags; review manifest.");
}

main().catch((e) => {
  console.error(e?.stack || String(e));
  process.exit(1);
});
