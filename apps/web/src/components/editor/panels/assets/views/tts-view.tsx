"use client";

import { useState } from "react";
import { PanelView } from "@/components/editor/panels/assets/views/base-panel";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { Slider } from "@/components/ui/slider";
import { Spinner } from "@/components/ui/spinner";
import {
	Section,
	SectionContent,
	SectionField,
	SectionFields,
} from "@/components/section";
import {
	AiVoiceIcon,
	PlayIcon,
	StopIcon,
	SparklesIcon,
	PlusSignIcon,
} from "@hugeicons/core-free-icons";
import { HugeiconsIcon } from "@hugeicons/react";
import {
	VOICE_PRESETS,
	generateVoiceoverWav,
	previewBrowserSpeech,
	stopBrowserSpeech,
} from "@/services/tts/tts-service";
import { processMediaAssets } from "@/lib/media/processing";
import { buildElementFromMedia } from "@/lib/timeline/element-utils";
import { useEditor } from "@/hooks/use-editor";
import { TICKS_PER_SECOND } from "@/lib/wasm";

const SAMPLE_SCRIPTS = [
	{
		label: "Viral Hook",
		text: "Stop scrolling! Here are three editing secrets top creators use to get millions of views.",
	},
	{
		label: "Story Intro",
		text: "Have you ever wondered what happens when you combine AI with high-energy editing? Let me show you.",
	},
	{
		label: "Call to Action",
		text: "Drop a like if this helped, and make sure to subscribe for more creator studio breakdowns!",
	},
];

export function TtsVoiceoverView() {
	const [text, setText] = useState(
		"Welcome to OpenCut Studio! Type any script here to generate a custom voiceover directly on your timeline.",
	);
	const [selectedVoiceId, setSelectedVoiceId] = useState("storyteller");
	const [speed, setSpeed] = useState(1.0);
	const [pitch, setPitch] = useState(1.0);
	const [isPlayingPreview, setIsPlayingPreview] = useState(false);
	const [isGenerating, setIsGenerating] = useState(false);
	const [statusMessage, setStatusMessage] = useState<string | null>(null);

	const editor = useEditor();
	const activeProject = useEditor((e) => e.project.getActive());

	const words = text.trim().split(/\s+/).filter(Boolean);
	const estimatedSeconds = Math.max(1, Math.round(words.length * (0.4 / speed)));

	const handleAudition = () => {
		if (isPlayingPreview) {
			stopBrowserSpeech();
			setIsPlayingPreview(false);
		} else {
			setIsPlayingPreview(true);
			previewBrowserSpeech(text, { speed, pitch, voiceId: selectedVoiceId });
			// Auto reset preview state after estimated duration
			setTimeout(() => {
				setIsPlayingPreview(false);
			}, (estimatedSeconds + 1) * 1000);
		}
	};

	const handleAddToTimeline = async () => {
		if (!text.trim() || isGenerating) return;

		setIsGenerating(true);
		setStatusMessage("Synthesizing voiceover audio...");
		try {
			const result = await generateVoiceoverWav({
				text,
				voiceId: selectedVoiceId,
				speed,
				pitch,
			});

			setStatusMessage("Processing audio track...");
			const fileName = `voiceover-${selectedVoiceId}-${Date.now()}.wav`;
			const file = new File([result.blob], fileName, { type: "audio/wav" });

			const processedAssets = await processMediaAssets({ files: [file] });
			if (processedAssets.length > 0 && activeProject) {
				const asset = processedAssets[0];
				const createdAsset = await editor.media.addMediaAsset({
					projectId: activeProject.metadata.id,
					asset,
				});
				if (!createdAsset) return;

				const duration = Math.round(
					(createdAsset.duration ?? result.durationSeconds) * TICKS_PER_SECOND,
				);
				const currentTime = editor.playback.getCurrentTime();
				const startTime = Math.round(currentTime * TICKS_PER_SECOND);

				const element = buildElementFromMedia({
					mediaId: createdAsset.id,
					mediaType: "audio",
					name: `Voiceover: ${text.slice(0, 24)}...`,
					duration,
					startTime,
				});

				editor.timeline.insertElement({
					element,
					placement: { mode: "auto" },
				});

				setStatusMessage("Voiceover placed on timeline!");
				setTimeout(() => setStatusMessage(null), 3000);
			}
		} catch (error) {
			console.error("Failed to generate voiceover:", error);
			setStatusMessage(
				error instanceof Error ? error.message : "Failed to generate voiceover",
			);
		} finally {
			setIsGenerating(false);
		}
	};

	const selectedPreset =
		VOICE_PRESETS.find((v) => v.id === selectedVoiceId) ?? VOICE_PRESETS[0];

	return (
		<PanelView
			title="AI Voiceover"
			contentClassName="px-0 flex flex-col h-full"
			actions={
				<div className="flex items-center gap-1.5 text-xs text-muted-foreground font-mono">
					<span>{words.length} words</span>
					<span>•</span>
					<span>~{estimatedSeconds}s</span>
				</div>
			}
		>
			<Section showTopBorder={false} showBottomBorder={false} className="flex-1">
				<SectionContent className="flex flex-col gap-4 h-full pt-1">
					{/* Quick Sample Prompts */}
					<div className="flex items-center gap-1.5 overflow-x-auto pb-1">
						<span className="text-[10px] text-muted-foreground font-mono uppercase tracking-wider shrink-0 mr-1 flex items-center gap-1">
							<HugeiconsIcon icon={SparklesIcon} size={12} className="text-yellow-400" />
							Templates:
						</span>
						{SAMPLE_SCRIPTS.map((sample) => (
							<button
								key={sample.label}
								type="button"
								onClick={() => setText(sample.text)}
								className="text-[11px] px-2 py-0.5 rounded-full border border-border bg-secondary/50 hover:bg-secondary text-foreground whitespace-nowrap transition-colors"
							>
								{sample.label}
							</button>
						))}
					</div>

					{/* Script Input Textarea */}
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-medium text-foreground">
							Voiceover Script
						</label>
						<Textarea
							value={text}
							onChange={(e) => setText(e.target.value)}
							placeholder="Enter script to speak..."
							rows={4}
							className="resize-none font-sans text-sm leading-relaxed"
						/>
					</div>

					<SectionFields>
						<SectionField label="Voice Character">
							<Select
								value={selectedVoiceId}
								onValueChange={(val) => setSelectedVoiceId(val)}
							>
								<SelectTrigger>
									<SelectValue placeholder="Select a voice" />
								</SelectTrigger>
								<SelectContent>
									{VOICE_PRESETS.map((v) => (
										<SelectItem key={v.id} value={v.id}>
											<div className="flex items-center gap-2">
												<HugeiconsIcon icon={AiVoiceIcon} size={14} className="text-primary" />
												<span className="font-medium">{v.name}</span>
											</div>
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</SectionField>

						{/* Voice details badge */}
						<div className="text-[11px] text-muted-foreground px-2 py-1 rounded bg-secondary/20 border border-border/40">
							<p>{selectedPreset.description}</p>
							<div className="flex gap-1.5 mt-1.5">
								{selectedPreset.tags.map((t) => (
									<span
										key={t}
										className="text-[9px] px-1.5 py-0.2 rounded bg-secondary text-foreground font-mono"
									>
										{t}
									</span>
								))}
							</div>
						</div>

						<SectionField label={`Speed: ${speed.toFixed(2)}x`}>
							<Slider
								value={[speed]}
								min={0.7}
								max={1.6}
								step={0.05}
								onValueChange={([val]) => setSpeed(val)}
							/>
						</SectionField>

						<SectionField label={`Pitch: ${pitch.toFixed(2)}x`}>
							<Slider
								value={[pitch]}
								min={0.75}
								max={1.35}
								step={0.05}
								onValueChange={([val]) => setPitch(val)}
							/>
						</SectionField>
					</SectionFields>

					{statusMessage && (
						<div className="p-2.5 rounded border border-primary/20 bg-primary/10 text-primary text-xs text-center font-medium animate-in fade-in">
							{statusMessage}
						</div>
					)}

					{/* Action Buttons */}
					<div className="mt-auto flex flex-col gap-2 pt-2">
						<div className="grid grid-cols-2 gap-2">
							<Button
								type="button"
								variant="outline"
								onClick={handleAudition}
								disabled={!text.trim() || isGenerating}
								className="gap-1.5"
							>
								<HugeiconsIcon
									icon={isPlayingPreview ? StopIcon : PlayIcon}
									size={14}
								/>
								{isPlayingPreview ? "Stop" : "Audition"}
							</Button>
							<Button
								type="button"
								onClick={handleAddToTimeline}
								disabled={!text.trim() || isGenerating}
								className="gap-1.5 bg-primary text-primary-foreground font-semibold"
							>
								{isGenerating ? (
									<Spinner className="mr-1" />
								) : (
									<HugeiconsIcon icon={PlusSignIcon} size={14} />
								)}
								{isGenerating ? "Synthesizing..." : "Add to Timeline"}
							</Button>
						</div>
					</div>
				</SectionContent>
			</Section>
		</PanelView>
	);
}
