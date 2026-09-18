#!/usr/bin/env python3
"""Generate a minimal CycloneDX-style SBOM from Cargo.lock."""
import json, re, sys
from pathlib import Path

def main():
    lock = Path("Cargo.lock").read_text()
    pkgs = re.findall(r'name = "([^"]+)"\nversion = "([^"]+)"', lock)
    components = [{"name": n, "version": v, "type": "library"} for n, v in pkgs]
    bom = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.5",
        "version": 1,
        "metadata": {"component": {"name": "ai-lang", "version": "1.6.8", "type": "application"}},
        "components": components,
    }
    Path("sbom.json").write_text(json.dumps(bom, indent=2))
    print(f"Wrote sbom.json with {len(components)} components")

if __name__ == "__main__":
    main()
