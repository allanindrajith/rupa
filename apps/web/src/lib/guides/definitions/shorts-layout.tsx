export function YouTubeShortsLayout() {
	return (
		<svg
			width="100%"
			height="100%"
			viewBox="0 0 1080 1920"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
			className="pointer-events-none select-none opacity-85"
		>
			<title>YouTube Shorts Safe Zone Layout</title>
			{/* Safe Area Box Guide */}
			<rect
				x="70"
				y="180"
				width="880"
				height="1460"
				stroke="#FF0000"
				strokeWidth="2"
				strokeDasharray="8 8"
				fill="none"
				opacity="0.5"
			/>
			<text x="80" y="215" fill="#FF4444" fontSize="24" fontFamily="sans-serif" opacity="0.8">
				YouTube Shorts Safe Zone (Keep essential visuals & text inside)
			</text>

			{/* Top Header Icons (Search, Camera, More) */}
			<g opacity="0.75" fill="white" transform="translate(850, 90)">
				{/* Search */}
				<circle cx="0" cy="0" r="14" stroke="white" strokeWidth="3" fill="none" />
				<line x1="10" y1="10" x2="20" y2="20" stroke="white" strokeWidth="3" />
				{/* More menu */}
				<circle cx="100" cy="-8" r="4" />
				<circle cx="100" cy="0" r="4" />
				<circle cx="100" cy="8" r="4" />
			</g>

			{/* Right Sidebar Action Icons */}
			<g transform="translate(980, 1180)" opacity="0.85">
				{/* Like button (Thumbs up) */}
				<g transform="translate(0, 0)">
					<path
						d="M-15 15 V-5 C-15 -10 -10 -15 -5 -15 H8 C14 -15 18 -10 16 0 L14 15 H-15 Z"
						fill="white"
					/>
					<text x="0" y="45" fill="white" fontSize="22" textAnchor="middle" fontFamily="sans-serif">
						240K
					</text>
				</g>

				{/* Dislike button (Thumbs down) */}
				<g transform="translate(0, 110)">
					<path
						d="M-15 -15 V5 C-15 10 -10 15 -5 15 H8 C14 15 18 10 16 0 L14 -15 H-15 Z"
						fill="white"
					/>
					<text x="0" y="45" fill="white" fontSize="22" textAnchor="middle" fontFamily="sans-serif">
						Dislike
					</text>
				</g>

				{/* Comments button */}
				<g transform="translate(0, 220)">
					<path
						d="M-20 -15 H20 C24 -15 26 -12 26 -8 V10 C26 14 24 18 20 18 H-6 L-18 28 V18 H-20 C-24 18 -26 14 -26 10 V-8 C-26 -12 -24 -15 -20 -15 Z"
						fill="white"
					/>
					<text x="0" y="55" fill="white" fontSize="22" textAnchor="middle" fontFamily="sans-serif">
						3.4K
					</text>
				</g>

				{/* Share button */}
				<g transform="translate(0, 330)">
					<path d="M-18 -10 L18 0 L-18 10 L-10 0 Z" fill="white" />
					<text x="0" y="45" fill="white" fontSize="22" textAnchor="middle" fontFamily="sans-serif">
						Share
					</text>
				</g>

				{/* Remix button */}
				<g transform="translate(0, 430)">
					<circle cx="0" cy="0" r="18" stroke="white" strokeWidth="3" fill="none" />
					<text x="0" y="45" fill="white" fontSize="22" textAnchor="middle" fontFamily="sans-serif">
						Remix
					</text>
				</g>

				{/* Audio track icon */}
				<g transform="translate(0, 520)">
					<rect x="-18" y="-18" width="36" height="36" rx="6" stroke="white" strokeWidth="3" fill="#222" />
					<circle cx="0" cy="0" r="6" fill="white" />
				</g>
			</g>

			{/* Bottom Left Channel & Video Details */}
			<g transform="translate(60, 1680)" opacity="0.85">
				{/* Avatar */}
				<circle cx="30" cy="30" r="30" fill="#444" stroke="white" strokeWidth="2" />
				{/* Channel Name */}
				<text x="80" y="38" fill="white" fontSize="28" fontWeight="bold" fontFamily="sans-serif">
					@ShortsCreator
				</text>
				{/* Subscribe pill button */}
				<rect x="290" y="10" width="130" height="38" rx="19" fill="#CC0000" />
				<text x="355" y="35" fill="white" fontSize="20" fontWeight="bold" textAnchor="middle" fontFamily="sans-serif">
					Subscribe
				</text>

				{/* Video title */}
				<text x="0" y="95" fill="white" fontSize="26" fontFamily="sans-serif">
					Watch what happens next! 🔥 #Shorts #Viral
				</text>

				{/* Sound Pill */}
				<g transform="translate(0, 130)">
					<rect x="0" y="0" width="260" height="36" rx="18" fill="rgba(255,255,255,0.2)" />
					<path d="M16 23 L26 17 V29 A4 4 0 1 1 20 25 V21 Z" fill="white" />
					<text x="36" y="24" fill="white" fontSize="18" fontFamily="sans-serif">
						Original audio · Sound Track
					</text>
				</g>
			</g>
		</svg>
	);
}
