import { fileURLToPath } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const getBinaryPath = () => {
	const isWindows = process.platform === "win32";
	const binaryName = isWindows ? "localdango.exe" : "localdango";
	return path.join(__dirname, "bin", binaryName);
};

export const getConfigPath = () => {
	console.log(process.cwd());
};
