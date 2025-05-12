from typing import Optional
import subprocess


def generate_help_section(command: Optional[str], readme: str) -> str:
    command_args = ["cargo", "run", "--"]

    if command is not None:
        command_args.append(command)

    command_args.append("--help")

    help_output = "\n".join(
        [
            line.rstrip()
            for line in subprocess.run(command_args, capture_output=True)
            .stdout.decode()
            .splitlines()
        ]
    )

    key = (command if command is not None else "base") + "-command-help"

    start = f"<!-- START_SECTION:{key} -->"
    end = f"<!-- END_SECTION:{key} -->"

    lines: list[str] = []

    inside = False
    start_seen = False
    end_seen = False
    for line in readme.splitlines():
        if inside:
            if line == end:
                lines.append(line)

                inside = False
                end_seen = True

        elif line == start:
            lines.append(line)
            lines.append(f"\n```\n{help_output}\n```\n")
            inside = True
            start_seen = True
        else:
            lines.append(line)

    if start_seen and end_seen:
        return "\n".join(lines)

    return readme


def read_readme() -> str:
    with open("README.md", "r") as f:
        return f.read()


if __name__ == "__main__":
    content = read_readme()

    for command in [None, "format", "check", "init", "completions"]:
        content = generate_help_section(command, content)

    with open("README.md", "w") as readme_file:
        readme_file.write(content.strip() + "\n")
