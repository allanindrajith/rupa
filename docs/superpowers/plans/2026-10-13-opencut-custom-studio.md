# OpenCut Creator Studio Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform the repository into OpenCut Creator Studio by importing the working OpenCut-0.3.0 base, running client-first without Docker, and delivering short-form aspect ratios, safe-zone overlays, AI auto-captions, TTS voiceovers, silence trimming, and an Obsidian Studio UI reskin.

**Architecture:** Next.js 16 with Turbopack and React 19; client-side media storage using OPFS and IndexedDB; WebCodecs and MediaBunny canvas compositing; Whisper WebWorker transcription; Web Speech/Edge synthesis for TTS; Zustand state management for timeline, preview, and properties.

**Tech Stack:** Next.js 16, React 19, Tailwind CSS v4, Radix UI, Zustand, MediaBunny, WebCodecs, `@huggingface/transformers` (Whisper), Web Speech API, Bun.

**Spec:** [`docs/superpowers/specs/2026-10-13-opencut-custom-studio-design.md`](file:///Users/allanindrajith/Github%20Project/OpenCut/docs/superpowers/specs/2026-10-13-opencut-custom-studio-design.md)

## Global Constraints

- Must import and build upon `~/Downloads/OpenCut-0.3.0` codebase.
- No mandatory background Docker/Postgres/Redis services required to run, edit, or export videos locally.
- All media and timeline data must remain 100% client-side (OPFS / IndexedDB) with zero remote telemetry.
- Brand name remains OpenCut Studio for this release.
- Editor must support 9:16, 1:1, 4:5, and 16:9 canvas aspect ratios.
- Safe-zone overlays must cover TikTok, Instagram Reels, and YouTube Shorts.

## Review Focus

1. **Aspect Ratio Switching:** Switching between 16:9 and 9:16 mid-project must recalculate preview canvas bounding box and clip centering without distorting existing clip coordinate transforms.
2. **Audio Track Synchronization:** Auto-generated captions and TTS audio clips must align accurately with playhead and audio timestamps without drifting during playback.
3. **Local Storage Limits:** Large media files in OPFS must not crash the browser memory; video caching must release decoded frame buffers when playhead is idle.
4. **Offline Resilience:** If network connection is unavailable, TTS falls back cleanly to local browser SpeechSynthesis voices, and Whisper worker displays clear progress or graceful fallback.
5. **MP4 Export Geometry:** Video exports in 9:16 must output exact 1080x1920 dimensions rather than letterboxed 1920x1080 canvas.

---

### Task 1: Codebase Ingestion & Standalone Local Environment Setup

**Files:**
- Modify: Root workspace (`/Users/allanindrajith/Github Project/OpenCut`)
- Modify: `apps/web/package.json`
- Modify: `apps/web/.env.example` -> `apps/web/.env.local`

**Interfaces:**
- Consumes: `~/Downloads/OpenCut-0.3.0` source tree
- Produces: Runnable standalone monorepo with `apps/web`, `rust/`, root configurations, and dependencies installed via Bun.

- [ ] **Step 1: Synchronize OpenCut-0.3.0 files into workspace**
  Sync all directories and files from `~/Downloads/OpenCut-0.3.0` into `/Users/allanindrajith/Github Project/OpenCut` (excluding `.git` from downloads).
- [ ] **Step 2: Configure local environment file**
  Create `apps/web/.env.local` with local development fallback settings disabling mandatory cloud auth and remote databases.
- [ ] **Step 3: Install dependencies**
  Run `bun install` in the repository root.
- [ ] **Step 4: Verify Next.js dev server boots**
  Run: `bun --cwd apps/web next info` or `bun dev:web` verification.
- [ ] **Step 5: Commit**
  ```bash
  git add -A
  git commit -m "feat: ingest OpenCut-0.3.0 base and configure standalone local setup"
  ```

---

### Task 2: Social & Short-Form Canvas Aspect Ratio System

**Files:**
- Create: `apps/web/src/components/editor/panels/preview/aspect-ratio-selector.tsx`
- Modify: `apps/web/src/stores/preview-store.ts`
- Modify: `apps/web/src/components/editor/panels/preview/canvas.tsx`
- Modify: `apps/web/src/components/editor/panels/preview/index.tsx`

**Interfaces:**
- Consumes: `preview-store.ts` canvas dimension state
- Produces:
  ```typescript
  export type AspectRatio = '16:9' | '9:16' | '1:1' | '4:5';
  interface PreviewState {
    aspectRatio: AspectRatio;
    setAspectRatio: (ratio: AspectRatio) => void;
    fitMode: 'fit' | 'fill' | 'blur';
    setFitMode: (mode: 'fit' | 'fill' | 'blur') => void;
  }
  ```

- [ ] **Step 1: Write test / verification helper for aspect ratio dimension calculation**
  Verify dimensions: 16:9 (1920x1080), 9:16 (1080x1920), 1:1 (1080x1080), 4:5 (1080x1350).
- [ ] **Step 2: Add `aspectRatio` and `fitMode` state to `preview-store.ts`**
  Add state properties, actions, and resolution mappings.
- [ ] **Step 3: Implement `aspect-ratio-selector.tsx` component**
  Add dropdown / segmented control with icons in the preview controls toolbar.
- [ ] **Step 4: Update `canvas.tsx` to react to aspect ratio changes**
  Ensure canvas aspect ratio, container sizing, and transform matrices update dynamically on change.
- [ ] **Step 5: Verify in browser / component test**
  Verify toggling ratio updates canvas viewport without errors.
- [ ] **Step 6: Commit**
  ```bash
  git add apps/web/src/components/editor/panels/preview/ apps/web/src/stores/preview-store.ts
  git commit -m "feat: add social aspect ratio switcher (9:16, 1:1, 4:5, 16:9)"
  ```

---

### Task 3: Interactive Social Safe-Zone Overlays

**Files:**
- Create: `apps/web/src/components/editor/panels/preview/safe-zone-overlay.tsx`
- Modify: `apps/web/src/stores/preview-store.ts`
- Modify: `apps/web/src/components/editor/panels/preview/canvas.tsx`

**Interfaces:**
- Consumes: `preview-store.ts` (canvas aspect ratio)
- Produces:
  ```typescript
  export type SafeZonePlatform = 'none' | 'tiktok' | 'reels' | 'shorts';
  interface PreviewState {
    safeZonePlatform: SafeZonePlatform;
    setSafeZonePlatform: (platform: SafeZonePlatform) => void;
  }
  ```

- [ ] **Step 1: Define safe-zone overlay geometry specifications**
  TikTok: bottom 160px text zone, right 70px icon column.
  Reels: bottom 180px text zone, right 64px icon column.
  Shorts: bottom 120px metadata zone, right 60px action column.
- [ ] **Step 2: Implement `safe-zone-overlay.tsx`**
  Renders SVG/CSS semi-transparent guide overlays locked to canvas bounds.
- [ ] **Step 3: Integrate safe-zone toggle button in preview control bar**
  Add menu button allowing one-click selection of TikTok, Reels, Shorts, or Off.
- [ ] **Step 4: Verify overlay appears only on 9:16 canvas and does not render in export**
- [ ] **Step 5: Commit**
  ```bash
  git add apps/web/src/components/editor/panels/preview/ apps/web/src/stores/preview-store.ts
  git commit -m "feat: add interactive safe-zone overlays for TikTok, Reels, and Shorts"
  ```

---

### Task 4: AI Auto-Captions Timeline Pipeline

**Files:**
- Create: `apps/web/src/components/editor/panels/assets/views/captions-view.tsx`
- Create: `apps/web/src/lib/caption-styles.ts`
- Modify: `apps/web/src/components/editor/panels/assets/index.tsx`
- Modify: `apps/web/src/stores/timeline-store.ts`

**Interfaces:**
- Consumes: `apps/web/src/services/transcription/service.ts`
- Produces:
  ```typescript
  export interface CaptionStylePreset {
    id: 'mrbeast' | 'neon' | 'subtle' | 'bold-yellow';
    name: string;
    fontFamily: string;
    fontSize: number;
    color: string;
    strokeColor?: string;
    strokeWidth?: number;
    backgroundColor?: string;
    textTransform?: 'uppercase' | 'none';
  }
  export function createCaptionTrack(segments: { start: number; end: number; text: string }[], style: CaptionStylePreset): void;
  ```

- [ ] **Step 1: Create `apps/web/src/lib/caption-styles.ts`**
  Define presets (`mrbeast`, `neon`, `subtle`, `bold-yellow`) with typography and color definitions.
- [ ] **Step 2: Add caption insertion action to `timeline-store.ts`**
  Add helper method that creates a `text` track named "Captions" and inserts text elements for all transcribed segments.
- [ ] **Step 3: Build `captions-view.tsx` in Assets Panel**
  Add UI allowing users to select target audio clip, language, model, and style preset, and display transcription progress.
- [ ] **Step 4: Wire transcription service trigger**
  Extract audio buffer from active video/audio clip, pass to `transcriptionService.transcribe()`, and feed results into `createCaptionTrack`.
- [ ] **Step 5: Verify caption generation on sample audio**
- [ ] **Step 6: Commit**
  ```bash
  git add apps/web/src/components/editor/panels/assets/ apps/web/src/lib/caption-styles.ts apps/web/src/stores/timeline-store.ts
  git commit -m "feat: add AI auto-captions generator with creator typography presets"
  ```

---

### Task 5: AI Text-to-Speech (TTS) Voiceover Generator

**Files:**
- Create: `apps/web/src/services/tts/tts-service.ts`
- Create: `apps/web/src/components/editor/panels/assets/views/tts-view.tsx`
- Modify: `apps/web/src/components/editor/panels/assets/index.tsx`

**Interfaces:**
- Consumes: Web Speech API / browser AudioContext
- Produces:
  ```typescript
  export interface TTSOptions {
    text: string;
    voiceURI?: string;
    rate?: number;
    pitch?: number;
  }
  export class TTSService {
    getVoices(): Promise<SpeechSynthesisVoice[]>;
    synthesizeToAudioBlob(options: TTSOptions): Promise<{ blob: Blob; duration: number }>;
  }
  ```

- [ ] **Step 1: Implement `tts-service.ts`**
  Use SpeechSynthesis and MediaRecorder (or AudioContext synthesis) to generate audio blobs from text scripts.
- [ ] **Step 2: Build `tts-view.tsx` in Assets Panel**
  Script text area, voice selector dropdown, speed/pitch sliders, and "Preview" / "Add to Timeline" buttons.
- [ ] **Step 3: Connect "Add to Timeline" to `timeline-store.ts`**
  Save blob to local OPFS video/audio cache and insert an `AudioElement` on the active audio track at playhead position.
- [ ] **Step 4: Verify speech synthesis and timeline placement**
- [ ] **Step 5: Commit**
  ```bash
  git add apps/web/src/services/tts/ apps/web/src/components/editor/panels/assets/
  git commit -m "feat: add AI text-to-speech voiceover generator"
  ```

---

### Task 6: Smart Silence Remover / Dead Air Trimmer

**Files:**
- Create: `apps/web/src/services/audio/silence-detector.ts`
- Create: `apps/web/src/components/editor/dialogs/silence-remover-dialog.tsx`
- Modify: `apps/web/src/components/editor/panels/timeline/timeline-toolbar.tsx`

**Interfaces:**
- Consumes: `AudioBuffer` from selected audio/video element in `timeline-store`
- Produces:
  ```typescript
  export interface SilenceInterval {
    start: number; // seconds
    end: number;   // seconds
    duration: number;
  }
  export function detectSilences(buffer: AudioBuffer, thresholdDb?: number, minDurationSec?: number): SilenceInterval[];
  export function removeSilencesFromClip(clipId: string, intervals: SilenceInterval[]): void;
  ```

- [ ] **Step 1: Implement `silence-detector.ts`**
  Analyze RMS volume across audio channels in 50ms chunks; detect regions where RMS < threshold (e.g. -35 dB) for > minDuration.
- [ ] **Step 2: Implement `silence-remover-dialog.tsx`**
  Shows count of detected silent gaps, total dead air duration saved, sensitivity threshold slider, and "Remove Silences" confirm button.
- [ ] **Step 3: Implement batch splitting and ripple delete in `timeline-store.ts`**
  Cuts clip at silence boundaries, drops silent intervals, and shifts subsequent clips leftward.
- [ ] **Step 4: Add "Remove Silences" button to timeline toolbar**
- [ ] **Step 5: Verify silence detection on sample audio**
- [ ] **Step 6: Commit**
  ```bash
  git add apps/web/src/services/audio/ apps/web/src/components/editor/dialogs/ apps/web/src/components/editor/panels/timeline/
  git commit -m "feat: add smart silence detection and 1-click dead air trimmer"
  ```

---

### Task 7: Obsidian Studio UI Reskin & Quick Export Presets

**Files:**
- Modify: `apps/web/src/styles/globals.css` (or CSS theme variables)
- Modify: `apps/web/src/components/editor/editor-header.tsx`
- Modify: `apps/web/src/components/editor/export-button.tsx`
- Modify: `apps/web/src/components/editor/panels/timeline/theme.ts`

**Interfaces:**
- Consumes: Studio color tokens (`#0B0D11`, `#14181F`, `#1E242E`, `#00E5FF`, `#7C3AED`)
- Produces: Updated styling across headers, toolbars, timeline tracks, and quick export profiles in `export-button.tsx`.

- [ ] **Step 1: Update design tokens in CSS and timeline theme**
  Apply obsidian dark background, high-contrast cyan/indigo active states, and refined track separators.
- [ ] **Step 2: Refine `editor-header.tsx` branding**
  Display "OpenCut Studio" badge with clean creator icon and status indicator.
- [ ] **Step 3: Expand `export-button.tsx` with one-click social presets**
  Add direct export profiles for TikTok/Reels (1080x1920 @ 60fps), YouTube (1080p @ 60fps), and Animated GIF.
- [ ] **Step 4: Verify UI appearance and export dialog functionality**
- [ ] **Step 5: Commit**
  ```bash
  git add apps/web/src/styles/ apps/web/src/components/editor/
  git commit -m "style: apply Obsidian Studio UI reskin and social quick-export presets"
  ```

---

### Task 8: End-to-End Build & Functional Verification

**Files:**
- Modify: Any configuration or type adjustments identified during verification

**Interfaces:**
- Consumes: Complete project workspace
- Produces: Verified production build and passing dev server runtime

- [ ] **Step 1: Run typecheck and linting**
  `bun --cwd apps/web run build` or `bun run check`.
- [ ] **Step 2: Launch dev server and verify in browser**
  `bun dev:web` (or `bun run dev` in `apps/web`).
- [ ] **Step 3: Smoke test all critical user flows**
  - Import video file.
  - Switch aspect ratio to 9:16 and toggle TikTok safe-zone overlay.
  - Trigger Auto-Captions and verify subtitle track appears.
  - Generate TTS voiceover and place on timeline.
  - Perform silence removal.
  - Trigger test export.
- [ ] **Step 4: Final commit**
  ```bash
  git add -A
  git commit -m "chore: complete OpenCut Creator Studio integration and verification"
  ```
