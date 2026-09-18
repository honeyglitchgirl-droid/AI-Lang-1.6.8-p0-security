"""AI-Lang Python package.

When built with maturin (`maturin develop --features python`), the
native extension provides `run` and `compile`. This pure-Python module
is a fallback that raises if the extension is missing.
"""

try:
    from .ai_lang import run, compile  # type: ignore  # noqa: F401
except ImportError:
    def run(source: str) -> str:
        raise ImportError(
            "Native ai_lang extension not built. "
            "Run: maturin develop --features python"
        )

    def compile(source: str) -> bytes:
        raise ImportError(
            "Native ai_lang extension not built. "
            "Run: maturin develop --features python"
        )

__all__ = ["run", "compile"]
