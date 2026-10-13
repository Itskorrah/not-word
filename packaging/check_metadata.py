#!/usr/bin/env python3
"""Check Not Word's public installation identity without building Rust.

Run from any directory: python3 packaging/check_metadata.py
Requires PyYAML. XML/Desktop/AppStream semantic validators and script parsers run
separately in packaging-lint.yml; this checks consistency between their inputs.
"""
from __future__ import annotations

import configparser
import plistlib
import re
import struct
import sys
import uuid
from pathlib import Path
from xml.etree import ElementTree

import yaml

ROOT = Path(__file__).resolve().parents[1]
APP_ID = "io.github.itskorrah.NotWord"
ERRORS: list[str] = []


def require(condition: bool, message: str) -> None:
    if not condition:
        ERRORS.append(message)


def load_yaml(path: Path) -> dict:
    with path.open(encoding="utf-8") as source:
        value = yaml.safe_load(source)
    if not isinstance(value, dict):
        raise ValueError(f"{path.relative_to(ROOT)} must contain a mapping")
    return value


def check() -> None:
    linux = ROOT / "packaging/linux"
    desktop = configparser.ConfigParser(interpolation=None)
    desktop.optionxform = str
    desktop.read(linux / f"{APP_ID}.desktop", encoding="utf-8")
    entry = desktop["Desktop Entry"]
    for key, expected in {
        "Name": "Not Word", "Exec": "not-word %F", "TryExec": "not-word",
        "Icon": APP_ID, "StartupWMClass": APP_ID, "Terminal": "false",
    }.items():
        require(entry.get(key) == expected, f"Desktop {key} must be {expected!r}")
    require("WordProcessor" in entry.get("Categories", ""), "Desktop must be a word processor")

    metadata = ElementTree.parse(linux / f"{APP_ID}.metainfo.xml.in").getroot()
    require(metadata.findtext("id") == APP_ID, "AppStream ID disagrees with desktop ID")
    require(metadata.findtext("name") == "Not Word", "AppStream product title must be Not Word")
    require(metadata.findtext("developer/name") == "Itskorrah", "AppStream maintainer must be Itskorrah")
    require(metadata.findtext("launchable") == f"{APP_ID}.desktop", "AppStream launchable must match desktop filename")
    require(metadata.findtext("icon") == APP_ID, "AppStream icon must match desktop icon")
    require(metadata.findtext("provides/binary") == "not-word", "AppStream executable must be not-word")
    desktop_mimes = set(entry.get("MimeType", "").split(";"))
    require(all(item.text in desktop_mimes for item in metadata.findall("provides/mediatype")), "AppStream media types must be present in the desktop entry")
    for link in metadata.findall("url"):
        require((link.text or "").startswith("https://github.com/Itskorrah/not-word"), "AppStream links must point to the maintained repository")

    manifests = [load_yaml(linux / "flatpak" / f"{APP_ID}{suffix}.yml") for suffix in ("", ".bundle")]
    for key in ("id", "runtime", "runtime-version", "sdk", "command", "finish-args"):
        require(manifests[0].get(key) == manifests[1].get(key), f"Flatpak manifests disagree on {key}")
    for manifest in manifests:
        require(manifest.get("id") == APP_ID, "Flatpak ID must match desktop and AppStream")
        require(manifest.get("command") == "not-word", "Flatpak command must be not-word")
        permissions = manifest.get("finish-args", [])
        require("--share=network" not in permissions, "Flatpak must keep document/network privacy by default")
        require("--filesystem=home" not in permissions and "--filesystem=host" not in permissions, "Flatpak must not grant unrestricted filesystem access")

    nfpm = load_yaml(linux / "nfpm.yaml")
    require(nfpm.get("name") == "not-word", "deb/rpm package name must be not-word")
    require(nfpm.get("vendor") == "Itskorrah", "deb/rpm vendor must be Itskorrah")
    destinations = {item.get("dst") for item in nfpm.get("contents", [])}
    require({"/usr/bin/not-word", "/usr/bin/not-word-cli"}.issubset(destinations), "deb/rpm must contain both primary binaries")

    with (ROOT / "packaging/macos/Info.plist.in").open("rb") as source:
        plist = plistlib.load(source)
    for key, expected in {
        "CFBundleIdentifier": APP_ID, "CFBundleName": "Not Word",
        "CFBundleDisplayName": "Not Word", "CFBundleExecutable": "not-word",
        "CFBundleIconFile": "not-word",
    }.items():
        require(plist.get(key) == expected, f"macOS {key} must be {expected!r}")
    require("WordCraft authors" in plist.get("NSHumanReadableCopyright", ""), "macOS must preserve accurate upstream copyright attribution")

    wix = ElementTree.parse(ROOT / "packaging/windows/not-word.wxs").getroot()
    namespace = {"w": "http://wixtoolset.org/schemas/v4/wxs"}
    package = wix.find("w:Package", namespace)
    if package is None:
        raise ValueError("Windows installer has no Package element")
    require(package.get("Name") == "Not Word", "MSI product title must be Not Word")
    require(package.get("Manufacturer") == "Itskorrah", "MSI manufacturer must be Itskorrah")
    guids = []
    for node in wix.iter():
        for attribute in ("UpgradeCode", "Guid"):
            value = node.get(attribute)
            if value is not None and value != "*":
                guids.append(str(uuid.UUID(value)))
    require(len(guids) == len(set(guids)), "MSI UpgradeCode/component GUIDs must be unique")
    old_guids = {"075497f9-e861-45d0-9c7f-a74f8c34e6ba", "855dcec6-9285-444f-9382-9654b83b34f6", "eff89e6a-f9b8-49f9-b3ae-e243347484b4"}
    require(not old_guids.intersection(guids), "Not Word must install independently of upstream WordCraft")
    files = {node.get("Source") for node in wix.findall(".//w:File", namespace)}
    require({r"$(var.BinDir)\not-word.exe", r"$(var.BinDir)\not-word-cli.exe"}.issubset(files), "MSI must install both primary binaries")
    for node in wix.findall(".//w:Icon", namespace):
        require(bool(re.search(r"\.(ico|exe)$", node.get("Id", ""))), "WiX Icon IDs must end in .ico or .exe")
    for node in wix.findall(".//w:ProgId", namespace):
        require(node.get("Id", "").startswith("NotWord."), "File associations must use independent NotWord ProgIds")
    for node in wix.findall(".//w:RegistryValue", namespace):
        require(node.get("Key", "") not in (r"Software\Classes\.docx", r"Software\Classes\.doc"), "Installer must never overwrite another editor's default document association")

    icons = ROOT / "assets/app-icon"
    for name in ("not-word.svg", "not-word.ico", "not-word.icns", "not-word-1024.png", "not-word-macos-512.png"):
        require((icons / name).is_file(), f"Missing product icon: {name}")
    for size in (16, 24, 32, 48, 64, 128, 256, 512):
        icon = icons / "hicolor" / f"{size}x{size}/apps/{APP_ID}.png"
        if icon.is_file():
            header = icon.read_bytes()[:24]
            require(len(header) == 24 and header[:8] == b"\x89PNG\r\n\x1a\n" and struct.unpack(">II", header[16:24]) == (size, size), f"Invalid hicolor icon dimensions: {size}")
        else:
            require(False, f"Missing hicolor icon at {size}px")
    require((icons / "hicolor/scalable/apps" / f"{APP_ID}.svg").is_file(), "Missing scalable Linux icon")

    for workflow_path in (ROOT / ".github/workflows").glob("*.yml"):
        workflow = load_yaml(workflow_path)
        for job in workflow.get("jobs", {}).values():
            uses = job.get("uses", "")
            require(not uses.startswith("storytold/craft-common-actions/.github/workflows/"), f"{workflow_path.name} must own product-specific orchestration locally")
            for step in job.get("steps", []):
                uses = step.get("uses", "")
                if uses and not uses.startswith("./"):
                    require(bool(re.fullmatch(r"[^@]+@[0-9a-f]{40}", uses)), f"{workflow_path.name}: external actions must be pinned to immutable commits: {uses}")


def main() -> int:
    try:
        check()
    except (OSError, ValueError, KeyError, configparser.Error, ElementTree.ParseError, yaml.YAMLError) as error:
        ERRORS.append(f"Could not validate packaging metadata: {error}")
    if ERRORS:
        for error in ERRORS:
            print(f"error: {error}", file=sys.stderr)
        return 1
    print("Packaging identity, associations, icons, Flatpak permissions and pinned workflows are consistent.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
