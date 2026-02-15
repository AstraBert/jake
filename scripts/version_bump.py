import json
import re
import sys
from typing import Any, Literal, NamedTuple

import tomllib as toml


class Version(NamedTuple):
    major: int
    minor: int
    patch: int


def load_version_from_toml() -> tuple[str, str]:
    with open("Cargo.toml", "rb") as f:
        data = toml.load(f)
    with open("Cargo.toml", "r") as f:
        content = f.read()
    assert isinstance(data, dict)
    assert "package" in data
    assert isinstance(data["package"], dict)
    assert "version" in data["package"]
    assert isinstance(data["package"]["version"], str)
    return data["package"]["version"], content


def load_version_from_json() -> tuple[str, dict[str, Any]]:
    with open("package.json", "r") as f:
        data = json.load(f)
    assert isinstance(data, dict)
    assert "version" in data
    assert isinstance(data["version"], str)
    return data["version"], data


def parse_semver(version: str) -> Version:
    sep = version.split(".")
    assert len(sep) == 3
    assert all(s.isdigit() for s in sep)
    return Version(major=int(sep[0]), minor=int(sep[1]), patch=int(sep[2]))


def compare_semver(v1: Version, v2: Version) -> bool:
    return v1.major == v2.major and v1.minor == v2.minor and v1.patch == v2.patch


def semver_to_str(v: Version) -> str:
    return f"{v.major}.{v.minor}.{v.patch}"


def bump_semver(version: Version, bump_type: Literal["major", "minor", "patch"]) -> str:
    major = version.major
    minor = version.minor
    patch = version.patch
    if bump_type == "major":
        major += 1
        minor = 0
        patch = 0
    elif bump_type == "minor":
        minor += 1
        patch = 0
    else:
        patch += 1
    return semver_to_str(Version(major=major, minor=minor, patch=patch))


def new_version_to_json(data: dict[str, Any], new_version: str) -> None:
    data["version"] = new_version
    with open("package.json", "w") as f:
        json.dump(data, f, indent=2)


def new_version_to_toml(content: str, new_version: str) -> None:
    content = re.sub(
        r"^version\s*=\s*\"[^\"]+\"$",
        f'version = "{new_version}"',
        content,
        count=1,
        flags=re.MULTILINE,
    )
    with open("Cargo.toml", "w") as f:
        f.write(content)


def new_version_to_pre_install_js(old_version: str, new_version: str) -> None:
    with open("pre-install.js", "r") as f:
        content = f.read()
    old_v_log = f"Installing and compiling jake {old_version}"
    old_v_install = f"cargo install jake --vers {old_version}"
    new_v_log = f"Installing and compiling jake {new_version}"
    new_v_install = f"cargo install jake --vers {new_version}"
    content = content.replace(old_v_log, new_v_log).replace(
        old_v_install, new_v_install
    )
    with open("pre-install.js", "w") as f:
        f.write(content)


def new_command_version_main_rs(old_version: str, new_version: str) -> None:
    with open("src/main.rs", "r") as f:
        content = f.read()
    content = content.replace(
        f'#[command(version = "{old_version}")]',
        f'#[command(version = "{new_version}")]',
    )
    with open("src/main.rs", "w") as f:
        f.write(content)


def main() -> None:
    v_rust, toml_content = load_version_from_toml()
    v_js, json_data = load_version_from_json()
    semv_rust = parse_semver(v_rust)
    semv_js = parse_semver(v_js)
    assert compare_semver(semv_rust, semv_js), (
        f"Version {v_js} (package.json) and {v_rust} (Cargo.toml) do not match!"
    )
    bump_type = ""
    while True:
        bump_type = input(
            "What type of bump do you want to perform? [major/minor/patch] "
        )
        bump_type = bump_type.lower().strip()
        if bump_type not in ["major", "minor", "patch"]:
            print("Please provide a valid version bump (major, minor or patch)")
            continue
        else:
            new_ver = bump_semver(semv_rust, bump_type)  # type: ignore
            new_version_to_json(json_data, new_ver)
            new_version_to_toml(toml_content, new_ver)
            new_version_to_pre_install_js(v_rust, new_ver)
            new_command_version_main_rs(v_rust, new_ver)
            break
    sys.exit(0)


if __name__ == "__main__":
    main()
