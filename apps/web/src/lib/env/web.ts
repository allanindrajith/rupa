import { z } from "zod";

const webEnvSchema = z.object({
	// Node
	NODE_ENV: z.enum(["development", "production", "test"]),
	ANALYZE: z.string().optional(),
	NEXT_RUNTIME: z.enum(["nodejs", "edge"]).optional(),

	// Public
	NEXT_PUBLIC_SITE_URL: z.string().default("http://localhost:3000"),
	NEXT_PUBLIC_MARBLE_API_URL: z.string().default("https://api.marblecms.com"),

	// Server
	DATABASE_URL: z
		.string()
		.default("postgresql://opencut:opencut@localhost:5432/opencut"),

	BETTER_AUTH_SECRET: z.string().default("standalone-secret-opencut-studio"),
	UPSTASH_REDIS_REST_URL: z.string().default("http://localhost:8079"),
	UPSTASH_REDIS_REST_TOKEN: z.string().default("local-token"),
	MARBLE_WORKSPACE_KEY: z.string().default("local-marble-key"),
	FREESOUND_CLIENT_ID: z.string().default("local-freesound-id"),
	FREESOUND_API_KEY: z.string().default("local-freesound-key"),
});

export type WebEnv = z.infer<typeof webEnvSchema>;

export const webEnv = webEnvSchema.parse(process.env);
