let ticks = 120_000;
try {
	// Dynamically check if opencut-wasm is initialized
	const wasm = require("opencut-wasm");
	if (typeof wasm?.TICKS_PER_SECOND === "function") {
		ticks = wasm.TICKS_PER_SECOND();
	}
} catch {
	ticks = 120_000;
}

export const TICKS_PER_SECOND = ticks;
