import { readFileSync } from "fs";
import { fork } from "child_process";

const fileName = process.argv[2] ?? "";

if (!process.env["IS_FORKED_CHILD"]) {
    console.log(
        `This is process pid: ${process.pid}, with ppid: ${process.ppid}, making babies`,
    );
    // Fork processes to run test cases
    const tests = ["src/no_file", "src/is_binary", "src/valid_file"];

    for (let test of tests) {
        const child = fork("./src/index.ts", [test], {
            env: { ...process.env, IS_FORKED_CHILD: "true" },
        });

        child.on("message", (message) => {
            console.log(
                `\nMessage from child ${child.pid}: ${JSON.stringify(message)}`,
            );
        });

        child.on("exit", (code) => {
            console.log(`\nChild ${child.pid} exited with code ${code}`);
        });
    }
} else {
    let file: Buffer;
    let fileString: string;

    console.log(
        `This is process pid: ${process.pid}, with ppid: ${process.ppid}, reading file: ${fileName}`,
    );

    try {
        file = readFileSync(fileName);
    } catch (e) {
        // Send error back to parent
        if (process.send) {
            process.send(
                "Error: " + (e instanceof Error ? e.message : String(e)),
            );
        }
        process.exit(1);
    }

    try {
        fileString = file.toString();
        if (fileString.includes("�")) {
            throw new Error("File contains non-UTF-8 characters");
        }
    } catch (e) {
        if (process.send) {
            process.send(
                "Error: " + (e instanceof Error ? e.message : String(e)),
            );
        }
        process.exit(2);
    }

    if (process.send) {
        process.send("Success, sending file contents back to parent");

        for (let line of fileString.split("\n"))
            process.send(isNaN(parseInt(line)) ? "Not a number!" : line);
    }
}
