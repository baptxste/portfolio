"""
main.py — Automatic Image Generation Suite

Discovers and executes all Python image generator scripts located inside
the `images/` directory recursively.

Images are automatically saved to the target assets directory configured in `.env`:
    PATH_TO_VAULT (e.g., ../.vault/notes)
    PATH_IN_VAULT (e.g., assets)

Example:
    `images/maths/thm_centrale_limite.py` -> `<PATH_TO_VAULT>/<PATH_IN_VAULT>/maths/thm_centrale_limite.png`
"""

import argparse
import importlib.util
import inspect
import os
import sys
import traceback
from pathlib import Path
from dotenv import load_dotenv

# Base directory resolution
IMG_GENERATOR_DIR = Path(__file__).resolve().parent
PROJECT_ROOT = IMG_GENERATOR_DIR.parent

# Load environment variables from .env
env_file = IMG_GENERATOR_DIR / ".env"
if env_file.exists():
    load_dotenv(dotenv_path=env_file)
else:
    load_dotenv(PROJECT_ROOT / ".env")

# Resolve vault path and assets subdirectory from environment variables with fallback defaults
raw_vault_path = os.getenv("PATH_TO_VAULT", os.getenv("VAULT_PATH", "../.vault/notes"))
raw_path_in_vault = os.getenv("PATH_IN_VAULT", os.getenv("ASSETS_PATH_IN_VAULT", "assets"))

vault_path = Path(raw_vault_path)
if not vault_path.is_absolute():
    vault_path = (IMG_GENERATOR_DIR / vault_path).resolve()

DEFAULT_IMAGES_DIR = IMG_GENERATOR_DIR / "images"
DEFAULT_ASSETS_DIR = vault_path / raw_path_in_vault

# Ensure img_generator directory is in sys.path so scripts can import styles / style
if str(IMG_GENERATOR_DIR) not in sys.path:
    sys.path.insert(0, str(IMG_GENERATOR_DIR))


def find_image_scripts(images_dir: Path) -> list[Path]:
    """Recursively discover all python scripts in images_dir, ignoring hidden and utility files."""
    scripts = []
    for path in sorted(images_dir.rglob("*.py")):
        if path.name.startswith("_") or path.name.startswith(".") or path.name == "__init__.py":
            continue
        scripts.append(path)
    return scripts


def run_script(script_path: Path, images_dir: Path, assets_dir: Path) -> bool:
    """Import and execute a single image generator script, saving its image to assets_dir."""
    rel_path = script_path.relative_to(images_dir)

    target_img_path = assets_dir / rel_path.with_suffix(".png")
    target_img_path.parent.mkdir(parents=True, exist_ok=True)

    module_name = f"img_script_{rel_path.with_suffix('').as_posix().replace('/', '_')}"

    try:
        spec = importlib.util.spec_from_file_location(module_name, script_path)
        if spec is None or spec.loader is None:
            print(f"❌ [ERROR] Could not load spec for script: {rel_path}")
            return False

        module = importlib.util.module_from_spec(spec)
        sys.modules[module_name] = module
        spec.loader.exec_module(module)

        if hasattr(module, "OUTPUT_FILENAME") and module.OUTPUT_FILENAME:
            target_img_path = target_img_path.parent / module.OUTPUT_FILENAME

        entrypoint = None
        for fn_name in ["generate", "build", "main"]:
            if hasattr(module, fn_name) and callable(getattr(module, fn_name)):
                entrypoint = getattr(module, fn_name)
                break

        if entrypoint:
            sig = inspect.signature(entrypoint)
            if len(sig.parameters) > 0:
                result_path = entrypoint(target_img_path)
            else:
                result_path = entrypoint()

            output = result_path if result_path else target_img_path
        else:
            output = target_img_path

        output_path = Path(output)
        if output_path.exists():
            rel_output = (
                output_path.relative_to(PROJECT_ROOT)
                if output_path.is_relative_to(PROJECT_ROOT)
                else output_path
            )
            print(f"✅ [SUCCESS] {rel_path} -> {rel_output}")
            return True
        else:
            print(f"⚠️  [WARNING] {rel_path} finished, but output file not found at {target_img_path}")
            return False

    except Exception as e:
        print(f"❌ [FAILED] {rel_path}: {e}")
        traceback.print_exc()
        return False
    finally:
        if module_name in sys.modules:
            del sys.modules[module_name]


def main():
    parser = argparse.ArgumentParser(description="Generate image assets for vault notes.")
    parser.add_argument(
        "--images-dir",
        type=Path,
        default=DEFAULT_IMAGES_DIR,
        help="Path to images scripts directory",
    )
    parser.add_argument(
        "--assets-dir",
        type=Path,
        default=DEFAULT_ASSETS_DIR,
        help=f"Target directory for output images (default resolved from .env: {DEFAULT_ASSETS_DIR})",
    )
    parser.add_argument(
        "--filter",
        type=str,
        default=None,
        help="Filter scripts by name or subfolder",
    )

    args = parser.parse_args()

    images_dir = args.images_dir.resolve()
    assets_dir = args.assets_dir.resolve()

    if not images_dir.exists():
        print(f"Error: Images directory not found: {images_dir}")
        sys.exit(1)

    assets_dir.mkdir(parents=True, exist_ok=True)

    scripts = find_image_scripts(images_dir)
    if args.filter:
        scripts = [s for s in scripts if args.filter.lower() in str(s).lower()]

    if not scripts:
        print(f"No image scripts found in {images_dir}")
        return

    print("==================================================")
    print("🎨 Running Image Generator Suite")
    print(f"📂 Source: {images_dir}")
    print(f"🎯 Vault Assets Target: {assets_dir}")
    print(f"📄 Found {len(scripts)} script(s)")
    print("==================================================")

    success_count = 0
    fail_count = 0

    for script in scripts:
        if run_script(script, images_dir, assets_dir):
            success_count += 1
        else:
            fail_count += 1

    print("--------------------------------------------------")
    print(f"📊 Summary: {success_count} succeeded, {fail_count} failed out of {len(scripts)} script(s).")
    print("==================================================")

    if fail_count > 0:
        sys.exit(1)


if __name__ == "__main__":
    main()
