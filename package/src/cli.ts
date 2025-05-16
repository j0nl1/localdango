#!/usr/bin/env node
import { Command } from "commander";
import { spawn } from "node:child_process";

import { getBinaryPath } from "./fs.js";

(async () => {
	new Command("localdango")
		.addCommand(
			new Command("start")
				.description("Start the localdango server")
				.option("--http-port <number>", "HTTP port to listen on", "8080")
				.option("--faucet-port <number>", "Faucet port to listen on", "8082")
				.option("--chain_id <string>", "Chain ID to use", "localdango-1")
				.action(({ httpPort, faucetPort, chainId }) => {
					const instance = spawn(getBinaryPath(), [
						"run",
						"--http-port",
						httpPort,
						"--faucet-port",
						faucetPort,
						"--chain-id",
						chainId,
					]);
					instance.stdout.pipe(process.stdout);
					instance.stderr.pipe(process.stderr);
				}),
		)
		.parse(process.argv);
})();
