# path: DataPipeline/utils/loggers.py
import logging
from colorlog import ColoredFormatter


def get_logger(name, level=logging.INFO):
    """Configure and return a logger with colored output."""
    formatter = ColoredFormatter(
        "%(log_color)s%(asctime)s - %(levelname)s - %(message)s",
        datefmt=None,
        reset=True,
        log_colors={
            "DEBUG": "cyan",
            "INFO": "green",
            "WARNING": "yellow",
            "ERROR": "red",
            "CRITICAL": "red,bg_white",
        },
        secondary_log_colors={},
        style="%",
    )

    handler = logging.StreamHandler()
    handler.setFormatter(formatter)

    logger = logging.getLogger(name)
    logger.addHandler(handler)
    logger.setLevel(level)
    return logger


def get_error_logger(name):
    return get_logger(name, level=logging.ERROR)


def get_warning_logger(name):
    return get_logger(name, level=logging.WARNING)


def get_info_logger(name):
    return get_logger(name, level=logging.INFO)


def get_debug_logger(name):
    return get_logger(name, level=logging.DEBUG)


def configure_logger(level=logging.INFO):
    """Configure the root logger with the specified logging level.

    Args:
        level: Logging level to set (default: logging.INFO)
    """
    # Configure the root logger
    formatter = ColoredFormatter(
        "%(log_color)s%(asctime)s - %(levelname)s - %(name)s - %(message)s",
        datefmt=None,
        reset=True,
        log_colors={
            "DEBUG": "cyan",
            "INFO": "green",
            "WARNING": "yellow",
            "ERROR": "red",
            "CRITICAL": "red,bg_white",
        },
        secondary_log_colors={},
        style="%",
    )

    # Configure root logger
    root_logger = logging.getLogger()

    # Remove existing handlers to avoid duplicates
    for handler in root_logger.handlers[:]:
        root_logger.removeHandler(handler)

    # Add a new handler
    handler = logging.StreamHandler()
    handler.setFormatter(formatter)
    root_logger.addHandler(handler)
    root_logger.setLevel(level)
