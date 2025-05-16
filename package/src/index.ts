import { spawn } from "node:child_process";
import { getBinaryPath, getConfigPath } from "./fs.js";

import type { ChildProcessWithoutNullStreams } from "node:child_process";

export class LocalDango {
	instance?: ChildProcessWithoutNullStreams;
	async start() {
		getConfigPath();
		await new Promise((resolve, reject) => {
			this.instance = spawn(getBinaryPath(), ["run"]);
			this.instance.stdout.pipe(process.stdout);
			this.instance.stderr.pipe(process.stderr);
			this.instance.on("close", resolve);
			this.instance.on("error", reject);
			this.instance.on("spawn", () => setTimeout(resolve, 1000));
		});
	}

	stop() {
		if (this.instance) {
			this.instance.kill("SIGKILL");
			this.instance = undefined;
		}
	}
}
